/*
Module handling all the add command, creating new blob objects or tree and saving them
in local repository
*/

use flate2::Compression;
use flate2::write::ZlibEncoder;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::Write;

use bincode;
use blob::{Blob, Standard};

mod helpers;

use crate::utils;

/// The `TreeEntry` struct in Rust represents an entry in a tree object with mode, name, and SHA-1 hash.
///
/// Properties:
///
/// * `mode`: The `mode` property in the `TreeEntry` struct represents the file mode or permissions of
/// the entry. It is typically a 32-bit unsigned integer that specifies the file type and permissions,
/// such as whether the entry is a file, directory, or symbolic link, and the read, write,
/// example: if the mode is `40000` it's a folder, else if it's `100644` it's a blob,
/// 160000 would be a commit
/// * `name`: The `name` property in the `TreeEntry` struct represents the name of the entry in the
/// tree. It is of type `String` and stores the name of the entry.
/// * `hash`: The `hash` property in the `TreeEntry` struct is an array of 20 unsigned 8-bit integers
/// (bytes). This array is used to store the SHA-1 hash value of the file or directory represented by
/// the `TreeEntry`. The SHA-1 hash is typically used to
#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(dead_code)]
struct TreeEntry {
    mode: u32,
    fileType: String,
    hash: [u8; 20],
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(dead_code)]
struct Tree {
    header: Vec<u8>,
    entries: Vec<u8>,
}

struct BlobObject {
    // "blob <size>\0" in binary
    header: Vec<u8>,
    content: Vec<u8>,
}

fn git_object_header(file_type: &str, content_length: usize) -> Vec<u8> {
    match file_type {
        "blob" => format!("blob {}\0", content_length).as_bytes().to_vec(),
        "tree" => format!("tree {}\0", content_length).as_bytes().to_vec(),
        _ => vec![],
    }
}

pub fn add_to_local_repo(arg: String) {
    let folder_vec: Vec<&str>;
    if arg.contains("/") {
        let folder_split: Vec<&str> = arg.split("/").collect();
        folder_vec = folder_split;
    } else {
        folder_vec = vec![&arg];
    }
    recursive_add(folder_vec, [0u8; 20], "".to_string(), "".to_string());
    // utils::read_blob_file();
}

//TODO
// can add new item to the tree vector, like update
// the current tree and not recreate one or panic if already exist

/// The function `add_tree` creates a new tree object, hashes its content with SHA-1, compresses it with
/// zlib, and writes it to a file in a local repository.
///
/// Arguments:
///
/// * `child`: The `child` parameter in the `add_tree` function represents the hash of the child object
/// that you want to add to the tree. It is of type `[u8; 20]`, which typically represents a SHA-1 hash
/// value in binary form.
/// * `name`: The `name` parameter in the `add_tree` function represents the name of the child tree
/// entry being added to the parent tree. It is a reference to a string (`&str`) that holds the name of
/// the child tree entry.
/// * `child_path`: The `child_path` parameter in the `add_tree` function represents the path to the
/// child object that you want to add to a tree object. It is used to determine the mode of the tree
/// entry for the child object.
///
/// Returns:
///
/// The function `add_tree` returns a `[u8; 20]` array, which represents the hash of the newly created
/// tree object.
fn add_tree(child: [u8; 20], name: &str, child_path: &str) -> [u8; 20] {
    // creation of tree entries
    let mode = helpers::define_tree_mode(child_path);
    let ftype: String;
    match mode {
        
    }
    let new_tree_entry: TreeEntry = TreeEntry {
        mode: mode,
        fileType: 
        hash: child,
        name: name.to_string(),
    };
    let mut tree_entry_vec: Vec<u8> = Vec::new();
    let tree_entry_string = format!(
        "{}\0{}\0{:?}",
        new_tree_entry.mode, new_tree_entry.name, new_tree_entry.hash
    );
    println!("prout {}", tree_entry_string);
    let tree_entry_bytes = tree_entry_string.as_bytes();
    tree_entry_vec.extend_from_slice(tree_entry_bytes);

    // creation of tree object
    let new_tree: Tree = Tree {
        header: git_object_header("tree", tree_entry_vec.len()),
        entries: tree_entry_vec,
    };
    let mut new_tree_concat = new_tree.header.clone();
    for entry in new_tree.entries.clone() {
        new_tree_concat.extend(bincode::serialize(&entry).unwrap());
    }
    // hash tree content with SHA-1
    let mut new_hash = Sha1::new();
    new_hash.update(&new_tree_concat);
    let hash_result = new_hash.finalize();
    let folder_hash = format!("{:#x}", hash_result);
    let split_hash_result_hex = folder_hash.chars().collect::<Vec<char>>();
    // create folder and file in local repository
    let new_folder_name = format!("{}{}", split_hash_result_hex[0], split_hash_result_hex[1]);
    utils::add_folder(&new_folder_name);
    let new_file_name = format!("{}", split_hash_result_hex[2..].iter().collect::<String>());
    let new_tree_path = format!(".lrngit/objects/{}/{}", new_folder_name, new_file_name);
    let mut file: File;
    match File::create(&new_tree_path) {
        Ok(f) => file = f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to create new tree file: {}", e));
            return [0u8; 20];
        }
    };
    // compress the new tree object with zlib
    let mut compress_file = ZlibEncoder::new(Vec::new(), Compression::default());
    let compress_file_write = compress_file.write_all(&new_tree_concat);
    match compress_file_write {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log_with_code(
                "Failed to add file to local repository",
                &e.to_string(),
            );
            return [0u8; 20];
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
            return [0u8; 20];
        }
    }
    // write zlib compressed into file
    let file_result = file.write_all(&compressed_bytes_vec);
    match file_result {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Error writing to tree file: {}", e));
            return [0u8; 20];
        }
    }
    hash_result.into()
}

