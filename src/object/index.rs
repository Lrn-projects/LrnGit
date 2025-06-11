use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter, Read, Write},
    os::unix::fs::MetadataExt,
    path::PathBuf,
    process::exit,
};

use serde::{Deserialize, Serialize};

use crate::{
    object::{commit, utils::walk_root_tree_content},
    refs::parse_current_branch,
};

use super::tree::RWO;

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

/// Structure used to store the temporary index and the entries sorted in different vectors
/// Use when recreating a temporary index when switching branch
#[derive(Clone)]
pub struct TempIndex {
    pub temp_index: Vec<(PathBuf, [u8; 20])>,
    pub unchanged_files: Vec<IndexEntry>,
    pub changed_files: Vec<IndexEntry>,
    pub to_delete_files: Vec<PathBuf>,
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

/// Remove index entry by entry path
/// used when adding a tracked file to avoid entry duplication
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

/// Recreate an index base on the specified branch.
/// Parse the last commit and recreate a temporary index and compare it with the current index.
///
///
pub fn build_temp_index(current_index: IndexObject) -> TempIndex {
    let last_commit = parse_current_branch();
    let parse_commit = commit::parse_commit_by_hash(&last_commit);
    let root_tree = hex::encode(parse_commit.tree);
    let mut temp_index: Vec<(PathBuf, [u8; 20])> = Vec::new();
    walk_root_tree_content(&root_tree, &mut PathBuf::new(), &mut temp_index);
    temp_index.sort();
    temp_index.dedup();

    // Entries that doesn't change while switching branch
    let mut same_entries: Vec<IndexEntry> = Vec::new();
    // Entries that has been modified between branches
    let mut modified_entries: Vec<IndexEntry> = Vec::new();
    // Entries that doesn't exist on the branch switch to
    let mut deleted_entries: Vec<PathBuf> = Vec::new();
    // Compare current index entries with temp index entries
    // find occurrence with different hash -> store in modified_entries vector
    // find occurrence with same hash -> store in same_entries vector
    // don't find occurrence -> store path in deleted_entries vector
    // TODO 
    // find a way to fix and to catch when a file is not present in the temp index
    for each in current_index.entries {
        if let Some(entry) = temp_index
            .iter()
            .find(|x| x.0 == PathBuf::from(str::from_utf8(&each.path).unwrap()))
        {
            if entry.1 != each.hash {
                modified_entries.push(each.clone());
            } else {
                same_entries.push(each.clone());
            }
        } else {
            deleted_entries.push(PathBuf::from(str::from_utf8(&each.path).unwrap()));
        }
    }
    TempIndex {
        temp_index,
        unchanged_files: same_entries,
        changed_files: modified_entries,
        to_delete_files: deleted_entries,
    }
}

/// Rebuild index from temporary index contents
/// Used when switching refs
pub fn rebuild_index(index: Vec<(PathBuf, [u8; 20])>) {
    let mut entry_vec: Vec<IndexEntry> = Vec::new();
    // Create index entry for each in temporary index
    for each in index {
        let metadata = fs::metadata(&each.0).expect("Failed to get file metadata");
        let mtime: u32 = metadata
            .mtime()
            .try_into()
            .expect("Failed to get file mtime");
        let file_size: u32 = metadata
            .len()
            .try_into()
            .expect("Failed to get the len of file");
        let mode: u32 = RWO;
        let path: Vec<u8> = each
            .0
            .into_os_string()
            .to_str()
            .expect("Failed to cast pathbuf as os string")
            .as_bytes()
            .to_owned();
        let entry: IndexEntry = IndexEntry {
            mtime,
            file_size,
            mode,
            hash: each.1,
            flag: 0,
            path,
        };
        entry_vec.push(entry);
    }
    // header
    let magic_number = b"DIRC";
    let header: IndexHeader = IndexHeader {
        magic_number: *magic_number,
        version: 1,
        entry_count: entry_vec.len() as u8,
    };
    let mut header_bytes: Vec<u8> = Vec::new();
    header_bytes.extend_from_slice(&header.magic_number);
    header_bytes.push(header.version);
    header_bytes.push(header.entry_count);
    // index file
    let index: IndexObject = IndexObject {
        header,
        entries: entry_vec,
    };
    let index_bytes: Vec<u8> =
        bincode::serialize(&index).expect("Failed to serialize index struct into bytes");
    // Write in index file
    let mut file = OpenOptions::new()
        .read(false)
        .write(true).create(false)
        .append(false)
        .open(".lrngit/index").expect("Failed to open index file");
   file.write_all(&index_bytes).expect("Failed to write in index file");
}

/// Display the content of the index file
pub fn ls_file() {
    let config = parse_index();
    for each in config.entries {
        println!(
            "{:o} {} {} {}\n",
            each.mode,
            hex::encode(each.hash),
            each.flag,
            String::from_utf8_lossy(&each.path)
        );
    }
}
