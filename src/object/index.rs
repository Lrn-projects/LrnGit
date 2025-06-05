use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
    process::exit,
};

use serde::{Deserialize, Serialize};

use crate::{object::commit, utils::walk_root_tree_content, refs::parse_current_branch};

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexHeader {
    pub magic_number: [u8; 4],
    pub version: u8,
    pub entry_count: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexObject {
    pub header: IndexHeader,
    pub entries: Vec<IndexEntry>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct IndexEntry {
    pub mtime: u32,
    pub file_size: u32,
    pub mode: u32,
    pub hash: [u8; 20],
    pub flag: u16,
    pub path: Vec<u8>,
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
            lrncore::logs::error_log(&format!("Error opening index file: {e}"));
            exit(1)
        }
    };
    match index_file.write_all(&index_bytes) {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to write in index file: {e}"));
            exit(1)
        }
    }
}

/// add a new indew entry to the index content
pub fn add_index_entry(mtime: u32, file_size: u32, mode: u32, hash: [u8; 20], path: Vec<u8>) {
    let index = parse_index();
    let mut header = index.header;
    let mut entries = index.entries;
    let new_entry: IndexEntry = IndexEntry {
        mtime,
        file_size,
        mode,
        hash,
        flag: 0,
        path,
    };
    entries.push(new_entry);
    entries.sort();
    header.entry_count += 1;
    let updated_index: IndexObject = IndexObject { header, entries };
    update_index(updated_index);
}

/// update index file with new index object
fn update_index(index: IndexObject) {
    let f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(".lrngit/index")
        .expect("Unable to open file");
    let index_as_bytes = bincode::serialize(&index).expect("Failed to serialize new indew file");
    let mut f = BufWriter::new(f);
    f.write_all(&index_as_bytes).expect("Unable to write data");
}

/// parse index file and return structure
pub fn parse_index() -> IndexObject {
    // get buffer from file
    let index_path = ".lrngit/index";
    let file = File::open(index_path).expect("Failed to open index file");
    let buffer = BufReader::new(file);
    let mut bytes_vec: Vec<u8> = Vec::new();
    for bytes in buffer.bytes() {
        bytes_vec.push(bytes.unwrap());
    }
    // header
    let header_size = std::mem::size_of::<IndexHeader>();
    let header_bytes = &bytes_vec[..header_size];
    let header: IndexHeader =
        bincode::deserialize(header_bytes).expect("Failed to deserialize header bytes");
    // contents
    let content_bytes = &bytes_vec[header_size..];
    let content: Vec<IndexEntry> =
        bincode::deserialize(content_bytes).expect("Failed to deserialize content bytes");
    // index
    let index: IndexObject = IndexObject {
        header,
        entries: content,
    };
    index
}

// Remove index entry by entry path
// used when adding a tracked file to avoid entry duplication
pub fn remove_index_entry(entry_path: &str) {
    let mut entries = parse_index().entries;
    if let Some(pos) = entries
        .iter()
        .position(|x| str::from_utf8(&x.path).unwrap() == entry_path)
    {
        entries.remove(pos);
        let magic_number = b"DIRC";
        let header: IndexHeader = IndexHeader {
            magic_number: *magic_number,
            version: 1,
            entry_count: 0,
        };
        let updated_index: IndexObject = IndexObject { header, entries };
        update_index(updated_index);
    }
}

/// Update the index file depending on the new ref head
pub fn recreate_index() {
    let last_commit = parse_current_branch();
    let parse_commit = commit::parse_commit_by_hash(&last_commit);
    let root_tree = hex::encode(&parse_commit.tree);
    let mut root_tree_content: Vec<(PathBuf, [u8; 20])> = Vec::new();
    walk_root_tree_content(&root_tree, &mut PathBuf::new(), &mut root_tree_content);
    root_tree_content.sort();
    root_tree_content.dedup();
    for each in root_tree_content {
        println!("name: {:?}\thash: {:?}", &each.0, hex::encode(&each.1));
    }
}
