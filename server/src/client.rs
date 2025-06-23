use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection was closed
            Ok(n) => {
                let received = String::from_utf8_lossy(&buffer[..n]);
                println!("Packet: {received:#?}");
                // Send response to client when packet received
                if let Err(e) = stream.write_all("Packet received".as_bytes()) {
                    eprintln!("Failed to send response: {e}");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {e}");
                break;
            }
        }
    }
}
