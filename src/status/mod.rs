use std::{
    env::{self, current_dir},
    fs, io,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::exit,
};

mod helper;
use helper::sort_file_status_vec;

pub struct FileStatusSort {
    staged: Vec<FileStatusEntry>,
    untracked: Vec<FileStatusEntry>,
    tracked: Vec<FileStatusEntry>,
    modified: Vec<FileStatusEntry>,
    deleted: Vec<FileStatusEntry>,
}

#[derive(Debug)]
enum FileStatus {
    Staged,
    Untracked,
    Tracked,
    Modify,
    Deleted,
}

#[derive(Debug)]
struct RepositoryStatus {
    entries: Vec<FileStatusEntry>,
}

#[derive(Debug)]
struct FileStatusEntry {
    file: String,
    status: FileStatus,
}

use crate::{
    add::{
        self,
        index::{self, IndexEntry},
    },
    branch,
    commit::{parse_commit, parse_commit_by_hash},
    utils, vec_of_path,
};

pub fn status_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        workdir_status();
        exit(0);
    }
    match args[2].as_str() {
        "" => {
            todo!()
        }
        _ => {
            lrncore::logs::warning_log("Unknown command");
            exit(1);
        }
    }
}

// print the repository status, files tracked, untracked and modified
fn workdir_status() {
    let index = add::index::parse_index();
    let index_entries = index.entries;
    let workdir = current_dir().expect("Failed to get the current working directory");
    // Vec containing all files path
    let mut file_vec: Vec<PathBuf> = Vec::new();
    // Fill the file_vec with all files path inside the repository
    let _ = walkdir(&workdir, &mut file_vec);
    // Vector containing all files with their status
    let status = check_file_status(index_entries, file_vec.to_owned(), &workdir);

    // Sort all file path by status
    let sort_files_status = sort_file_status_vec(status.entries);
    println!("Tracked file:");
    for each in sort_files_status.tracked {
        println!("\t{}", each.file);
        let last_commit = branch::parse_current_branch();
        let parse_commit = parse_commit_by_hash(&last_commit);
        let root_tree_file_hash = utils::walk_root_tree_to_file(&hex::encode(parse_commit.tree), &each.file);
        
    }
    println!("\nUntracked file:");
    println!("  (use 'git add <file>...' to update what will be committed)");
    println!("  (use 'git restore <file>...' to discard changes in working directory)");
    for each in sort_files_status.untracked {
        let split: Vec<&str> = each
            .file
            .split(&(workdir.to_str().unwrap().to_owned() + "/"))
            .collect();

        println!("\t{}", split[1]);
    }
    println!("\nModified file:");
    for each in sort_files_status.modified {
        println!("\t{:?} {}", each.status, each.file);
    }
    for each in sort_files_status.deleted {
        println!("\t{:?} {}", each.status, each.file);
    }
}

/// Recursive function to get all files in current workdir
///
/// # Errors
///
/// This function will return an error if the function cannot access a directory.
fn walkdir(workdir: &PathBuf, file_vec: &mut Vec<PathBuf>) -> io::Result<()> {
    let avoid_path_sufx: Vec<&Path> = vec_of_path!(".lrngit", ".git", "target");
    if workdir.is_dir() {
        for entry in fs::read_dir(workdir)? {
            let entry = entry?;
            let path = entry.path();
            // avoid all unwanted path
            if !avoid_path_sufx
                .iter()
                .any(|&suffix| entry.file_name() == suffix)
            {
                if path.is_dir() {
                    if let Err(e) = walkdir(&path, file_vec) {
                        eprintln!("Error walking directory {:?}: {}", path, e);
                    }
                } else if path.is_file() {
                    file_vec.push(path);
                }
            }
        }
    }

    Ok(())
}

/// Create a RepositoryStatus struct containing all files inside the repository with their status.
/// Params:
/// Vec<IndexEntry> Containing all entries of the index file
/// Vec<PathBuf> Containing all files inside the repository
/// &Path Path reference of the current workdir
fn check_file_status(
    index_entries: Vec<IndexEntry>,
    mut files: Vec<PathBuf>,
    workdir: &Path,
) -> RepositoryStatus {
    let mut files_status_vec: Vec<FileStatusEntry> = Vec::new();
    let mut found_index_entries_vec: Vec<IndexEntry> = Vec::new();
    // check if file tracked
    for entries in &index_entries {
        let entry_path_str = str::from_utf8(&entries.path).expect("Failed to parse buffer to str");
        let mut i = 0;
        while i < files.len() {
            let workdir_owned = workdir.to_str().unwrap();
            let files_path_concat = workdir_owned.to_owned() + "/" + entry_path_str;
            if files_path_concat == *files[i].to_str().unwrap() {
                found_index_entries_vec.push(entries.clone());
                let file_status: FileStatusEntry = check_modified_file(&entry_path_str);
                files_status_vec.push(file_status);
                files.remove(i);
            } else {
                i += 1;
            }
        }
    }
    // Check differences between found file in index and disk
    let mut difference = vec![];
    for i in index_entries {
        if !found_index_entries_vec.contains(&i) {
            difference.push(i);
        }
    }
    // For each differences, create a new file_status for a deleted file
    for each in difference {
        let file_status = FileStatusEntry {
            file: str::from_utf8(&each.path).unwrap().to_owned(),
            status: FileStatus::Deleted,
        };
        files_status_vec.push(file_status);
    }
    // All files not tracked is untracked
    for each in files {
        let file_status: FileStatusEntry = FileStatusEntry {
            file: each.to_str().unwrap().to_owned(),
            status: FileStatus::Untracked,
        };
        files_status_vec.push(file_status);
    }

    let repo_status: RepositoryStatus = RepositoryStatus {
        entries: files_status_vec,
    };
    repo_status
}

fn check_modified_file(files_path: &str) -> FileStatusEntry {
    let index = index::parse_index();
    let mut index_entries = index.entries;
    let mut file_status: FileStatusEntry = FileStatusEntry {
        file: "".to_owned(),
        status: FileStatus::Untracked,
    };

    let file_metadata = fs::metadata(files_path).expect("Failed to get file metadata");

    if let Some(pos) = index_entries
        .iter()
        .position(|x| str::from_utf8(&x.path).unwrap() == files_path)
    {
        let entry = index_entries.remove(pos);
        if file_metadata.mtime() as u32 != entry.mtime
            || file_metadata.len() as u32 != entry.file_size
        {
            file_status = FileStatusEntry {
                file: files_path.to_owned(),
                status: FileStatus::Modify,
            };
        } else {
            file_status = FileStatusEntry {
                file: files_path.to_owned(),
                status: FileStatus::Tracked,
            };
        }
    }
    file_status
}
