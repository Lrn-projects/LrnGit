use std::{fs::File, io::Write, path::Path, process::Command};

use crate::pack::upload::ObjectsPackData;

pub fn write_pack_to_disk(objects: Vec<ObjectsPackData>) {
    for each in objects {
        println!("debug write_pack_to_disk: {:?}", str::from_utf8(&each.hash).unwrap());
        let hash_chars: Vec<char> = hex::encode(each.hash).chars().collect();
        new_file_dir(&hash_chars, &each.data).expect("Failed to create objects");
    }
}

/// Create a new folder in objects
pub fn add_folder(dir: &str) {
    if dir.is_empty() {
        return;
    }
    if Path::new(&format!("objects/{dir}")).exists() {
        return;
    }
    let new_dir_path = format!("objects/{dir}");
    let mut mkdir = Command::new("mkdir")
        .arg(new_dir_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to create all directories");
    let wait_mkdir = mkdir.wait().expect("Failed to wait the mkdir command");
    if !wait_mkdir.success() {
        panic!("Failed to execute the mkdir command");
    }
}

/// The function `new_file_dir` creates a new file in a specified directory based on input characters.
///
/// Arguments:
///
/// * `hash_vec`: The `hash_vec` parameter is a reference to a vector of characters. The function
///   new_file_dir` takes this vector as input and performs the following operations:
///
/// Returns:
///
/// The function `new_file_dir` is returning a `Result` enum with the success variant containing a
/// `File` if the file creation is successful, and the error variant containing a `std::io::Error` if
/// there is an error during the file creation process.
pub fn new_file_dir(hash_vec: &[char], content: &[u8]) -> Result<(), std::io::Error> {
    let new_folder_name = format!("{}{}", hash_vec[0], hash_vec[1]);
    add_folder(&new_folder_name);
    let new_file_name = hash_vec[2..].iter().collect::<String>().to_string();
    let new_tree_path = format!("objects/{new_folder_name}/{new_file_name}");
    let mut file: File = match File::create(&new_tree_path) {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to create new tree file: {e}"));
            return Err(e);
        }
    };
    file.write_all(content).expect("Failed to write content to newly created object");
    Ok(())
}

