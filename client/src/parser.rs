use std::{error::Error, io::Read};

use lrngitcore::objects::{tree::{Tree, TreeEntry}, utils::split_object_header};

pub fn parse_tree_entries_obj(buff: Vec<u8>) -> Result<Vec<TreeEntry>, Box<dyn Error>> {
    let mut d = flate2::read::ZlibDecoder::new(buff.as_slice());
    let mut buffer = Vec::new();
    d.read_to_end(&mut buffer)
        .expect("Failed to read tree buffer for parsing");
    let (_, content) = split_object_header(buffer);
    let entries: Vec<TreeEntry> = match bincode::deserialize(&content) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing tree");
            return Err(Box::new(e));
        }
    };
    Ok(entries)
}
