use std::{env, io::{Read, Write}, process::exit};

use crate::tcp;

pub fn pull_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        pull_remote_branch();
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

/// Pull change from remote repository on current origin
fn pull_remote_branch() {
    let mut stream = tcp::tcp_connect_to_remote("lrngit-upload-pack");
    stream.flush().expect("Failed to flush stream");
    let mut buffer = [0u8; 1024];
    loop {
        let n = stream.read(&mut buffer).unwrap();
        if n == 0 {
            println!("Connection closed.");
            break;
        }
        println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    }
}
