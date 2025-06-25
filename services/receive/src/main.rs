use std::{
    env::{self, set_current_dir},
    io::{BufRead, Read, stdin},
    net::TcpStream,
    os::fd::FromRawFd,
    path::Path,
    process::exit,
};

fn main() {
    println!("[SERVICE] lrngit-receive");
    let args: Vec<String> = env::args().collect();
    let stdin = stdin();
    let lrngit_repo_path: &str = "/home/ubuntu/lrngit/repositories/";
    let mut lines: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        let line_str = line.unwrap();
        lines.push(line_str);
    }
    let mut stream = unsafe {
        TcpStream::from_raw_fd(lines[0].parse::<i32>().expect("Failed to cast str to i32"))
    };
    let mut buffer: [u8; 512] = [0; 512];
    match stream.read(&mut buffer) {
        Ok(n) => {
                println!(
                    "debug buffer from stream: {:?}",
                    str::from_utf8(&buffer[..n])
                );
            }
        Err(_) => panic!("Failed to read from stream"),
    }
    let repo_path = lrngit_repo_path.to_owned() + &args[1];
    if !Path::new(&repo_path).exists() {
        eprintln!("ERR repository doesn't exist");
        exit(1)
    }
    set_current_dir(repo_path).expect("Failed to change current dir");
}
