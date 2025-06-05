use helpers::calculate_file_hash_and_blob;
/*
Module handling all the add command, creating new blob objects or tree and saving them
in local repository
*/
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::MetadataExt;

use crate::utils;
use crate::object::tree::RWO;

pub mod helpers;
use crate::object::index;

pub struct FileHashBlob {
    pub blob: Vec<u8>,
    pub hash: [u8; 20],
    pub hash_split: Vec<char>,
}

struct BlobObject {
    // "blob <size>\0" in binary
    header: Vec<u8>,
    content: Vec<u8>,
}

pub fn add_to_local_repo(arg: String) {
    let _folder_vec: Vec<&str> = if arg.contains("/") {
        let folder_split: Vec<&str> = arg.split("/").collect();
        folder_split
    } else {
        vec![&arg]
    };
    add_blob(&arg);
}

/// The function `add_blob` reads a file, calculates its SHA-1 hash, creates a new blob, and stores the
/// file in a local repository with error handling.
///
/// Arguments:
///
/// * `arg`: The function `add_blob` takes a reference to a string `arg` as a parameter. This function
///   reads the contents of a file specified by the `arg`, calculates its SHA-1 hash, creates a new blob
///   from the hash, and then stores the blob in a local repository.
///
/// Returns:
///
/// The function `add_blob` returns a `String` which is the hexadecimal representation of the SHA-1 hash
/// of the file content that was read and added to the local repository.
fn add_blob(arg: &str) -> [u8; 20] {
    let blob_hash = calculate_file_hash_and_blob(arg)
        .expect("Failed to get the blob and the hash from the file path");
    // check index entry
    index::remove_index_entry(arg);
    // creation of file to local repo
    let mut file: File;
    let file_result = utils::new_file_dir(&blob_hash.hash_split);
    match file_result {
        Ok(f) => file = f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Error writing to tree file: {e}"));
            return [0u8; 20];
        }
    }
    let compressed_bytes_vec = utils::compress_file(blob_hash.blob);
    // write compress file with zlib to file
    file.write_all(&compressed_bytes_vec).unwrap();
    let added_file_metadata = fs::metadata(arg).expect("Failed to get added file metadata");
    let mtime: u32 = added_file_metadata.mtime().try_into().unwrap();
    let file_size: u32 = added_file_metadata.len().try_into().unwrap();
    let mode: u32 = RWO;
    let path = arg.to_string().into_bytes();
    index::add_index_entry(mtime, file_size, mode, blob_hash.hash, path);
    blob_hash.hash
}
