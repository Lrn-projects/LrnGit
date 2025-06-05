use std::{error::Error, io::Read};

use crate::fs::tree::{Tree, TreeEntry};

pub fn parse_tree_entries_obj(buff: Vec<u8>) -> Result<Vec<TreeEntry>, Box<dyn Error>> {
    let mut d = flate2::read::ZlibDecoder::new(buff.as_slice());
    let mut buffer = Vec::new();
    d.read_to_end(&mut buffer)
        .expect("Failed to read tree buffer for parsing");

    let entries: Tree = match bincode::deserialize(&buffer) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing tree");
            return Err(Box::new(e));
        }
    };
    Ok(entries.entries)
}
