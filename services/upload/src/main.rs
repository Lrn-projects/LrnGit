use std::{env::{self, set_current_dir}, io::{self, Write}, net::{Shutdown, TcpStream}, os::fd::FromRawFd, path::Path, process::exit};

fn main() {
    println!("[SERVICE] lrngit-upload");
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
}
