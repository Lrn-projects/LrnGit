use std::{
    env::{self, set_current_dir},
    io::{self, stdout, Read, Write},
    path::Path,
    process::exit,
};

fn main() {
    println!("[SERVICE] lrngit-receive");
    let args: Vec<String> = env::args().collect();
    let lrngit_repo_path: &str = "/home/ubuntu/lrngit/repositories/";
    if args.len() < 2 {
        eprintln!("ERR: repository name argument missing");
        exit(1);
    }
    let repo_path = lrngit_repo_path.to_owned() + &args[1];
    if !Path::new(&repo_path).exists() {
        eprintln!("ERR repository doesn't exist");
        exit(1)
    }
    set_current_dir(repo_path).expect("Failed to change current dir");
    let mut buffer = [0u8; 1024];
    loop {
        let n = io::stdin().read(&mut buffer).expect("read failed");

        if n == 0 {
            eprintln!("TCP connection closed");
            io::stderr().flush().unwrap();
            break;
        }

        println!("Packet: {:?}", String::from_utf8_lossy(&buffer[..n]));
        stdout().flush().unwrap();
    }
}
