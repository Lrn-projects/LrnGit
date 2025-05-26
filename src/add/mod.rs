use helpers::{RWO, calculate_file_hash_and_blob};
/*
Module handling all the add command, creating new blob objects or tree and saving them
in local repository
*/
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::MetadataExt;

use crate::utils;

pub mod helpers;
pub mod index;
/// The `TreeEntry` struct in Rust represents an entry in a tree object with mode, name, and SHA-1 hash.
///
/// Properties:
///
/// * `mode`: The `mode` property in the `TreeEntry` struct represents the file mode or permissions of
///   the entry. It is typically a 32-bit unsigned integer that specifies the file type and permissions,
///   such as whether the entry is a file, directory, or symbolic link, and the read, write,
///   example: if the mode is `40000` it's a folder, else if it's `100644` it's a blob,
///   160000 would be a commit
/// * `name`: The `name` property in the `TreeEntry` struct represents the name of the entry in the
///   tree. It is of type `String` and stores the name of the entry.
/// * `hash`: The `hash` property in the `TreeEntry` struct is an array of 20 unsigned 8-bit integers
///   (bytes). This array is used to store the SHA-1 hash value of the file or directory represented by
///   the `TreeEntry`. The SHA-1 hash is typically used to
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[allow(dead_code)]
pub struct TreeEntry {
    pub mode: u32,
    pub name: Vec<u8>,
    pub hash: [u8; 20],
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(dead_code)]
pub struct Tree {
    pub header: Vec<u8>,
    pub entries: Vec<TreeEntry>,
}

pub struct FileHashBlob {
    pub blob: Vec<u8>,
    pub hash: [u8; 20],
    pub hash_split: Vec<char>,
}

struct BlobObject {
    // "blob <size>\0" in binary
    header: Vec<u8>,
    content: Vec<u8>,
}

pub fn add_to_local_repo(arg: String) {
    let _folder_vec: Vec<&str> = if arg.contains("/") {
        let folder_split: Vec<&str> = arg.split("/").collect();
        folder_split
    } else {
        vec![&arg]
    };
    add_blob(&arg);
}

//TODO
// can add new item to the tree vector, like update
// the current tree and not recreate one or panic if already exist

/*
The function `add_tree` creates a new tree object, hashes its content with SHA-1, compresses it with
zlib, and writes it to a file in a local repository.

Arguments:

* `child`: The `child` parameter in the `add_tree` function represents the hash of the child object
that you want to add to the tree. It is of type `[u8; 20]`, which typically represents an SHA-1 hash
value in binary form.
* `name`: The `name` parameter in the `add_tree` function represents the name of the child tree
entry being added to the parent tree. It is a reference to a string (`&str`) that holds the name of
the child tree entry.
* `child_path`: The `child_path` parameter in the `add_tree` function represents the path to the
child object that you want to add to a tree object. It is used to determine the mode of the tree
entry for the child object.

Returns:

The function `add_tree` returns a `[u8; 20]` array, which represents the hash of the newly created
tree object.
*/
//TODO fix tree structure to make compatible with git
fn add_tree(child: [u8; 20], name: &str) -> [u8; 20] {
    // creation of tree entries
    let mode = helpers::DIR;
    let new_tree_entry: TreeEntry = TreeEntry {
        mode,
        name: name.as_bytes().to_vec(),
        hash: child,
    };
    let tree_entry_vec: Vec<TreeEntry> = vec![new_tree_entry.clone()];
    // creation of tree object
    let new_tree: Tree = Tree {
        header: utils::git_object_header("tree", tree_entry_vec.len()),
        entries: tree_entry_vec.clone(),
    };
    let tree_vec: Vec<u8> = bincode::serialize(&new_tree).expect("Failed to serialize new tree");
    // Compress the new tree object with zlib
    let compressed_bytes_vec = utils::compress_file(tree_vec);
    // hash tree content with SHA-1
    let (new_hash, split_hash_result_hex) = utils::hash_sha1(&compressed_bytes_vec);
    // File creation
    let mut file: File;
    let file_result = utils::new_file_dir(&split_hash_result_hex);
    match file_result {
        Ok(f) => file = f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Error writing to tree file: {e}"));
            return [0u8; 20];
        }
    }
    // write zlib compressed into file
    let file_result = file.write_all(&compressed_bytes_vec);
    match file_result {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Error writing to tree file: {e}"));
        }
    }
    // returned new created hash
    new_hash
}

