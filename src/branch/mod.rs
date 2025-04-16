use std::{fs::File, io::{Read, Write}};

pub fn init_head() {
    let mut file = File::create(".lrngit/HEAD").expect("Failed to create HEAD file");
    file.write_all("ref: refs/heads/main".as_bytes()).expect("Failed to write in HEAD file"); 
}

fn parse_head() -> String {
    let mut head = File::open(".lrngit/HEAD").expect("Failed to open HEAD file");
    let mut content: String = String::new();
    head.read_to_string(&mut content).expect("Failed to read HEAD file content");
    let split_content: Vec<&str> = content.split("ref: ").collect();
    split_content[1].to_string()
}

pub fn init_refs(commit_hash: &[u8]) {
    let head_content = parse_head();
    let mut file = File::create(".lrngit/".to_string() + head_content.as_str()).expect("Failed to create refs/head file");
    file.write_all(commit_hash).expect("Failed to write commit to branch file");

}
