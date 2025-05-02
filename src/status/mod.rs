use std::{
    env::{self, current_dir},
    fs, io,
    path::{Path, PathBuf},
    process::exit,
};

use crate::{add, branch, commit, vec_of_path};

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
    println!("debug: {:?}", last_commit);
    let index = add::index::parse_index();
    println!("debug index: {:?}", index);
    let workdir = current_dir().expect("Failed to get the current working directory");
    let mut file_vec: Vec<PathBuf> = Vec::new();
    walkdir(&workdir, &mut file_vec);
    println!("debug file_vec len {:?}", file_vec);
}

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
        // TODO
        // need to sort the entries by all != from avoid_path
        // maybe keep the long version from https://doc.rust-lang.org/std/fs/fn.read_dir.html
        // or sort from this one idk
    }

    Ok(())
}
