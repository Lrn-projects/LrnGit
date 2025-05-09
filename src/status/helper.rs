use crate::status::{FileStatus, FileStatusEntry, FileStatusSort};

/// Sort the file status vector and return a FileStatusSort struct
pub fn sort_file_status_vec(
    files: Vec<FileStatusEntry>,
) -> FileStatusSort {
    let mut staged: Vec<FileStatusEntry> = Vec::new();
    let mut tracked: Vec<FileStatusEntry> = Vec::new();
    let mut untracked: Vec<FileStatusEntry> = Vec::new();
    let mut modified: Vec<FileStatusEntry> = Vec::new();
    let mut deleted: Vec<FileStatusEntry> = Vec::new();
    for each in files {
        match each.status {
            FileStatus::Staged => staged.push(each),
            FileStatus::Untracked => untracked.push(each),
            FileStatus::Tracked => tracked.push(each),
            FileStatus::Modify => modified.push(each),
            FileStatus::Deleted => deleted.push(each),
        }
    }
    let file_status_sort: FileStatusSort = FileStatusSort { staged, untracked, tracked, modified, deleted };
    file_status_sort
}

