use std::{fs::{self, File}, io::Write, os::unix::fs::PermissionsExt};

use crate::{parser, types::{BatchIndexEntriesMap, BatchIndexEntriesTuple, BatchIndexEntriesVec}};
use lrngitcore::{fs::new_file_dir, objects::tree::{Tree, TreeEntry, DIR, EXE, RWO, SYM}};
use crate::object::utils::{git_object_header, compress_file};

use super::utils::hash_sha1;

/// The function `define_tree_mode` determines the mode of a file (symlink, directory, executable, or
/// read-write).
///
/// Arguments:
///
/// * `path`: The function `define_tree_mode` takes a path as input and determines the mode of the file
///   or directory located at that path. The mode can be one of the following:
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
        SYM // Symlink
    } else if metadata.file_type().is_dir() {
        DIR// Tree (directory)
    } else {
        let perm = metadata.permissions().mode();
        if perm & 0o111 != 0 {
            EXE// executable
        } else {
            RWO// RW
        }
    }
}


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
fn add_tree(entries: Vec<(String, u32, [u8; 20])>) -> [u8; 20] {
    // creation of tree entries
    let mut new_tree_entry_vec: Vec<TreeEntry> = Vec::new();
    for each in entries {
        let new_tree_entry: TreeEntry = TreeEntry {
            mode: each.1,
            name: each.0.as_bytes().to_vec(),
            hash: each.2,
        };
        new_tree_entry_vec.push(new_tree_entry);
    }

    // creation of tree object
    let new_tree: Tree = Tree {
        header: git_object_header("tree", new_tree_entry_vec.len()),
        entries: new_tree_entry_vec,
    };
    let mut tree_concat = new_tree.header;
    let tree_entries_buff: Vec<u8> = bincode::serialize(&new_tree.entries).expect("Failed to serialize tree entries to bytes");
    tree_concat.extend(tree_entries_buff);
    // Compress the new tree object with zlib
    let compressed_bytes_vec = compress_file(tree_concat);
    // hash tree content with SHA-1
    let (new_hash, split_hash_result_hex) = hash_sha1(&compressed_bytes_vec);
    // File creation
    let mut file: File;
    let file_result = new_file_dir(&split_hash_result_hex);
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

/// The `batch_tree_add` function batch create all needed tree from the hashmap
///
/// Arguments:
///
/// * `entity_hashmap`: entity_hashmap is a hashmap containing all entries from index file split
///   and sort in separate tree's.
/// * `hash`: The `hash` parameter represent the hash of the object contained in the new tree
///   object
pub fn batch_tree_add(
    entity_hashmap: BatchIndexEntriesMap,
    root_tree_ptr: &mut [u8; 20],
) {
    let mut entity_vec: BatchIndexEntriesVec = entity_hashmap
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    // Sorts the vector by the depth value of each tuple using default comparison
    entity_vec.sort_by(|x, y| x.0.1.cmp(&y.0.1));
    entity_vec.reverse();
    let mut tree_hash_vec: Vec<(String, u32, [u8; 20])> = Vec::new();
    for each in entity_vec {
        let (tree_name, hash) = sort_hashmap_entry_and_create_tree(each, tree_hash_vec.clone());
        tree_hash_vec.push((tree_name, 1, hash));
    }
    let mut root_tree = (String::new(), 0, [0u8; 20]);
    if let Some(i) = tree_hash_vec.iter().position(|x| x.0.is_empty()) {
        root_tree = tree_hash_vec.remove(i);
    }
    *root_tree_ptr = root_tree.2;
}

/// Sort the hashmap depending on the buffer content and create the specific tree
///
/// Arguments:
///
/// * `entry`: the hashmap with all entry inside
/// * `tree_vec`: vector containing all tree that need to be created
fn sort_hashmap_entry_and_create_tree(
    entry: BatchIndexEntriesTuple,
    tree_vec: Vec<(String, u32, [u8; 20])>,
) -> (String, [u8; 20]) {
    let mut tree_entry_vec: Vec<(String, u32, [u8; 20])> = Vec::new();
    for each in entry.1 {
        if each.2 != [0u8; 20] {
            tree_entry_vec.push(each);
        } else {
            let mut existing_tree_hash: [u8; 20] = [0u8; 20];
            if let Some(i) = tree_vec.iter().position(|x| x.0 == each.0) {
                existing_tree_hash = tree_vec.get(i).unwrap().2;
            }
            if !existing_tree_hash.is_empty() {
                tree_entry_vec.push((each.0, each.1, existing_tree_hash));
            }
        }
    }
    let hash = add_tree(tree_entry_vec);
    (entry.0.0, hash)
}

/// Display the content of a tree file
pub fn print_tree_content(buff: &[u8]) {
    let parse_tree =
        parser::parse_tree_entries_obj(buff.to_vec()).expect("Failed to parse tree object");
    for each in parse_tree {
        println!("{:?}", str::from_utf8(&each.name).unwrap());
        println!("{:?}", hex::encode(each.hash));
    }
}

