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
/// * `sha1`: The `sha1` property in the `TreeEntry` struct is an array of 20 unsigned 8-bit integers
/// (bytes). This array is used to store the SHA-1 hash value of the file or directory represented by
/// the `TreeEntry`. The SHA-1 hash is typically used to
#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
struct TreeEntry {
    mode: u32,
    sha1: [u8; 20],
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
struct Tree {
    entries: Vec<TreeEntry>,
}

pub fn add_to_local_repo(arg: String) {
    let mut folder_vec: Vec<&str> = Vec::new();
    if arg.contains("/") {
        let folder_split: Vec<&str> = arg.split("/").collect();
        folder_vec = folder_split;
    }
    recursive_add(folder_vec, [0u8; 20], "".to_string(), "".to_string());
    utils::read_blob_file();
}

//TODO
// can add new item to the tree vector, like update
// the current tree and not recreate one or panic if already exist
fn add_tree(folder: &str, child: [u8; 20], name: &str, child_path: &str) -> [u8; 20] {
    let mut new_hash = Sha1::new();
    new_hash.update(folder);
    let hash_result = new_hash.finalize();
    let folder_hash = format!("{:#x}", hash_result);
    let split_hash_result_hex = folder_hash.chars().collect::<Vec<char>>();
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
    let mode = helpers::define_tree_mode(child_path);
    let new_tree_entry: TreeEntry = TreeEntry {
        mode: mode,
        sha1: child,
        name: name.to_string(),
    };
    let mut tree_entry_vec: Vec<TreeEntry> = Vec::new();
    tree_entry_vec.push(new_tree_entry);

    let new_tree: Tree = Tree {
        entries: tree_entry_vec,
    };
    let buffer: Vec<u8>;
    let new_tree_buffer = bincode::serialize(&new_tree);
    match new_tree_buffer {
        Ok(b) => buffer = b,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to create buffer for new tree: {}", e));
            return [0u8; 20];
        }
    }
    let file_result = file.write(&buffer);
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
    let read_file = fs::read_to_string(arg);
    let file: String;
    match read_file {
        Ok(file_as_string) => file = file_as_string,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to read the file: {}", e));
            return [0u8; 20];
        }
    }
    let mut new_hash = Sha1::new();
    new_hash.update(file);
    let hash_result = new_hash.finalize();
    let new_blob: Blob<Standard> = Blob::from(hash_result.to_vec());
    let hash_result_hex = format!("{:#x}", hash_result);
    let split_hash_result_hex = hash_result_hex.chars().collect::<Vec<char>>();
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
    file_result.write_all(&new_blob).unwrap();
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
        let new_tree = add_tree(&last, child.clone(), &name, &child_path);
        child = new_tree;
        child_path = file_child_path;
    }
    name = last.to_string();
    arg_vec.pop();
    recursive_add(arg_vec, child, name, child_path);
}
