use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection was closed
            Ok(n) => {
                print!("Packet received: {:?}", &buffer[..n]);
                // Echo everything received back to the client
                if let Err(e) = stream.write_all(&buffer[..n]) {
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
