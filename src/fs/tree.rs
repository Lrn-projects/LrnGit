use std::{collections::HashMap, fs::File, io::Write};

use crate::utils;
use serde::{Deserialize, Serialize};

/// The `TreeEntry` struct in Rust represents an entry in a tree object with mode, name, and SHA-1 hash.
///
/// Properties:
///
/// * `mode`: The `mode` property in the `TreeEntry` struct represents the file mode or permissions of
///   the entry. It is typically a 32-bit unsigned integer that specifies the filetype and permissions,
///   such as whether the entry is a file, directory, or symbolic link, and the read, write,
///   example: if the mode is `40000` it's a folder, else if it's `100644` it's a blob,
///   160000 would be a commit
/// * `name`: The `name` property in the `TreeEntry` struct represents the name of the entry in the
///   tree. It is of type `String` and stores the name of the entry.
/// * `hash`: The `hash` property in the `TreeEntry` struct is an array of 20 unsigned 8-bit integers
///   (bytes). This array is used to store the SHA-1 hash value of the file or directory represented by
///   the `TreeEntry`. The SHA-1 hash is typically used to
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Ord, Eq)]
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
        header: utils::git_object_header("tree", new_tree_entry_vec.len()),
        entries: new_tree_entry_vec.clone(),
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

/// The `batch_tree_add` function batch create all needed tree from the hashmap
///
/// Arguments:
///
/// * `entity_hashmap`: entity_hashmap is a hashmap containing all entries from index file split
///   and sort in separate tree's.
/// * `hash`: The `hash` parameter represent the hash of the object contained in the new tree
///   object
pub fn batch_tree_add(
    entity_hashmap: HashMap<(String, usize), Vec<(String, u32, [u8; 20])>>,
    root_tree_ptr: &mut [u8; 20],
) {
    let mut entity_vec: Vec<((String, usize), Vec<(String, u32, [u8; 20])>)> = entity_hashmap
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
    entry: ((String, usize), Vec<(String, u32, [u8; 20])>),
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
