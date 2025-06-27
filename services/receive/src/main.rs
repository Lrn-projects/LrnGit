use std::{
    env::{self, set_current_dir},
    io::{self, Read, Write},
    path::Path,
    process::exit, thread::sleep, time::Duration,
};

fn main() {
    println!("[SERVICE] lrngit-receive");
    io::stdout().flush().unwrap();
    let args: Vec<String> = env::args().collect();
    let lrngit_repo_path: &str = "/home/ubuntu/lrngit/repositories/";
    if args.len() < 2 {
        println!("ERR: repository name argument missing");
        io::stdout().flush().unwrap();
        sleep(Duration::new(1, 0));
        exit(1);
    }
    let repo_path = lrngit_repo_path.to_owned() + &args[1];
    if !Path::new(&repo_path).exists() {
        println!("ERR repository doesn't exist");
        io::stdout().flush().unwrap();
        sleep(Duration::new(1, 0));
        exit(1)
    }
    set_current_dir(repo_path).expect("Failed to change current dir");
    let mut buffer = [0u8; 1024];
    loop {
        let n = io::stdin().read(&mut buffer).expect("read failed");
        if n == 0 {
            println!("TCP connection closed");
            io::stdout().flush().unwrap();
            break;
        }

        println!("Packet: {:?}", String::from_utf8_lossy(&buffer[..n]));
        io::stdout().flush().unwrap();
    }
}
