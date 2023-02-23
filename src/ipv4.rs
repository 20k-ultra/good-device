use std::net::{IpAddr, Ipv4Addr};
use std::ops::Range;

use pnet::datalink::NetworkInterface;

fn to_u32(ipv4: Ipv4Addr) -> u32 {
    let octets: [u8; 4] = ipv4.octets().try_into().unwrap();
    u32::from_be_bytes(octets)
}

pub fn default(interface: &NetworkInterface) -> Option<(Ipv4Addr, u8)> {
    // Filter the interface for an IPv4 network
    let ipv4_network = interface
        .clone()
        .ips
        .into_iter()
        .find(|ip| ip.is_ipv4())
        .unwrap();

    // Parse the subnet mask and IP of the network
    match ipv4_network.ip() {
        IpAddr::V4(ipv4) => Some((ipv4, ipv4_network.prefix())),
        IpAddr::V6(_) => None,
    }
}

pub fn subnet_range(ip_in_subnet: Ipv4Addr, subnet_mask: u8) -> Range<Ipv4Addr> {
    // Calculate the network part of the address using the prefix
    let start_ip = to_u32(ip_in_subnet) & (0xFFFFFFFF << (32 - subnet_mask));

    // Calculate the last IP address in the range
    let end_ip: u32 = start_ip | (0xFFFF_FFFF >> subnet_mask);

    // Convert the starting and ending IP addresses back to Ipv4Addr values
    Range {
        start: Ipv4Addr::from(start_ip),
        end: Ipv4Addr::from(end_ip),
    }
}

pub fn is_broadcast_ip(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();
    octets[3] == 255
}
