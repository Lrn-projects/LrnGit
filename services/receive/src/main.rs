use std::{
    env::{self, set_current_dir},
    io::{self, Read, Write},
    os::fd::FromRawFd,
    path::Path,
    process::exit,
};

use std::net::{Shutdown, TcpStream};

use lrngitcore::{fs::pack::write_pack_to_disk, pack::upload::parse_upload_pack};

fn main() {
    let mut stdout = io::stdout();
    {
        let message: &str = "[SERVICE] lrngit-receive";
        let length: u32 = message.len() as u32;
        stdout
            .write_all(&length.to_le_bytes())
            .expect("Failed to write length in stdout");
        stdout
            .write_all(&message.as_bytes())
            .expect("Failed to write message in stdout");
    }
    stdout.flush().unwrap();
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
        let message: &str = "ERR repository doesn't exist";
        let length: u32 = message.len() as u32;
        println!("{}", format!("{:?} {}", length.to_le_bytes(), message));
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
        // Read buffer length
        if let Err(e) = io::stdin().read_exact(&mut stream_length) {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                let message: &str = "TCP connection closed";
                let length: u32 = message.len() as u32;
                println!("{}", format!("{:?} {}", length.to_le_bytes(), message));
                io::stdout().flush().unwrap();
                break;
            } else {
                eprintln!("Failed to read stream length: {e}");
                break;
            }
        }
        let length = u32::from_le_bytes(stream_length);
        if length == 0 {
            let message: &str = "Received zero-length packet, closing connection.";
            let length: u32 = message.len() as u32;
            println!("{}", format!("{:?} {}", length.to_le_bytes(), message));
            io::stdout().flush().unwrap();
            break;
        }
        // Read rest of the stream in buffer
        io::stdin()
            .read_exact(&mut buffer[..length as usize])
            .expect("Failed to read framed stream");
        if buffer.is_empty() {
            let message: &str = "TCP connection closed";
            let length: u32 = message.len() as u32;
            println!("{}", format!("{:?} {}", length.to_le_bytes(), message));
            io::stdout().flush().unwrap();
            break;
        }
        let pack = match parse_upload_pack(&buffer) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to parse upload pack: {e}");
                break;
            }
        };
        let message: &str = "Received upload pack";
        let length: u32 = message.len() as u32;
        println!("{}", format!("{:?} {}", length.to_le_bytes(), message));
        write_pack_to_disk(pack.data);
        io::stdout().flush().unwrap();
    }
}
