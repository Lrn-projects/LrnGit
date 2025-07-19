use std::{error::Error, io::Read};

use serde::{Deserialize, Serialize};

use super::utils::{get_file_by_hash, split_object_header};

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

/// Parse commit from a buffer
pub fn parse_commit(buf: Vec<u8>) -> Result<CommitContent, Box<dyn Error>> {
    let (_, content) = split_object_header(buf);
    let commit: CommitContent = match bincode::deserialize(&content) {
        Ok(c) => c,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    Ok(commit)
}

/// Parse the init commit from buffer
pub fn parse_init_commit(buf: Vec<u8>) -> Result<InitCommitContent, Box<dyn Error>> {
    let (_, content) = split_object_header(buf);
    let init_commit: InitCommitContent =
        bincode::deserialize(&content).expect("Failed to deserialize init commit");
    Ok(init_commit)
}

pub fn parse_commit_author(buf: Vec<u8>) -> CommitUser {
    let commit_user: CommitUser =
        bincode::deserialize(&buf).expect("Failed to deserialize commit user");
    commit_user
}

/// Unwind commit from given commit to specified commit. Return error if there's missing commits in
///
/// Parameters:
/// 'begin': hash of the commit to begin unwinding.
/// 'end': hash of the commit where ending unwinding.
pub fn unwind_commits(begin: &str, end: &str) {
    // Begin unwinding
    let mut commit_object = get_file_by_hash(begin);
    let mut content_buf: Vec<u8> = Vec::new();
    commit_object
        .read_to_end(&mut content_buf)
        .expect("Failed to read commit content");
    // decode buffer using zlib
    let mut d = flate2::read::ZlibDecoder::new(content_buf.as_slice());
    let mut buffer = Vec::new();
    // Read decoded file and fill buffer
    d.read_to_end(&mut buffer).unwrap();

    let parse_commit = parse_commit(buffer.clone());
    let init_commit: InitCommitContent;
    if parse_commit.is_err() {
        init_commit = parse_init_commit(buffer).unwrap();
        println!("init commit: {:?}", init_commit);
        return;
    };
    let commit_unwrapped = parse_commit.unwrap();
    let new_commit_object: CommitObject = CommitObject {
        commit_hash: begin.into(),
        commit_content: commit_unwrapped.clone(),
    };
    println!("new commit object: {:?}", new_commit_object);
    unwind_commits(str::from_utf8(&commit_unwrapped.parent.to_vec()).expect("Failed to cast bytes vector to str"), end)
}
