use crate::status::{FileStatus, FileStatusEntry};

/// Sort the file status vector and return two separate vectors containing, 1: All tracked files,
/// 2: All untracked files
pub fn sort_file_status_vec(
    files: Vec<FileStatusEntry>,
) -> (
    Vec<FileStatusEntry>,
    Vec<FileStatusEntry>,
    Vec<FileStatusEntry>,
) {
    let mut tracked: Vec<FileStatusEntry> = Vec::new();
    let mut untracked: Vec<FileStatusEntry> = Vec::new();
    let mut modify: Vec<FileStatusEntry> = Vec::new();
    for each in files {
        match each.status {
            FileStatus::Untracked => untracked.push(each),
            FileStatus::Tracked => tracked.push(each),
            FileStatus::Modify => modify.push(each),
        }
    }
    (tracked, untracked, modify)
}

