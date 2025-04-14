use std::{env, fs, io::Read, path::Path, process::Command};

use crate::add;

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

    usage
}

pub fn change_wkdir(dir: &str) {
    env::set_current_dir(dir).expect("Failed to change directory");
}

pub fn add_folder(dir: &str) {
    if dir.is_empty() {
        return;
    }
    if Path::new(&format!(".lrngit/objects/{}", dir)).exists() {
        return;
    }
    let new_dir_path = format!(".lrngit/objects/{}", dir);
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

/// The function `read_blob_file` reads a compressed file, decompresses it, and prints its contents as a
/// string.
pub fn read_blob_file(hash: &str) {
    let hash_char: Vec<char> = hash.chars().collect();
    let folder: String = format!("{}{}", hash_char[0], hash_char[1]);
    let object: String = hash_char[2..].iter().collect();
    let object_path = format!(".lrngit/objects/{}/{}", &folder, &object);
    let mut read_file = fs::File::open(object_path).expect("Failed to open file");
    let mut buf = Vec::new();
    read_file
        .read_to_end(&mut buf)
        .expect("Failed to read file");
    let mut d = flate2::read::ZlibDecoder::new(buf.as_slice());
    let mut buffer = Vec::new();
    d.read_to_end(&mut buffer).unwrap();
    //TODO parse buffer to tree or blob struct to display
}

pub fn ls_file() {
    let config = add::index::parse_index();
    for each in config.entries {
        println!("{:o} {} {} {}\n", each.mode, hex::encode(each.hash), each.flag, String::from_utf8_lossy(&each.path));
    }
}

/// The function `git_object_header` generates a Git object header based on the file type and content
/// length provided.
///
/// Arguments:
///
/// * `file_type`: The `file_type` parameter represents the type of Git object, which can be either
/// "blob" or "tree".
/// * `content_length`: The `content_length` parameter represents the length of the content associated
/// with the Git object. It is used to construct the header of the Git object based on the specified
/// `file_type`.
///
/// Returns:
///
/// The function `git_object_header` returns a vector of bytes representing the header of a Git object
/// based on the provided `file_type` and `content_length`. If the `file_type` is "blob", it will return
/// a byte vector containing the header "blob {content_length}\0". If the `file_type` is "tree", it will
/// return a byte vector containing the header "tree
pub fn git_object_header(file_type: &str, content_length: usize) -> Vec<u8> {
    match file_type {
        "blob" => format!("blob {}\0", content_length).as_bytes().to_vec(),
        "tree" => format!("tree {}\0", content_length).as_bytes().to_vec(),
        "commit" => format!("commit {}\0", content_length).as_bytes().to_vec(),
        _ => vec![],
    }
}
