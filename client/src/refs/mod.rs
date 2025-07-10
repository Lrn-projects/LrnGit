use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub mod origin;

pub fn init_head() {
    let mut file = File::create(".lrngit/HEAD").expect("Failed to create HEAD file");
    file.write_all("ref: refs/heads/main".as_bytes())
        .expect("Failed to write in HEAD file");
}

/// get content of the HEAD file, ref of the current branch
pub fn parse_head() -> String {
    let mut head = File::open(".lrngit/HEAD").expect("Failed to open HEAD file");
    let mut content: String = String::new();
    head.read_to_string(&mut content)
        .expect("Failed to read HEAD file content");
    let split_content: Vec<&str> = content.split("ref: ").collect();
    split_content[1].to_string()
}

/// get last commit from the current HEAD
pub fn parse_current_branch() -> String {
    let head = parse_head();
    let branch_string = ".lrngit/".to_string() + &head;
    let branch_path = Path::new(&branch_string);
    if !Path::exists(branch_path) {
        return "".to_string();
    }
    let mut parse_branch = File::open(".lrngit/".to_string() + &head)
        .unwrap_or_else(|_| panic!("Failed to open {head} file"));
    let mut content: String = String::new();
    parse_branch
        .read_to_string(&mut content)
        .expect("Failed to read current branch content");
    content
}

pub fn init_refs(commit_hash: &[u8]) {
    let head_content = parse_head();
    let mut file = File::create(".lrngit/".to_string() + head_content.as_str())
        .expect("Failed to create refs/head file");
    file.write_all(commit_hash)
        .expect("Failed to write commit to branch file");
}
