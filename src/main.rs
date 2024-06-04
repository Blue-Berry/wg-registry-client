use std::{net::UdpSocket, str::FromStr};

use defguard_wireguard_rs::key::Key;

mod structure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let header = structure::Header {
        msg_id: 0,
        qr: structure::QUERY,
        opcode: structure::Opcode::EndpointQuery,
        rcode: structure::ResponseCode::NoError,
    };

    let header = header.to_bytes();
    let key: Key = Key::from_str("sMSXGksm27bWGi0wZ04c3APZpDsmN0l0wwkpPq0DPjY=")?;
    let mut buffer = [0u8; 36];
    buffer[..4].copy_from_slice(&header);
    buffer[4..].copy_from_slice(key.as_slice());

    let udp_socket = UdpSocket::bind("127.0.0.1:2054").expect("Failed to bind to address");
    udp_socket
        .send_to(&buffer, "127.0.0.1:2043")
        .expect("Failed to send data");
    let mut buffer = [0u8; 512];
    let (amt, src) = udp_socket
        .recv_from(&mut buffer)
        .expect("Failed to receive data");
    println!("Received data from: {}", src);
    println!("Received data: {:?}", &buffer[..amt]);
    Ok(())
}
