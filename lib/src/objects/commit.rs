use serde::{Deserialize, Serialize};

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

/// Unwind commit from given commit to specified commit. Return error if there's missing commits in
/// 
/// Parameters:
/// 'begin': hash of the commit to begin unwinding.
/// 'end': hash of the commit where ending unwinding.
pub fn unwind_commits(begin: &str, end: &str) {

}
