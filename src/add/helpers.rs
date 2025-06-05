/*
Helper module for the add module, contain useful pub function
*/
#![allow(dead_code)]
use std::fs::{self};

use blob::{Blob, Standard};

use crate::utils::{self};

use super::{BlobObject, FileHashBlob};

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