/// The function `add_blob` reads a file, calculates its SHA-1 hash, creates a new blob, and stores the
/// file in a local repository with error handling.
///
/// Arguments:
///
/// * `arg`: The function `add_blob` takes a reference to a string `arg` as a parameter. This function
/// reads the contents of a file specified by the `arg`, calculates its SHA-1 hash, creates a new blob
/// from the hash, and then stores the blob in a local repository.
///
/// Returns:
///
/// The function `add_blob` returns a `String` which is the hexadecimal representation of the SHA-1 hash
/// of the file content that was read and added to the local repository.
fn add_blob(arg: &str) -> [u8; 20] {
    // read file content
    let read_file = fs::read_to_string(arg);
    let file: String;
    match read_file {
        Ok(file_as_string) => file = file_as_string,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to read the file: {}", e));
            return [0u8; 20];
        }
    }
    // creation of blob object
    let new_blob: Blob<Standard> = Blob::from(file.as_bytes());
    let blob_object: BlobObject = BlobObject {
        header: git_object_header("blob", new_blob.len()),
        content: new_blob.to_vec(),
    };
    // concat the blob object from struct
    let mut blob_object_concat = blob_object.header.clone();
    blob_object_concat.extend(blob_object.content.clone());
    // hash file content with SHA-1
    let mut new_hash = Sha1::new();
    new_hash.update(&blob_object_concat);
    let hash_result = new_hash.finalize();
    // hash to readable format
    let hash_result_hex = format!("{:#x}", hash_result);
    let split_hash_result_hex = hash_result_hex.chars().collect::<Vec<char>>();
    // creation of file to local repo
    let new_folder_name = format!("{}{}", split_hash_result_hex[0], split_hash_result_hex[1]);
    utils::add_folder(&new_folder_name);
    let new_file_name = format!("{}", split_hash_result_hex[2..].iter().collect::<String>());
    let file = fs::File::create(format!(
        ".lrngit/objects/{}/{}",
        new_folder_name, new_file_name
    ));
    let mut file_result: File;
    match file {
        Ok(f) => {
            file_result = f;
            lrncore::logs::info_log("File added to local repository")
        }
        Err(e) => {
            lrncore::logs::error_log_with_code(
                "Failed to add file to local repository",
                &e.to_string(),
            );
            return [0u8; 20];
        }
    }
    // compress file to zlib
    let mut compress_file = ZlibEncoder::new(Vec::new(), Compression::default());
    let compress_file_write = compress_file.write_all(&blob_object_concat);
    match compress_file_write {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log_with_code(
                "Failed to add file to local repository",
                &e.to_string(),
            );
            return [0u8; 20];
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
            return [0u8; 20];
        }
    }
    // write compress file with zlib to file
    file_result.write_all(&compressed_bytes_vec).unwrap();
    hash_result.into()
}

/// The `recursive_add` function in Rust recursively processes elements in a vector and performs
/// different actions based on whether the last element contains a period or not.
///
/// Arguments:
///
/// * `arg_vec`: arg_vec is a vector of string references that contains the elements being processed
/// recursively in the function.
/// * `child`: The `child` parameter in the `recursive_add` function seems to represent a string value
/// that is either empty or contains some data. It is used as an argument in the function calls to
/// `add_tree` and `recursive_add`.
fn recursive_add(
    mut arg_vec: Vec<&str>,
    mut child: [u8; 20],
    mut name: String,
    mut child_path: String,
) {
    if arg_vec.is_empty() {
        return;
    }
    let last = arg_vec.last().unwrap();
    let file_child_path = arg_vec.join("/");
    let metadata = fs::symlink_metadata(&file_child_path).expect("Failed to read path metadata");
    if metadata.file_type().is_file() {
        child_path = file_child_path;
        let new_blob = add_blob(&child_path);
        child = new_blob;
    } else {
        let new_tree = add_tree(child.clone(), &name, &child_path);
        child = new_tree;
        child_path = file_child_path;
    }
    name = last.to_string();
    arg_vec.pop();
    recursive_add(arg_vec, child, name, child_path);
}
