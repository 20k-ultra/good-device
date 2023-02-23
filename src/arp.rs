extern crate pnet;

use std::net::Ipv4Addr;

use pnet::datalink::{Channel, MacAddr, NetworkInterface};
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::{MutablePacket, Packet};

use crate::ipv4;

fn create_arp_request_packet(
    buf: &mut [u8],
    source_ip: Ipv4Addr,
    source_mac: [u8; 6],
) -> MutableArpPacket {
    let mut packet = MutableArpPacket::new(buf).unwrap();
    packet.set_hardware_type(ArpHardwareTypes::new(1));
    packet.set_protocol_type(0x0800);
    packet.set_hw_addr_len(6);
    packet.set_proto_addr_len(4);
    packet.set_operation(1);
    packet.set_sender_hw_addr(source_mac);
    packet.set_sender_proto_addr(source_ip.into());
    packet.set_target_hw_addr([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    packet.set_target_proto_addr(Ipv4Addr::new(0, 0, 0, 0).into());
    packet
}

pub fn get_mac(interface: NetworkInterface, target_ip: Ipv4Addr) -> Option<MacAddr> {
    // Get an Ipv4Addr for this interface
    let (source_ip, _) =
        ipv4::default(&interface).expect("Failed to find IPv4 network on provided interface");

    let (mut sender, mut receiver) = match pnet::datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    let mut ethernet_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination(MacAddr::broadcast());
    ethernet_packet.set_source(interface.mac.unwrap());
    ethernet_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Request);
    arp_packet.set_sender_hw_addr(interface.mac.unwrap());
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(MacAddr::zero());
    arp_packet.set_target_proto_addr(target_ip);

    ethernet_packet.set_payload(arp_packet.packet_mut());

    sender
        .send_to(ethernet_packet.packet(), None)
        .unwrap()
        .unwrap();

    let buf = receiver.next().unwrap();

    // TODO this is always returning the same mac address..
    match ArpPacket::new(&buf[MutableEthernetPacket::minimum_packet_size()..]) {
        Some(reply) => Some(reply.get_sender_hw_addr()),
        None => None,
    }
}
