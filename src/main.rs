use std::net::Ipv4Addr;

// mod arp;
// mod icmp;
mod interface;
// mod ipv4;
// mod network;
mod tcp;

fn main() {
    // Get an interface to use for searching the network
    let interface = interface::default().expect("Failed to find an interface to scan with");

    println!(
        "10.0.0.156 is responsive -> {}",
        tcp::syn_ping(interface, Ipv4Addr::new(10, 0, 0, 156)).unwrap()
    );

    // Search the network for devices
    // let devices = network::devices(interface);
    // Log devices found
    // for device in devices {
    //     println!("Found device at IP: {}", device.ip)
    // }
}
