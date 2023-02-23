use std::io;
use std::net::Ipv4Addr;
use std::time::Duration;

use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::packet::MutablePacket;
use pnet::transport::{icmp_packet_iter, transport_channel, TransportChannelType::Layer3};
use pnet::util;

static IPV4_HEADER_LEN: usize = 21;
static ICMP_HEADER_LEN: usize = 8;
static ICMP_PAYLOAD_LEN: usize = 32;

fn create_icmp_packet<'a>(
    buf_ip: &'a mut [u8],
    buf_icmp: &'a mut [u8],
    dest: Ipv4Addr,
    ttl: u8,
    sequence_number: u16,
) -> MutableIpv4Packet<'a> {
    let mut ipv4_packet = MutableIpv4Packet::new(buf_ip).expect("Error creating ipv4 packet");
    ipv4_packet.set_version(4);
    ipv4_packet.set_header_length(IPV4_HEADER_LEN as u8);
    ipv4_packet.set_total_length((IPV4_HEADER_LEN + ICMP_HEADER_LEN + ICMP_PAYLOAD_LEN) as u16);
    ipv4_packet.set_ttl(ttl);
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
    ipv4_packet.set_destination(dest);

    let mut icmp_packet =
        MutableEchoRequestPacket::new(buf_icmp).expect("Error creating icmp packet");
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_sequence_number(sequence_number);
    let checksum = util::checksum(&icmp_packet.packet_mut(), 1);
    icmp_packet.set_checksum(checksum);
    ipv4_packet.set_payload(icmp_packet.packet_mut());

    ipv4_packet
}

pub fn ping(dest: Ipv4Addr) -> Result<Ipv4Addr, io::Error> {
    let timeout = Duration::new(2, 0);
    let protocol = Layer3(IpNextHeaderProtocols::Icmp);
    let (mut tx, mut rx) = transport_channel(2 << 15, protocol)
        .map_err(|err| format!("Error opening the channel: {}", err))
        .expect("Transport channel error");

    let mut rx = icmp_packet_iter(&mut rx);
    let ttl = 60;
    let mut buf_ip = [0u8; 40];
    let mut buf_icmp = [0u8; 40];

    let icmp_packet = create_icmp_packet(&mut buf_ip, &mut buf_icmp, dest, ttl, 1);

    tx.send_to(icmp_packet, std::net::IpAddr::V4(dest))
        .expect(&format!("Failed to send ICMP packet to {}", dest));

    loop {
        match rx.next_with_timeout(timeout) {
            Ok(opt) => match opt {
                Some((_, addr)) => {
                    if addr.eq(&dest) {
                        return Ok(dest);
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::NotConnected,
                            "Device not reachable",
                        ));
                    }
                }
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::NotConnected,
                        "Device not reachable",
                    ))
                }
            },
            Err(error) => return Err(error),
        }
    }
}
