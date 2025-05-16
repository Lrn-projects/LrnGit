/*
Helper module for the add module, contain useful pub function
*/
#![allow(dead_code)]
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::PermissionsExt,
};

use blob::{Blob, Standard};

use crate::{
    parser,
    utils::{self, add_folder},
};

use super::{BlobObject, FileHashBlob, TreeEntry};

pub const SYM: u32 = 0o120000;
pub const DIR: u32 = 0o040000;
pub const EXE: u32 = 0o100755;
pub const RWO: u32 = 0o100644;

/// The function `define_tree_mode` determines the mode of a file (symlink, directory, executable, or
/// read-write).
///
/// Arguments:
///
/// * `path`: The function `define_tree_mode` takes a path as input and determines the mode of the file
///   or directory located at that path. The mode can be one of the following:
///
/// Returns:
///
/// The function `define_tree_mode` is returning a string based on the type of file at the given path.
/// The possible return values are:
/// - "SYM" for symbolic link
/// - "DIR" for directory
/// - "EXE" for executable file
/// - "RWO" for read-write file
pub fn define_tree_mode(path: &str) -> u32 {
    let metadata = fs::symlink_metadata(path).expect("Failed to read metadata");
    if metadata.file_type().is_symlink() {
        SYM // Symlink
    } else if metadata.file_type().is_dir() {
        return DIR; // Tree (directory)
    } else {
        let perm = metadata.permissions().mode();
        if perm & 0o111 != 0 {
            return EXE; // executable
        } else {
            return RWO; // RW
        }
    }
}

// calculate file hash and create blob object
pub fn calculate_file_hash_and_blob(file_path: &str) -> Result<FileHashBlob, std::io::Error> {
    // read file content
    let read_file = fs::read_to_string(file_path);
    let file: String = match read_file {
        Ok(file_as_string) => file_as_string,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to read the file: {e}"));
            return Err(e);
        }
    };
    // creation of blob object
    let new_blob: Blob<Standard> = Blob::from(file.as_bytes());
    let blob_object: BlobObject = BlobObject {
        header: utils::git_object_header("blob", new_blob.len()),
        content: new_blob.to_vec(),
    };
    // concat the blob object from struct
    let mut blob_object_concat = blob_object.header.clone();
    blob_object_concat.extend(blob_object.content.clone());
    // hash file content with SHA-1
    let new_hash: [u8; 20];
    let split_hash_result_hex: Vec<char>;
    (new_hash, split_hash_result_hex) = utils::hash_sha1(&blob_object_concat);
    Ok(FileHashBlob {
        blob: blob_object_concat,
        hash: new_hash,
        hash_split: split_hash_result_hex,
    })
}

pub fn check_objects_exist(path: &str) -> bool {
    fs::exists(path).unwrap()
}

pub fn create_new_tree(path: &Vec<char>, buff: Vec<u8>) {
    let mut file: File;
    let file_result = utils::new_file_dir(path);
    match file_result {
        Ok(f) => file = f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Error writing to tree file: {e}"));
            return ;
        }
    }
    // write zlib compressed into file
    let file_result = file.write_all(&buff);
    match file_result {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Error writing to tree file: {e}"));
            return;
        }
    }
}

pub fn append_existing_tree(path: &str, new_entry: &TreeEntry) {
    let mut tree_obj = File::open(path).expect("Failed to open root tree file");
    let mut file_buff: Vec<u8> = Vec::new();
    tree_obj
        .read_to_end(&mut file_buff)
        .expect("Failed to read root tree content to buffer");
    let mut parse_tree =
        parser::parse_tree_entries_obj(file_buff).expect("Failed to parse tree object");
    parse_tree.push(new_entry.clone());
    let buff = bincode::serialize(&parse_tree).expect("Failed to serialize tree object");
    let mut file = OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(true)
        .open(path)
        .expect("Failed to open the tree object file");
    file.write_all(&buff)
        .expect("Failed to append to tree object file");
}
