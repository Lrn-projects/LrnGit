use sha1::{Digest, Sha1};
use std::fs;

use blob::{Blob, Standard};

pub fn add_to_local_repo(arg: String) {
    println!("{}", arg);
    let read_file = fs::read_to_string(arg);
    let file: String;
    match read_file {
        Ok(file_as_string) => file = file_as_string,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to read the file: {}", e));
            return;
        }
    }
    let mut new_hash = Sha1::new();
    new_hash.update(file);
    let hash_result = new_hash.finalize();
    let new_blob: Blob<Standard> = Blob::from(hash_result.to_vec());
    let hash_result_hex = format!("{:#x}", hash_result);
    let mut split_hash_result_hex = hash_result_hex.chars().collect::<Vec<char>>();
    let mut new_folder_name = format!("{}{}", split_hash_result_hex[0], split_hash_result_hex[1]);
    println!("{}", new_folder_name);
}
