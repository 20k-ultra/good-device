pub fn default() -> Option<pnet_datalink::NetworkInterface> {
    // Get a vector with all network interfaces found
    let all_interfaces = pnet_datalink::interfaces();

    // Search for the default interface - the one that is
    // up, not loopback and has an IP.
    let default_interface = all_interfaces
        .into_iter()
        .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

    match default_interface {
        Some(interface) => Some(interface),
        None => None,
    }
}
