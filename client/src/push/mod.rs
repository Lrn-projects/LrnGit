use std::{
    env,
    io::{self, Read, Write},
    net::TcpStream,
    process::exit,
};

use lrngitcore::remote::origin::parse_origin_branch;

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
    let last_remote_commit = parse_origin_branch();
    let refs = &parse_head();
    let mut stream = tcp::tcp_connect_to_remote("lrngit-receive-pack");
    // Reference to last local commit and last remote commit pack
    let mut ref_buff: Vec<u8> = Vec::new();
    ref_buff.extend_from_slice(&b"REFS ".to_vec());
    ref_buff.extend_from_slice(&refs.as_bytes());
    ref_buff.extend_from_slice(&last_commit.as_bytes());
    ref_buff.extend_from_slice(&last_remote_commit.as_bytes());
    let ref_buff_len: u32 = ref_buff.len() as u32;
    let mut ref_pack: Vec<u8> = Vec::new();
    ref_pack.extend_from_slice(&ref_buff_len.to_le_bytes());
    ref_pack.extend_from_slice(&ref_buff);
    ref_pack.extend_from_slice(last_remote_commit.as_bytes());
    // Pack object
    let pack = create_upload_pack();
    let mut upload_pack: Vec<u8> = Vec::new();
    upload_pack.extend_from_slice(&b"PACK ".to_vec());
    upload_pack.extend_from_slice(&pack);
    let pack_length: u32 = upload_pack.len() as u32;
    let mut stream_framed: Vec<u8> = Vec::new();
    stream_framed.extend_from_slice(&pack_length.to_le_bytes());
    stream_framed.extend_from_slice(&upload_pack);
    // ---- Stream packet to remote host ----
    // Reference packet
    stream
        .write_all(&ref_pack)
        .expect("Failed to stream references to remote host");
    stream.flush().expect("Failed to flush references stream");
    // Upload pack
    stream
        .write_all(&stream_framed)
        .expect("Failed to stream upload pack to remote host");
    stream.flush().expect("Failed to flush upload pack stream");
    handle_server(stream);
}

/// Handle connection with remote host and read incoming stream
fn handle_server(mut stream: TcpStream) {
    let mut buffer = vec![0u8; 1024];
    // Loop over the stream to read all incoming packets
    loop {
        let mut stream_length = [0u8; 4];
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
        let received: &str =
            str::from_utf8(&buffer[..length as usize]).expect("Failed to cast buffer to str");
        if received == "ACK" {
            break;
        }
        println!("remote: {}", received);
    }
    stream
        .shutdown(std::net::Shutdown::Write)
        .expect("Failed to shutdown stream");
}
