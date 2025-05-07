use crate::status::{FileStatus, FileStatusEntry, FileStatusSort};

/// Sort the file status vector and return two separate vectors containing, 1: All tracked files,
/// 2: All untracked files
pub fn sort_file_status_vec(
    files: Vec<FileStatusEntry>,
) -> FileStatusSort {
    let mut tracked: Vec<FileStatusEntry> = Vec::new();
    let mut untracked: Vec<FileStatusEntry> = Vec::new();
    let mut modify: Vec<FileStatusEntry> = Vec::new();
    let mut delete: Vec<FileStatusEntry> = Vec::new();
    for each in files {
        match each.status {
            FileStatus::Untracked => untracked.push(each),
            FileStatus::Tracked => tracked.push(each),
            FileStatus::Modify => modify.push(each),
            FileStatus::Deleted => delete.push(each),
        }
    }
    let file_status_sort: FileStatusSort = FileStatusSort { untracked: untracked, tracked: tracked, modify: modify, deleted: delete };
    file_status_sort
}

