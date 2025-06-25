use std::{
    env::{current_dir, set_current_dir},
    io::{stdin, BufRead},
    path::Path,
    process::exit,
};

fn main() {
    println!("[SERVICE] lrngit-receive");
    let stdin = stdin();
    let lrngit_repo_path: &str = "/home/ubuntu/lrngit/repositories";
    let mut lines: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        let line_str = line.unwrap();
        lines.push(line_str);
    }
    let repo_path = lrngit_repo_path.to_owned() + &lines[0];
    if !Path::new(&repo_path).exists() {
        exit(1)
    }
    set_current_dir(repo_path).expect("Failed to change current dir");
    println!("pwd: {:?}", current_dir());
}
