use std::net::TcpStream;
use std::io::Read;
use crate::domain::infra::ssrf_engine::process_network_request;

// Camouflaged function to receive a URL via TCP stream (CWE-918)
pub fn receive_url_from_tcp() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("failed to connect TCP stream");
    let mut buf = [0u8; 1024];
    //SOURCE
    let _ = stream.read(&mut buf).expect("read failed");
    let tainted_url = String::from_utf8_lossy(&buf).trim_matches(char::from(0)).to_string();
    process_network_request(tainted_url);
} 