/*
Helper module for the add module, contain useful pub function
*/

use crate::utils;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

pub const SYM: u32 = 0o120000;
pub const DIR: u32 = 0o040000;
pub const EXE: u32 = 0o100755;
pub const RWO: u32 = 0o100644;

/// The function `define_tree_mode` determines the mode of a file (symlink, directory, executable, or
/// read-write).
///
/// Arguments:
///
/// * `path`: The function `define_tree_mode` takes a path as input and determines the mode of the file
/// or directory located at that path. The mode can be one of the following:
///
/// Returns:
///
/// The function `define_tree_mode` is returning a string based on the type of file at the given path.
/// The possible return values are:
/// - "SYM" for symbolic link
/// - "DIR" for directory
/// - "EXE" for executable file
/// - "RWO" for read-write file
pub fn define_tree_mode(path: &str) -> u32 {
    let metadata = fs::symlink_metadata(path).expect("Failed to read metadata");
    if metadata.file_type().is_symlink() {
        return SYM; // Symlink
    } else if metadata.file_type().is_dir() {
        return DIR; // Tree (directory)
    } else {
        let perm = metadata.permissions().mode();
        if perm & 0o111 != 0 {
            return EXE; // executable
        } else {
            return RWO; // RW
        }
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
    utils::add_folder(&new_folder_name);
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

