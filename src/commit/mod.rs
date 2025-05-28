use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::File,
    io::{Read, Write},
    process::exit,
    time::SystemTime,
};

use chrono::{Local, Offset};
use serde::{Deserialize, Serialize};

use crate::{
    add::{self, index},
    branch, config,
    utils::{self},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    // "commit <size>\0" in binary
    pub header: Vec<u8>,
    pub content: CommitContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitObject {
    pub commit_hash: Vec<u8>,
    pub commit_content: CommitContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitCommitContent {
    pub tree: [u8; 20],
    pub author: Vec<u8>,
    pub commiter: Vec<u8>,
    pub message: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitContent {
    pub tree: [u8; 20],
    pub parent: Vec<u8>,
    pub author: Vec<u8>,
    pub commiter: Vec<u8>,
    pub message: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitUser {
    pub name: Vec<u8>,
    pub email: Vec<u8>,
    pub timestamp: i64,
    pub timezone: Vec<u8>,
}

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
    let mut index_entry_map: HashMap<(String, usize), Vec<(String, [u8; 20])>> = HashMap::new();
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
                if !entry_vec.iter().any(|(name, _)| name == last) {
                    if i != 0 {
                        entry_vec.push((last.to_owned().to_string(), [0u8; 20]));
                    } else {
                        entry_vec.push((last.to_owned().to_string(), each.hash));
                    }
                }
            }
            i += 1;
        }
    }
    add::recursive_add(index_entry_map, &mut root_tree);
    create_commit_object(root_tree, commit_message);
}

fn create_commit_object(root_tree_hash: [u8; 20], commit_message: &str) {
    let global_config = config::parse_global_config();
    let offset = Local::now().offset().fix().local_minus_utc();
    let sign = if offset >= 0 { "+" } else { "-" };
    let hours = offset.abs() / 3600;
    let minutes = (offset.abs() % 3600) / 60;
    let tz_str = format!("{}{:02}{:02}", sign, hours, minutes);
    let commiter: CommitUser = CommitUser {
        name: global_config.user.name.as_bytes().to_vec(),
        email: global_config.user.email.as_bytes().to_vec(),
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        timezone: tz_str.as_bytes().to_vec(),
    };
    let commiter_bytes: Vec<u8> =
        bincode::serialize(&commiter).expect("Failed to serialize CommitUser struct");
    let parent_commit = branch::parse_current_branch();
    let commit_content_bytes: Vec<u8>;
    if parent_commit.is_empty() {
        let init_commit_content: InitCommitContent = InitCommitContent {
            tree: root_tree_hash,
            author: commiter_bytes.clone(),
            commiter: commiter_bytes.clone(),
            message: commit_message.as_bytes().to_vec(),
        };
        commit_content_bytes =
            bincode::serialize(&init_commit_content).expect("Failed to serialize commit content");
    } else {
        let commit_content: CommitContent = CommitContent {
            tree: root_tree_hash,
            // Inline field to copy parent bytes as buffer in a buffer of 20 bytes
            parent: parent_commit.as_bytes().to_vec(),
            author: commiter_bytes.clone(),
            commiter: commiter_bytes,
            message: commit_message.as_bytes().to_vec(),
        };
        commit_content_bytes =
            bincode::serialize(&commit_content).expect("Failed to serialize commit content");
    }
    let mut commit_bytes: Vec<u8> = Vec::new();
    commit_bytes.extend_from_slice(&utils::git_object_header(
        "commit",
        commit_content_bytes.len(),
    ));
    commit_bytes.extend_from_slice(&commit_content_bytes);
    let commit_bytes_compressed = utils::compress_file(commit_bytes.clone());
    // hash tree content with SHA-1
    let split_hash_result_hex: Vec<char>;
    (_, split_hash_result_hex) = utils::hash_sha1(&commit_bytes_compressed);

    // Create folder and file in local repository
    let mut file: File;
    let file_result = utils::new_file_dir(&split_hash_result_hex);
    match file_result {
        Ok(f) => file = f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Error writing to tree file: {}", e));
            return;
        }
    }
    // write zlib compressed into file
    let file_result = file.write_all(&commit_bytes_compressed);
    match file_result {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Error writing to tree file: {}", e));
            exit(1)
        }
    }
    let commit_hash_string: String = split_hash_result_hex.iter().collect();
    let commit_hash_bytes = commit_hash_string.as_bytes();
    branch::init_refs(commit_hash_bytes);
}

#[allow(dead_code)]
pub fn parse_commit_by_hash(hash: &str) -> CommitContent {
    let mut commit_object = utils::get_file_by_hash(&hash);
    let mut content_buf: Vec<u8> = Vec::new();
    commit_object
        .read_to_end(&mut content_buf)
        .expect("Failed to read commit content");
    // decode buffer using zlib
    let mut d = flate2::read::ZlibDecoder::new(content_buf.as_slice());
    let mut buffer = Vec::new();
    // read decoded file and populate buffer
    d.read_to_end(&mut buffer).unwrap();

    match parse_commit(buffer) {
        Ok(c) => c,
        Err(e) => {
            lrncore::logs::error_log(&format!("Error parsing commit: {}", e));
            exit(1)
        }
    }
}

pub fn parse_commit(buf: Vec<u8>) -> Result<CommitContent, Box<dyn Error>> {
    let content = utils::split_object_header(buf);
    let commit: CommitContent = match bincode::deserialize(&content[1]) {
        Ok(c) => c,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    Ok(commit)
}

pub fn parse_init_commit(buf: Vec<u8>) -> Result<InitCommitContent, Box<dyn Error>> {
    let content = utils::split_object_header(buf);
    let init_commit: InitCommitContent =
        bincode::deserialize(&content[1]).expect("Failed to deserialize init commit");
    Ok(init_commit)
}

pub fn parse_commit_author(buf: Vec<u8>) -> CommitUser {
    let commit_user: CommitUser =
        bincode::deserialize(&buf).expect("Failed to deserialize commit user");
    commit_user
}
