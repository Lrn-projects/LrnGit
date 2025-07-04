use std::{
    env::{self, set_current_dir},
    io::{self, Read, Write},
    os::fd::FromRawFd,
    path::Path,
    process::exit,
};

use std::net::{Shutdown, TcpStream};

use lrngitcore::pack::upload::{UploadPack, UploadPackData, parse_upload_pack};

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
    // buffer of 64kb size
    let mut buffer = vec![0u8; 65536];
    loop {
        if let Err(e) = io::stdin().read_exact(&mut stream_length) {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                println!("TCP connection closed");
                io::stdout().flush().unwrap();
                break;
            } else {
                eprintln!("Failed to read stream length: {e}");
                io::stdout().flush().unwrap();
                break;
            }
        }
        let length = u32::from_le_bytes(stream_length);
        if length == 0 {
            println!("Received zero-length packet, closing connection.");
            io::stdout().flush().unwrap();
            break;
        }
        io::stdin()
            .read_exact(&mut buffer[..length as usize])
            .expect("Failed to read framed stream");
        if buffer.is_empty() {
            println!("TCP connection closed");
            io::stdout().flush().unwrap();
            break;
        }
        let pack = match parse_upload_pack(&buffer) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to parse upload pack: {e}");
                io::stdout().flush().unwrap();
                break;
            }
        };
        println!("debug pack: {:?}", pack);
        println!("Received upload pack");
        io::stdout().flush().unwrap();
    }
}
