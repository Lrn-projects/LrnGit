use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::Write;

use blob::{Blob, Standard};

use crate::utils;

pub fn add_to_local_repo(arg: String) {
    let mut folder_vec: Vec<&str> = Vec::new();
    if arg.contains("/") {
        let folder_split: Vec<&str> = arg.split("/").collect();
        folder_vec = folder_split;
    }
    recursive_add(folder_vec, "".to_string());
}

fn add_tree(folder: &str) {
    let mut new_hash = Sha1::new();
    new_hash.update(folder);
    let hash_result = new_hash.finalize();
    let folder_hash = format!("{:#x}", hash_result);
}

fn add_blob(arg: &str) -> String {
    let read_file = fs::read_to_string(arg);
    let file: String;
    match read_file {
        Ok(file_as_string) => file = file_as_string,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to read the file: {}", e));
            return "".to_string();
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
            return "".to_string();
        }
    }
    file_result.write_all(&new_blob).unwrap();
    hash_result_hex
}

fn recursive_add(mut arg_vec: Vec<&str>, child: String) {
    let last = arg_vec.last().unwrap();
    // println!("last {}", last);
    let mut child: String = "".to_string();
    if last.contains(".") {
        let file_path = utils::concat_elem_vec(arg_vec.clone());
        let new_blob = add_blob(&file_path);
        child = new_blob
    } else {
        let new_tree = add_tree(&last);
    }
    arg_vec.pop();
    recursive_add(arg_vec, child);
}
