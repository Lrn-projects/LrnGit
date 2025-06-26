use std::io::Read;
use std::net::TcpStream;

use crate::process;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 512] = [0; 512];
    match stream.read(&mut buffer) {
        Ok(0) => panic!("connection closed"), // Connection was closed
        Ok(n) => {
            let received = String::from_utf8_lossy(&buffer[..n]);
            println!("Packet: {received:#?}");
            let mut service: Vec<u8> = Vec::new();
            let mut path: Vec<u8> = Vec::new();
            let mut temp: bool = false;
            for &b in buffer.iter() {
                if b == b' ' {
                    temp = true;
                }
                if !temp {
                    service.push(b);
                } else {
                    if b == b'\0' {
                        break;
                    }
                    path.push(b);
                }
            }
            let service_str: &str = str::from_utf8(&service).unwrap();
            let path_str: &str = str::from_utf8(&path).unwrap().trim();
            match service_str {
                "lrngit-receive-pack" => {
                    process::fork_service("lrngit-receive-service", path_str, stream);
                }
                "lrngit-upload-pack" => {}
                _ => {}
            }
        }
        Err(e) => {
            eprintln!("Failed to read from socket: {e}");
        }
    }
}
