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
    let mut buffer = [0u8; 1024];
    loop {
        let n = io::stdin().read(&mut buffer).expect("read failed");
        if n == 0 {
            println!("TCP connection closed");
            io::stdout().flush().unwrap();
            break;
        }
        let mut clean_buff: Vec<u8> = Vec::new();
        for each in buffer {
            if each != 0 {
                clean_buff.push(each);
            } else {
                break;
            } 
        }
        let pack: UploadPack = parse_upload_pack(&clean_buff).expect("Failed to parse upload pack");
        println!("debug pack: {pack:?}");
        io::stdout().flush().unwrap();
    }
}
