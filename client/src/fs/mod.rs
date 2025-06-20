use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
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
pub fn new_file_dir(hash_vec: &[char]) -> Result<File, std::io::Error> {
    let new_folder_name = format!("{}{}", hash_vec[0], hash_vec[1]);
    add_folder(&new_folder_name);
    let new_file_name = hash_vec[2..].iter().collect::<String>().to_string();
    let new_tree_path = format!(".lrngit/objects/{new_folder_name}/{new_file_name}");
    let file: File = match File::create(&new_tree_path) {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to create new tree file: {e}"));
            return Err(e);
        }
    };
    Ok(file)
}

/// Create a new folder in objects
pub fn add_folder(dir: &str) {
    if dir.is_empty() {
        return;
    }
    if Path::new(&format!(".lrngit/objects/{dir}")).exists() {
        return;
    }
    let new_dir_path = format!(".lrngit/objects/{dir}");
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

fn write_files(buff: &[u8], path: &str) {
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
