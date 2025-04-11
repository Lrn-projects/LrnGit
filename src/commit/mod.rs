use crate::add::{self, index};

#[derive(Debug)]
struct Commit {
    // "commit <size>\0" in binary
    header: Vec<u8>,
    content: Vec<u8>,
}

struct CommitContent {
    tree: [u8; 24],
    author: Vec<u8>,
    commiter: Vec<u8>,
    message: String,
}

pub fn new_commit() {
    let config = index::parse_index();
    for each in config.entries {
        let path = String::from_utf8_lossy(&each.path);
        let mut folder_vec: Vec<&str> = if path.contains("/") {
            let folder_split: Vec<&str> = path.split("/").collect();
            folder_split
        } else {
            vec![&path]
        };
        let file = folder_vec.pop().unwrap();
        add::recursive_add(folder_vec, each.hash, file.to_string(), String::new());
    }
}
