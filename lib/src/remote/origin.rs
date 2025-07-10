use std::{fs::File, io::Read};

pub fn parse_origin_head() -> Vec<u8> {
    let path: &str = ".lrngit/refs/remotes/origin/HEAD"; 
    let mut head: File = File::open(path).expect("Failed to open origin HEAD file");
    let mut buff: Vec<u8> = Vec::new();
    head.read_to_end(&mut buff).expect("Failed to read origin HEAD content");
    buff
}

pub fn parse_origin_branch() -> String {
    let head: String = String::from_utf8(parse_origin_head()).expect("Failed to cast origin HEAD buffer to String");
    let head_split: Vec<&str> = head.split("ref: ").collect();
    let branch_path = ".lrngit/".to_owned() + head_split[1];
    let mut origin_branch: File = File::open(branch_path).expect("Failed to open origin branch"); 
    let mut buff: String = String::new();
    origin_branch.read_to_string(&mut buff).expect("Failed to read origin branch content");
    buff
}
