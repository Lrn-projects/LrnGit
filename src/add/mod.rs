use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::Write;

use blob::{Blob, Standard};

use crate::utils;

pub fn add_to_local_repo(arg: String) {
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
    let split_hash_result_hex = hash_result_hex.chars().collect::<Vec<char>>();
    let new_folder_name = format!("{}{}", split_hash_result_hex[0], split_hash_result_hex[1]);
    utils::add_folder(&new_folder_name);
    let new_file_name = format!("{}", split_hash_result_hex[2..].iter().collect::<String>());
    let file = fs::File::create(format!(
        ".lrngit/objects/{}/{}",
        new_folder_name, new_file_name
    ));
    let mut file_result: File;
    match file {
        Ok(f) => {
            file_result = f;
            lrncore::logs::info_log("File added to local repository")
        }
        Err(e) => {
            lrncore::logs::error_log_with_code(
                "Failed to add file to local repository",
                &e.to_string(),
            );
            return;
        }
    }
    file_result.write_all(&new_blob).unwrap();
    println!("{}", hash_result_hex)
}
