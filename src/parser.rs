use std::error::Error;

use crate::add::{Tree, TreeEntry};

pub fn parse_tree_entries_obj(buff: Vec<u8>) -> Result<TreeEntry, Box<dyn Error>> {
    println!("debug: {:?}", buff);
    let entries: Tree = match bincode::deserialize(&buff) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing tree");
            return Err(Box::new(e));
        }
    };
    println!("debug {entries:?}");
    let content: TreeEntry = match bincode::deserialize(&buff) {
        Ok(c) => c,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    Ok(content)
}
