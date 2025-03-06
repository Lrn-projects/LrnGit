use std::{env, fs, process::Command, ptr::null};

use blob::{Blob, Standard};
use sha1::{Digest, Sha1};

pub fn lrngit_usage() -> &'static str {
    let usage = r"
lrngit's cli.


Usage: lrngit command [options]


Commands:
    init            Init a local repository
    add             Add file to local repository
    help            Show this help message

Options:

    -h, --help      Show command usage
    -v, --version   Show the current version of LrnGit
";

    return usage;
}

pub fn change_wkdir(dir: &str) {
    env::set_current_dir(dir).expect("Failed to change directory");
}

pub fn add_folder(dir: &str) {
    let new_dir_path = format!(".lrngit/objects/{}", dir);
    Command::new("mkdir")
        .arg(new_dir_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to create all directories");
}

pub fn hash_and_blob_file(file: &str) -> String {
    let read_file = fs::read_to_string(file);
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
    let hash_result_hex = format!("{:#x}", hash_result);
    let split_hash_result_hex = hash_result_hex.chars().collect::<Vec<char>>();
    let new_folder_name = format!("{}{}", split_hash_result_hex[0], split_hash_result_hex[1]);
    let new_file_name = format!("{}", split_hash_result_hex[2..].iter().collect::<String>());
    let file_path = format!("{}/{}", new_folder_name, new_file_name);
    println!("{}", file_path);
    file_path
}
