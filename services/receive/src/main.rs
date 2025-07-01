use std::{
    env::{self, set_current_dir},
    io::{self, Read, Write},
    os::fd::FromRawFd,
    path::Path,
    process::exit,
};

use std::net::{Shutdown, TcpStream};

use lrngitcore::pack::upload::{parse_upload_pack, UploadPack};

fn main() {
    println!("[SERVICE] lrngit-receive");
    io::stdout().flush().unwrap();
    let args: Vec<String> = env::args().collect();
    let lrngit_repo_path: &str = "/home/ubuntu/lrngit/repositories/";
    if args.len() < 2 {
        println!("ERR: repository name argument missing");
        io::stdout().flush().unwrap();
        // Create stream from fd and shutdown to properly send err to client
        let _ = unsafe { TcpStream::from_raw_fd(1) }.shutdown(Shutdown::Write);
        exit(1);
    }
    let repo_path = lrngit_repo_path.to_owned() + &args[1];
    if !Path::new(&repo_path).exists() {
        println!("ERR repository doesn't exist");
        io::stdout().flush().unwrap();
        // Create stream from fd and shutdown to properly send err to client
        let _ = unsafe { TcpStream::from_raw_fd(1) }.shutdown(Shutdown::Write);
        exit(1)
    }
    set_current_dir(repo_path).expect("Failed to change current dir");
    // Loop over stdin for incoming packets
    let mut stream_length = [0u8; 4];
    loop {
        if let Err(e) = io::stdin().read_exact(&mut stream_length) {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                println!("TCP connection closed");
                io::stdout().flush().unwrap();
                break;
            } else {
                panic!("Failed to read stream length: {e}");
            }
        }
        let length = u32::from_le_bytes(stream_length);
        let mut buffer = vec![0u8; length as usize];
        io::stdin()
            .read_exact(&mut buffer)
            .expect("Failed to read framed stream");
        println!("debug buff: {buffer:?}");
        let pack: UploadPack = parse_upload_pack(&buffer).expect("Failed to parse upload pack");
        println!("debug pack: {pack:?}");
        io::stdout().flush().unwrap();
    }
}
