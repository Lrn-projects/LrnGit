use std::{fs::File, io::Write, process::exit};

use serde::Serialize;

pub struct IndexHeader {
    magic_number: [u8; 4],
    version: u8,
    entry_count: u8,
}

#[derive(Debug, Serialize)]
pub struct IndexObject {
    header: Vec<u8>,
    entries: Vec<u8>,
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
        header: header_bytes,
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

pub fn parse_index() {

}
