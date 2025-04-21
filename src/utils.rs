use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process::Command,
};

use crate::add;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectHeader {
    pub types: Vec<u8>,
    pub size: usize,
}

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
    println!("{}", String::from_utf8_lossy(&buffer));
    //TODO parse buffer to tree or blob struct to display
}

pub fn ls_file() {
    let config = add::index::parse_index();
    for each in config.entries {
        println!(
            "{:o} {} {} {}\n",
            each.mode,
            hex::encode(each.hash),
            each.flag,
            String::from_utf8_lossy(&each.path)
        );
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

/// The `compress_file` function compresses a vector of bytes using zlib compression.
///
/// Arguments:
///
/// * `vec`: The `vec` parameter in the `compress_file` function is a vector of unsigned 8-bit integers
/// (`Vec<u8>`) that represents the data of a file that you want to compress using the zlib compression
/// algorithm.
///
/// Returns:
///
/// The function `compress_file` returns a `Vec<u8>` containing the compressed bytes of the input
/// `Vec<u8>` after compressing it using zlib compression.
pub fn compress_file(vec: Vec<u8>) -> Vec<u8> {
    // compress file to zlib
    let mut compress_file = ZlibEncoder::new(Vec::new(), Compression::default());
    let compress_file_write = compress_file.write_all(&vec);
    match compress_file_write {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log_with_code(
                "Failed to add file to local repository",
                &e.to_string(),
            );
            return vec![];
        }
    }
    let compressed_bytes = compress_file.finish();
    let compressed_bytes_vec: Vec<u8>;
    match compressed_bytes {
        Ok(v) => compressed_bytes_vec = v,
        Err(e) => {
            lrncore::logs::error_log_with_code(
                "Failed to add file to local repository",
                &e.to_string(),
            );
            return vec![];
        }
    }
    compressed_bytes_vec
}

/// The function `new_file_dir` creates a new file in a specified directory based on input characters.
///
/// Arguments:
///
/// * `hash_vec`: The `hash_vec` parameter is a reference to a vector of characters. The function
/// `new_file_dir` takes this vector as input and performs the following operations:
///
/// Returns:
///
/// The function `new_file_dir` is returning a `Result` enum with the success variant containing a
/// `File` if the file creation is successful, and the error variant containing a `std::io::Error` if
/// there is an error during the file creation process.
pub fn new_file_dir(hash_vec: &Vec<char>) -> Result<File, std::io::Error> {
    let new_folder_name = format!("{}{}", hash_vec[0], hash_vec[1]);
    add_folder(&new_folder_name);
    let new_file_name = format!("{}", hash_vec[2..].iter().collect::<String>());
    let new_tree_path = format!(".lrngit/objects/{}/{}", new_folder_name, new_file_name);
    let file: File;
    match File::create(&new_tree_path) {
        Ok(f) => file = f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to create new tree file: {}", e));
            return Err(e);
        }
    };
    Ok(file)
}

/// The function `hash_sha1` calculates the SHA-1 hash of a given vector of bytes and returns the hash
/// as an array of bytes and as a vector of characters representing the hexadecimal hash.
///
/// Arguments:
///
/// * `data`: The `data` parameter is a reference to a vector of unsigned 8-bit integers (`Vec<u8>`),
/// which represents the data that you want to hash using the SHA-1 algorithm.
pub fn hash_sha1(data: &Vec<u8>) -> ([u8; 20], Vec<char>) {
    let mut new_hash = Sha1::new();
    new_hash.update(data);
    let hash_result = new_hash.finalize();
    let folder_hash = format!("{:#x}", hash_result);
    let split_hash_result_hex = folder_hash.chars().collect::<Vec<char>>();
    (hash_result.into(), split_hash_result_hex)
}

pub fn get_file_by_hash(hash: &str) -> File {
    let split_hash: Vec<char> = hash.chars().collect();
    let folder_name: String = format!("{}{}", split_hash[0], split_hash[1]);
    let file_name: String = split_hash[2..].iter().collect::<String>().to_string();
    let path = format!(".lrngit/objects/{}/{}", folder_name, file_name);
    File::open(path).expect("Failed to open file")
}

// pub fn get_hash_by_path(path: &str) -> String {
//
// }
