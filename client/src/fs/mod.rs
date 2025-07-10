use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use lrngitcore::objects::index::TempIndex;

use crate::object::{blob, utils::get_path_by_hash};

/// Remove file at the end of the path and try to remove directory if empty  
pub fn delete_path(path: &PathBuf) {
    if !fs::exists(path).unwrap() {
        panic!("Error while removing path. Path does not exist.");
    }
    fs::remove_file(path).expect("Failed to remove path from disk");
}

pub fn write_files(buff: &[u8], path: &str) {
    if !fs::exists(path).expect("Failed to check if the path exist") {
        File::create_new(path).expect("Failed to create the new file");
    }
    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Failed to open/create file");
    file.write_all(buff).expect("Failed to write in file");
}

/// Update the working directory depending on the temporary index
///
pub fn update_workdir(temp_index: TempIndex) {
    for each in temp_index.to_delete_files {
        delete_path(&each);
    }
    for each in temp_index.new_files {
        let hash: &str = &hex::encode(each.1);
        let hash_char: Vec<char> = hash.chars().collect();
        let hash_path = get_path_by_hash(&hash_char);
        let blob_content = blob::read_blob_content(&hash_path);
        write_files(&blob_content, each.0.to_str().unwrap());
    }
    for each in temp_index.changed_files {
        let hash: &str = &hex::encode(each.hash);
        let hash_char: Vec<char> = hash.chars().collect();
        let hash_path = get_path_by_hash(&hash_char);
        let blob_content = blob::read_blob_content(&hash_path);
        write_files(&blob_content, str::from_utf8(&each.path).unwrap());
    }
}
