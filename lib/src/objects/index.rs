use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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
    pub new_files: Vec<(PathBuf, [u8; 20])>,
    pub changed_files: Vec<IndexEntry>,
    pub to_delete_files: Vec<PathBuf>,
}
