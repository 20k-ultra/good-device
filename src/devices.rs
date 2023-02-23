use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

fn is_google_home_device(ip: &str) -> bool {
    // Convert the IP address string to a socket address
    let addr = (ip, 8008).to_socket_addrs().unwrap().next().unwrap();

    // Connect to the device using a TcpStream
    let mut stream = TcpStream::connect(addr).unwrap();

    // Send a probe request to the device
    let request = b"M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:8008\r\nMAN: \"ssdp:discover\"\r\nMX: 1\r\nST: urn:dial-multiscreen-org:service:dial:1\r\n\r\n";
    stream.write_all(request).unwrap();

    // Read the response from the device
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    // Check if the device is a Google Home device based on the response
    response.contains("Google Home")
}
