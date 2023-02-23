use std::net::Ipv4Addr;

use pnet::datalink::NetworkInterface;
use rayon::prelude::*;

use crate::icmp;
use crate::ipv4;

pub struct Device {
    pub ip: Ipv4Addr,
}

pub fn devices(interface: NetworkInterface) -> Vec<Device> {
    // Get an Ipv4Addr and subnet mask for this interface
    let (my_ipv4_addr, subnet_mask) =
        ipv4::default(&interface).expect("Failed to find IPv4 network on provided interface");

    // Get IP ranges (start, end) from our target network
    let ip_range = ipv4::subnet_range(my_ipv4_addr, subnet_mask);

    // Iterate over the IP addresses in the specified range and parallelize the loop using rayon
    let start: u32 = ip_range.start.into();
    let end: u32 = ip_range.end.into();
    let devices: Vec<Device> = (start..=end)
        .into_par_iter()
        .filter_map(|ip_from_subnet| {
            let dest_ipv4_addr: Ipv4Addr = ip_from_subnet.into();
            // Skip our own IP and broadcast IP
            if dest_ipv4_addr.eq(&my_ipv4_addr) || ipv4::is_broadcast_ip(dest_ipv4_addr) {
                println!("Skipping ICMP ping for IP: {}", dest_ipv4_addr);
                None
            } else {
                match icmp::ping(dest_ipv4_addr.into()) {
                    Ok(ip) => Some(Device { ip }),
                    Err(_) => None,
                }
            }
        })
        .collect();

    devices
}
