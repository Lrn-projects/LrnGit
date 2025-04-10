use std::{fs::File, io::{BufReader, Read, Write}, process::exit};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexHeader {
    magic_number: [u8; 4],
    version: u8,
    entry_count: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexObject {
    header: IndexHeader,
    entries: Vec<IndexEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexEntry {
    mode: u32,
    hash: [u8;20],
    flag: u16,
    name: Vec<u8>
}

pub fn init_index() {
    // header
    let magic_number = b"DIRC";
    let header: IndexHeader = IndexHeader {
        magic_number: *magic_number,
        version: 1,
        entry_count: 0,
    };
    let mut header_bytes: Vec<u8> = Vec::new();
    header_bytes.extend_from_slice(&header.magic_number);
    header_bytes.push(header.version);
    header_bytes.push(header.entry_count);
    // index file
    let index: IndexObject = IndexObject {
        header,
        entries: vec![],
    };
    let index_bytes: Vec<u8> =
        bincode::serialize(&index).expect("Failed to serialize index struct into bytes");
    let mut index_file = match File::create(".lrngit/index") {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Error opening index file: {}", e));
            exit(1)
        }
    };
    match index_file.write_all(&index_bytes) {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to write in index file: {}", e));
            exit(1)
        }
    }
}

// parse index file and return structure
pub fn parse_index() -> IndexObject {
    // get buffer from file
    let index_path = ".lrngit/index";
    let file = File::open(index_path).expect("Failed to open index file");
    let buffer = BufReader::new(file);
    let mut bytes_vec: Vec<u8> = Vec::new();
    for bytes in buffer.bytes() {
        bytes_vec.push(bytes.unwrap());
    };
   // header
   let header_size = std::mem::size_of::<IndexHeader>();
   let header_bytes = &bytes_vec[..header_size];
   let header: IndexHeader = bincode::deserialize(header_bytes).expect("Failed to deserialize header bytes");
   // contents
   let content_bytes = &bytes_vec[header_size..];
   let content: Vec<IndexEntry> = bincode::deserialize(content_bytes).expect("Failed to deserialize content bytes");
   // index
   let index: IndexObject = IndexObject { header, entries: content };
   index
}
