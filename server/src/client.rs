use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection was closed
            Ok(_n) => {
                let stream_reader = BufReader::new(&stream);
                let packet: Vec<_> = stream_reader
                    .lines()
                    .map(|result| result.unwrap())
                    .take_while(|line| !line.is_empty())
                    .collect();

                println!("Packet: {packet:#?}");
                // Echo everything received back to the client
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
