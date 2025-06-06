use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::{fs, io::{Read, Write}, path::Path, process::Command};

use serde::{Deserialize, Serialize};

use super::tree::print_tree_content;

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectHeader {
    pub types: Vec<u8>,
    pub size: usize,
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

/**
The function `git_object_header` generates a Git object header based on the file type and content
length provided.

Arguments:

* `file_type`: The `file_type` parameter represents the type of Git object, which can be either
  "blob" or "tree".
* `content_length`: The `content_length` parameter represents the length of the content associated
  with the Git object. It is used to construct the header of the Git object based on the specified
  `file_type`.

Returns:

The function `git_object_header` returns a vector of bytes representing the header of a Git object
based on the provided `file_type` and `content_length`. If the `file_type` is "blob", it will return
a byte vector containing the header "blob {content_length}\0". If the `file_type` is "tree", it will
return a byte vector containing the header "tree
*/
pub fn git_object_header(file_type: &str, content_length: usize) -> Vec<u8> {
    match file_type {
        "blob" => format!("blob {content_length}\0").as_bytes().to_vec(),
        "tree" => format!("tree {content_length}\0").as_bytes().to_vec(),
        "commit" => format!("commit {content_length}\0").as_bytes().to_vec(),
        _ => vec![],
    }
}

/**
The `compress_file` function compresses a vector of bytes using zlib compression.

Arguments:

* `vec`: The `vec` parameter in the `compress_file` function is a vector of unsigned 8-bit integers
  (`Vec<u8>`) that represents the data of a file that you want to compress using the zlib compression
  algorithm.

Returns:

The function `compress_file` returns a `Vec<u8>` containing the compressed bytes of the input
`Vec<u8>` after compressing it using zlib compression.
*/
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

    let compressed_bytes_vec: Vec<u8> = match compressed_bytes {
        Ok(v) => v,
        Err(e) => {
            lrncore::logs::error_log_with_code(
                "Failed to add file to local repository",
                &e.to_string(),
            );
            return vec![];
        }
    };
    compressed_bytes_vec
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
    // let header = split_object_header(buffer);
    // let header_string = String::from_utf8_lossy(&header[0]);
    // let header_split: Vec<&str> = header_string.split(" ").collect();
    // let magic = header_split[0];
    print_tree_content(&buf);
    // match magic {
    // "tree" => print_tree_content(&buf),
    // _ => (),
    // }
    //TODO parse buffer to tree or blob struct to display
}

