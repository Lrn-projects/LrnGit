use std::{
    env::{self, set_current_dir},
    io::{self, Read},
    os::fd::FromRawFd,
    path::Path,
    process::exit,
};

use std::net::{Shutdown, TcpStream};

use lrngitcore::{
    fs::pack::write_pack_to_disk, out::write_framed_message_stdout, pack::{refs::{parse_refs_pack, ParsedRefsPack}, upload::parse_upload_pack},
};

mod head;

fn main() {
    let mut stdout = io::stdout();
    let args: Vec<String> = env::args().collect();
    let lrngit_repo_path: &str = "/home/ubuntu/lrngit/repositories/";
    if args.len() < 2 {
        write_framed_message_stdout("ERR repository name argument missing", &mut stdout);
        // Create stream from fd and shutdown to properly send err to client
        let _ = unsafe { TcpStream::from_raw_fd(1) }.shutdown(Shutdown::Write);
        exit(1);
    }
    let repo_path = lrngit_repo_path.to_owned() + &args[1];
    if !Path::new(&repo_path).exists() {
        write_framed_message_stdout("ERR repository doesn't exist", &mut stdout);
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
    // Loop over standard input for incoming packets
    let refs: ParsedRefsPack;
    loop {
        let mut stream_length = [0u8; 4];
        // Read buffer length
        if let Err(e) = io::stdin().read_exact(&mut stream_length) {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                write_framed_message_stdout("TCP connection closed", &mut stdout);
                break;
            } else {
                eprintln!("Failed to read stream length: {e}");
                break;
            }
        }
        let length = u32::from_le_bytes(stream_length);
        if length == 0 {
            write_framed_message_stdout("Received zero-length packet, closing connection", &mut stdout);
            let _ = unsafe { TcpStream::from_raw_fd(1) }.shutdown(Shutdown::Write);
            exit(1);
        }
        // Read rest of the stream in buffer
        io::stdin()
            .read_exact(&mut buffer[..length as usize])
            .expect("Failed to read framed stream");
        if buffer.is_empty() {
            write_framed_message_stdout("TCP connection closed", &mut stdout);
            break;
        }
        // Check first 4 bytes to know which packets
        let magic_number: &str =
            str::from_utf8(&buffer[..4]).expect("Failed to cast first 4 buffer's bytes into str");
        // Switch on magic number to handle packet correctly
        match magic_number {
            "REFS" => {
                // Drain 4 first bytes + \0
                buffer.drain(..5);
                refs = parse_refs_pack(&buffer[..length as usize]);
                // Check if refs exist on remote repository
                if !Path::exists(Path::new(refs.refs)) {
                    write_framed_message_stdout("ERR reference doesn't exist on remote host", &mut stdout);
                    break;
                }
            }
            "PACK" => {
                // Drain 4 first bytes + \0
                buffer.drain(..5);
                let pack = match parse_upload_pack(&buffer) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Failed to parse upload pack: {e}");
                        break;
                    }
                };
                write_framed_message_stdout("received upload pack", &mut stdout);
                // Write pack content to disk
                write_pack_to_disk(pack.data);
                // Update head using refs and pack
                update_refs(refs, pack);
                write_framed_message_stdout("ACK", &mut stdout);
            }
            _ => {
                write_framed_message_stdout("ACK", &mut stdout);
            }
        }
    }
}
