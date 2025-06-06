use blob::{Blob, Standard};

use super::index;
use crate::{fs::new_file_dir, object::tree::RWO, utils};
use std::fs;
use std::fs::File;
use std::os::unix::fs::MetadataExt;
use std::io::Write;
use crate::object::utils::{git_object_header, compress_file};

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
pub fn add_blob(arg: &str) -> [u8; 20] {
    let blob_hash = calculate_file_hash_and_blob(arg)
        .expect("Failed to get the blob and the hash from the file path");
    // check index entry
    index::remove_index_entry(arg);
    // creation of file to local repo
    let mut file: File;
    let file_result = new_file_dir(&blob_hash.hash_split);
    match file_result {
        Ok(f) => file = f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Error writing to tree file: {e}"));
            return [0u8; 20];
        }
    }
    let compressed_bytes_vec = compress_file(blob_hash.blob);
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

// Calculate file hash and create blob object
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
        header: git_object_header("blob", new_blob.len()),
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
