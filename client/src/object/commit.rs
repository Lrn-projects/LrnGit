use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use std::time::SystemTime;

use chrono::{Local, Offset};
use lrngitcore::fs::new_file_dir;
use lrngitcore::objects::commit::{parse_commit, CommitContent, CommitUser, InitCommitContent};
use lrngitcore::objects::utils::get_file_by_hash;

use crate::config;
use crate::object::utils::{compress_file, git_object_header};
use crate::refs::{init_refs, parse_current_branch};

use super::utils::hash_sha1;

/// Create a new commit object.
/// Get the author and commiter from the git config.
///
///
pub fn create_commit_object(root_tree_hash: [u8; 20], commit_message: &str) {
    let global_config = config::parse_global_config();
    let offset = Local::now().offset().fix().local_minus_utc();
    let sign = if offset >= 0 { "+" } else { "-" };
    let hours = offset.abs() / 3600;
    let minutes = (offset.abs() % 3600) / 60;
    let tz_str = format!("{sign}{hours:02}{minutes:02}");
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
    let parent_commit = parse_current_branch();
    let commit_content_bytes: Vec<u8> = if parent_commit.is_empty() {
        let init_commit_content: InitCommitContent = InitCommitContent {
            tree: root_tree_hash,
            author: commiter_bytes.clone(),
            commiter: commiter_bytes.clone(),
            message: commit_message.as_bytes().to_vec(),
        };
        bincode::serialize(&init_commit_content).expect("Failed to serialize commit content")
    } else {
        let commit_content: CommitContent = CommitContent {
            tree: root_tree_hash,
            // Inline field to copy parent bytes as buffer in a buffer of 20 bytes
            parent: parent_commit.as_bytes().to_vec(),
            author: commiter_bytes.clone(),
            commiter: commiter_bytes,
            message: commit_message.as_bytes().to_vec(),
        };
        bincode::serialize(&commit_content).expect("Failed to serialize commit content")
    };
    let mut commit_bytes: Vec<u8> = Vec::new();
    commit_bytes.extend_from_slice(&git_object_header("commit", commit_content_bytes.len()));
    commit_bytes.extend_from_slice(&commit_content_bytes);
    let commit_bytes_compressed = compress_file(commit_bytes.clone());
    // hash tree content with SHA-1
    let split_hash_result_hex: Vec<char>;
    (_, split_hash_result_hex) = hash_sha1(&commit_bytes_compressed);

    // Create folder and file in local repository
    let mut file: File;
    let file_result = new_file_dir(&split_hash_result_hex);
    match file_result {
        Ok(f) => file = f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Error writing to tree file: {e}"));
            return;
        }
    }
    // write zlib compressed into file
    let file_result = file.write_all(&commit_bytes_compressed);
    match file_result {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Error writing to tree file: {e}"));
            exit(1)
        }
    }
    let commit_hash_string: String = split_hash_result_hex.iter().collect();
    let commit_hash_bytes = commit_hash_string.as_bytes();
    init_refs(commit_hash_bytes);
}

/// Parse the commit object from is hash and return a readable commit object
#[allow(dead_code)]
pub fn parse_commit_by_hash(hash: &str) -> CommitContent {
    let mut commit_object = get_file_by_hash(hash);
    let mut content_buf: Vec<u8> = Vec::new();
    commit_object
        .read_to_end(&mut content_buf)
        .expect("Failed to read commit content");
    // decode buffer using zlib
    let mut d = flate2::read::ZlibDecoder::new(content_buf.as_slice());
    let mut buffer = Vec::new();
    // Read decoded file and populate buffer
    d.read_to_end(&mut buffer).unwrap();

    match parse_commit(buffer) {
        Ok(c) => c,
        Err(e) => {
            lrncore::logs::error_log(&format!("Error parsing commit: {e}"));
            exit(1)
        }
    }
}

