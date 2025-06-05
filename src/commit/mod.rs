use std::{
    collections::HashMap,
    env,
    process::exit,
};

use crate::{
    object::index,
    object::commit::create_commit_object,
    object::tree::batch_tree_add,
};

pub fn commit_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        lrncore::usage_exit::usage_and_exit("Invalid command", "use -m flag");
    }
    match args[2].as_str() {
        "-m" => {
            let message = args[3].as_str();
            new_commit(message);
        }
        _ => {
            lrncore::logs::warning_log("Unknown command");
            exit(1);
        }
    }
}

pub fn new_commit(commit_message: &str) {
    let config = index::parse_index();
    let mut root_tree: [u8; 20] = [0; 20];
    // HashMap to store all index entry with blob and tree for batch tree creation
    // Use strings to avoid dropping value and dangling ref
    let mut index_entry_map: HashMap<(String, usize), Vec<(String, u32, [u8; 20])>> =
        HashMap::new();
    // Iterate over the index, each entry contain file path and blob hash
    for each in config.entries {
        let path = &each.path;
        let path_string = String::from_utf8_lossy(path).to_string();
        let mut folder_vec: Vec<&str> = if path_string.contains("/") {
            let folder_split: Vec<&str> = path_string.split("/").collect();
            folder_split
        } else {
            vec![&path_string]
        };
        folder_vec.reverse();
        folder_vec.push("");
        let mut folder_vec_clone = folder_vec.clone();
        // Index iterator to keep track if last element of path is file or folder
        let mut i: usize = 0;
        while !folder_vec_clone.is_empty() {
            let last = folder_vec_clone.remove(0);
            let key = if !folder_vec_clone.is_empty() {
                folder_vec_clone[0]
            } else {
                ""
            };
            // Only insert if last is not empty to avoid empty keys
            if !last.is_empty() {
                // Use reverse index to represent folder hierarchy correctly
                let folder_index = folder_vec.len() - i - 1;
                let entry_vec = index_entry_map
                    .entry((key.to_string(), folder_index))
                    .or_default();
                // Avoid duplicate entries, by checking if entry doesn't exist
                if !entry_vec.iter().any(|(name, _, _)| name == last) {
                    if i != 0 {
                        entry_vec.push((last.to_owned().to_string(), 0o040000, [0u8; 20]));
                    } else {
                        entry_vec.push((last.to_owned().to_string(), 0o100644, each.hash));
                    }
                }
            }
            i += 1;
        }
    }
    batch_tree_add(index_entry_map, &mut root_tree);
    create_commit_object(root_tree, commit_message);
}

