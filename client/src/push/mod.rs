use std::{
    env,
    io::{self, Read, Write},
    process::exit,
};

use crate::{
    pack::upload::create_upload_pack,
    refs::{parse_current_branch, parse_head},
    tcp,
};

pub fn push_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        push_remote_branch();
        exit(0);
    }
    if args.len() == 3 {
        exit(0);
    }
    match args[2].as_str() {
        "" => {}
        _ => {
            lrncore::logs::warning_log("Unknown command");
            exit(1);
        }
    }
}

/// Push the local change to the remote repository. Get the last commit and refs, enable connection
/// between client and remote host and send object through an upload pack.
fn push_remote_branch() {
    let last_commit = parse_current_branch();
    let refs = &parse_head();
    let mut stream = tcp::tcp_connect_to_remote("lrngit-receive-pack");
    let pack = create_upload_pack(refs, last_commit.as_bytes().to_vec());
    let pack_length: u32 = pack.len() as u32;
    let mut stream_framed: Vec<u8> = Vec::new();
    stream_framed.extend_from_slice(&pack_length.to_le_bytes());
    stream_framed.extend_from_slice(&pack);
    stream
        .write_all(&stream_framed)
        .expect("Failed to stream upload pack");
    stream.flush().expect("Failed to flush stream");
    // Loop over the stream to read all incoming packets
    let mut stream_length = [0u8; 4];
    let mut buffer = vec![0u8; 1024];
    loop {
        // Read buffer length
        if let Err(e) = stream.read_exact(&mut stream_length) {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                println!("TCP connection closed");
                break;
            } else {
                eprintln!("Failed to read stream length: {e}");
                break;
            }
        }
        let length = u32::from_le_bytes(stream_length);
        if length == 0 {
            println!("Received zero-length packet, closing connection.");
            break;
        }
        // Read rest of the stream in buffer
        stream
            .read_exact(&mut buffer[..length as usize])
            .expect("Failed to read framed stream");
        if buffer.is_empty() {
            println!("TCP connection closed");
            break;
        }
        // println!("Enumerate objects: {:?}", pack.len());
        println!(
            "Received: {}",
            String::from_utf8_lossy(&buffer[..length as usize])
        );
    }
    stream
        .shutdown(std::net::Shutdown::Write)
        .expect("Failed to shutdown stream");
}
