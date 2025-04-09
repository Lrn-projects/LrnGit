#[derive(Debug)]
struct Commit {
    // "commit <size>\0" in binary
    header: Vec<u8>,
    content: Vec<u8>
}

struct CommitContent {
    tree: [u8; 24],
    author: Vec<u8>,
    commiter: Vec<u8>,
    message: String,
}

pub fn new_commit() {
    
    println!("prout");
}
