use std::{fs::read_dir, path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
 pub struct ObjectHeader {
     pub types: Vec<u8>,
     pub size: usize,
 }

/// Split the given hash to return the path to the hash object
pub fn split_hash(hash: &str) -> String {
    let split_hash: Vec<char> = hash.chars().collect();
    let folder_name: String = format!("{}{}", split_hash[0], split_hash[1]);
    let file_name: String = split_hash[2..].iter().collect::<String>().to_string();
    let path = format!(".lrngit/objects/{folder_name}/{file_name}");
    path
}

pub fn get_all_object(path: &str, object_vec: &mut Vec<PathBuf>) {
    let object_path = PathBuf::from_str(path).expect("Failed to create new pathbuf from object path");
    for object_read_dir in read_dir(object_path).unwrap() {
        let object = object_read_dir.expect("Failed to get object in objects");
        let object_metadata = object.metadata().expect("Failed to get object metadata");
        if object_metadata.is_file() {
            object_vec.push(object.path());
        } else if object_metadata.is_dir() {
            get_all_object(object.path().as_os_str().to_str().expect("Failed to cast os_str to str"), object_vec);
        }
    }
}
