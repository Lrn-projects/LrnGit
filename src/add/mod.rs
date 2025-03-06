use std::{fs, str::FromStr};

use blob::Blob;

pub fn add_to_local_repo(arg: String) {
    println!("{}", arg);
    let read_file = fs::read(arg);
    let file: Vec<u8>;
    match read_file {
        Ok(file_as_string) => file = file_as_string,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to read the file: {}", e));
            return;
        }
    }
    let new_blob = Blob::from(file);
    println!("{}", new_blob);
}
