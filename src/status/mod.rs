use std::{
    env::{self, current_dir},
    fs, io,
    path::{Path, PathBuf},
    process::exit,
};

#[derive(Debug)]
enum FileStatus {
    Untracked,
    Tracked,
    Modify,
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
    add::{self, index::IndexEntry},
    branch, commit, vec_of_path,
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

fn workdir_status() {
    let parse_head = branch::parse_current_branch();
    let last_commit = commit::parse_commit_by_hash(parse_head);
    let index = add::index::parse_index();
    let index_entries = index.entries;
    let workdir = current_dir().expect("Failed to get the current working directory");
    // vec containing all files path
    let mut file_vec: Vec<PathBuf> = Vec::new();
    walkdir(&workdir, &mut file_vec);
    // fill the file_vec with all files path inside the repository
    let status = check_file_status(index_entries, file_vec, &workdir);
    // sort all file path by status
    let (tracked, untracked, modify) = sort_file_status_vec(status.entries);
    println!("Tracked file:");
    for each in tracked {
        println!("{:?}", each.file);
    }
    println!("\nUntracked file:");
    for each in untracked {
        println!("{:?}", each.file);
    }
    println!("\nModified file:");
    for each in modify {
        println!("{:?}", each.file);
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

fn check_file_status(
    index_entries: Vec<IndexEntry>,
    mut files: Vec<PathBuf>,
    workdir: &Path,
) -> RepositoryStatus {
    let mut files_status_vec: Vec<FileStatusEntry> = Vec::new();
    for entries in index_entries {
        let entry_path_str = str::from_utf8(&entries.path).expect("Failed to parse buffer to str");
        // TODO
        // fix the loop by checking if the element is in files, if it is then add in
        // as tracked, else untracked.
        // check both vector, check if there's one element in both, if not, untracked
        let mut i = 0;
        while i < files.len() {
            let workdir_owned = workdir.to_str().unwrap();
            let files_path_concat = workdir_owned.to_owned() + "/" + entry_path_str;
            if files_path_concat == *files[i].to_str().unwrap() {
                let file_status: FileStatusEntry = FileStatusEntry {
                    file: entry_path_str.to_owned(),
                    status: FileStatus::Tracked,
                };
                files_status_vec.push(file_status);
                files.remove(i);
            } else {
                i += 1;
            }
        }
        for each in &files {
            let file_status: FileStatusEntry = FileStatusEntry {
                file: each.to_str().unwrap().to_owned(),
                status: FileStatus::Untracked,
            };
            files_status_vec.push(file_status);
        }
    }
    files = vec![];
    let repo_status: RepositoryStatus = RepositoryStatus {
        entries: files_status_vec,
    };
    repo_status
}

/// Sort the file status vector and return two separate vectors containing, 1: All tracked files,
/// 2: All untracked files
fn sort_file_status_vec(
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
