use std::{
    env::{self, set_current_dir},
    io::Read,
    path::Path,
    process::exit,
};

fn main() {
    println!("[SERVICE] lrngit-receive");
    let args: Vec<String> = env::args().collect();
    let lrngit_repo_path: &str = "/home/ubuntu/lrngit/repositories/";
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    println!("debug stdin: {input:?}");
    let repo_path = lrngit_repo_path.to_owned() + &args[1];
    if !Path::new(&repo_path).exists() {
        eprintln!("ERR repository doesn't exist");
        exit(1)
    }
    set_current_dir(repo_path).expect("Failed to change current dir");
}