/// The function `add_blob` reads a file, calculates its SHA-1 hash, creates a new blob, and stores the
/// file in a local repository with error handling.
///
/// Arguments:
///
/// * `arg`: The function `add_blob` takes a reference to a string `arg` as a parameter. This function
///   reads the contents of a file specified by the `arg`, calculates its SHA-1 hash, creates a new blob
///   from the hash, and then stores the blob in a local repository.
///
/// Returns:
///
/// The function `add_blob` returns a `String` which is the hexadecimal representation of the SHA-1 hash
/// of the file content that was read and added to the local repository.
fn add_blob(arg: &str) -> [u8; 20] {
    let blob_hash = calculate_file_hash_and_blob(arg)
        .expect("Failed to get the blob and the hash from the file path");
    // check index entry
    index::remove_index_entry(arg);
    // creation of file to local repo
    let mut file: File;
    let file_result = utils::new_file_dir(&blob_hash.hash_split);
    match file_result {
        Ok(f) => file = f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Error writing to tree file: {e}"));
            return [0u8; 20];
        }
    }
    let compressed_bytes_vec = utils::compress_file(blob_hash.blob);
    // write compress file with zlib to file
    file.write_all(&compressed_bytes_vec).unwrap();
    let added_file_metadata = fs::metadata(arg).expect("Failed to get added file metadata");
    let mtime: u32 = added_file_metadata.mtime().try_into().unwrap();
    let file_size: u32 = added_file_metadata.len().try_into().unwrap();
    let mode: u32 = RWO;
    let path = arg.to_string().into_bytes();
    index::add_index_entry(mtime, file_size, mode, blob_hash.hash, path);
    blob_hash.hash
}

/// The `recursive_add` function in Rust recursively processes elements in a vector and performs
/// different actions based on whether the last element contains a period or not.
///
/// Arguments:
///
/// * `arg_vec`: arg_vec is a vector of string references that contains the elements being processed
///   recursively in the function.
/// * `hash`: The `hash` parameter represent the hash of the object contained in the new tree
///   object
pub fn recursive_add(
    entity_hashmap: HashMap<(String, usize), Vec<(String, [u8; 20])>>,
    _root_tree_ptr: &mut [u8; 20],
) {
    let mut entity_vec: Vec<((String, usize), Vec<(String, [u8; 20])>)> = entity_hashmap
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    // Sorts the vector by the depth value of each tuple using default comparison 
    entity_vec.sort_by(|x,y| x.0.1.cmp(&y.0.1));
    println!("debug: {:?}", entity_vec); 
    // // add root folder tree object and break recursive
    // if arg_vec.is_empty() {
    //     let root_tree = add_tree(hash, &name);
    //     root_tree_ptr.copy_from_slice(&root_tree);
    //     return;
    // }
    // let last = arg_vec
    //     .last()
    //     .expect("Failed to get last element of file path");
    // let file_child_path = arg_vec.join("/");
    // match fs::symlink_metadata(&file_child_path) {
    //     Ok(_) => (),
    //     Err(_) => panic!("Failed to read path metadata"),
    // }
    // let new_tree = add_tree(hash, &name);
    // root_tree_ptr.copy_from_slice(&new_tree);
    // hash = new_tree;
    // root_tree_ptr.copy_from_slice(&new_tree);
    // name = last.to_string();
    // arg_vec.pop();
    // recursive_add(arg_vec, hash, name, root_tree_ptr);
}
