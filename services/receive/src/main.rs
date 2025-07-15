use std::{
    env::{self, set_current_dir},
    io::{self, Read},
    os::fd::FromRawFd,
    path::Path,
    process::exit,
};

use std::net::{Shutdown, TcpStream};

use lrngitcore::{
    fs::pack::write_pack_to_disk, out::write_framed_message_stdout, pack::upload::parse_upload_pack,
};

fn main() {
    let mut stdout = io::stdout();
    let args: Vec<String> = env::args().collect();
    let lrngit_repo_path: &str = "/home/ubuntu/lrngit/repositories/";
    if args.len() < 2 {
        let message: &str = "ERR repository name argument missing";
        write_framed_message_stdout(message.len() as u32, message, &mut stdout);
        // Create stream from fd and shutdown to properly send err to client
        let _ = unsafe { TcpStream::from_raw_fd(1) }.shutdown(Shutdown::Write);
        exit(1);
    }
    let repo_path = lrngit_repo_path.to_owned() + &args[1];
    if !Path::new(&repo_path).exists() {
        let message: &str = "ERR repository doesn't exist";
        write_framed_message_stdout(message.len() as u32, message, &mut stdout);
        // Create stream from fd and shutdown to properly send err to client
        let _ = unsafe { TcpStream::from_raw_fd(1) }.shutdown(Shutdown::Write);
        exit(1)
    }
    set_current_dir(repo_path).expect("Failed to change current dir");
    handle_stream(stdout);
    // Close properly stream when handling stream returned
    let _ = unsafe { TcpStream::from_raw_fd(1) }.shutdown(Shutdown::Write);
    exit(1)
}

fn handle_stream(mut stdout: io::Stdout) {
    // buffer of 64kb size
    let mut buffer = vec![0u8; 65536];
    // Loop over stdin for incoming packets
    loop {
        let mut stream_length = [0u8; 4];
        // Read buffer length
        if let Err(e) = io::stdin().read_exact(&mut stream_length) {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                let message: &str = "TCP connection closed";
                write_framed_message_stdout(message.len() as u32, message, &mut stdout);
                break;
            } else {
                eprintln!("Failed to read stream length: {e}");
                break;
            }
        }
        let length = u32::from_le_bytes(stream_length);
        if length == 0 {
            let message: &str = "Received zero-length packet, closing connection.";
            write_framed_message_stdout(message.len() as u32, message, &mut stdout);
            let _ = unsafe { TcpStream::from_raw_fd(1) }.shutdown(Shutdown::Write);
            exit(1);
        }
        // Read rest of the stream in buffer
        io::stdin()
            .read_exact(&mut buffer[..length as usize])
            .expect("Failed to read framed stream");
        if buffer.is_empty() {
            let message: &str = "TCP connection closed";
            write_framed_message_stdout(message.len() as u32, message, &mut stdout);
            break;
        }
        // Check first 4 bytes to know which packets
        let magic_number: &str =
            str::from_utf8(&buffer[..4]).expect("Failed to cast first 4 buffer's bytes into str");
        let magic_string: &str = &format!("magic number: {:?}", magic_number);
        write_framed_message_stdout(magic_string.len() as u32, magic_string, &mut stdout);
        match magic_number {
            "REFS" => println!("received refs"),
            "PACK" => {
                let pack = match parse_upload_pack(&buffer) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Failed to parse upload pack: {e}");
                        break;
                    }
                };
                let mut message: &str = "received upload pack";
                write_framed_message_stdout(message.len() as u32, message, &mut stdout);
                message = "ACK";
                write_framed_message_stdout(message.len() as u32, message, &mut stdout);
                write_pack_to_disk(pack.data);
            },
            _ => ()
        }
    }
}
