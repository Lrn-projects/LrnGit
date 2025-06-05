use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    process::exit,
};

use lrncore::logs::error_log;

use crate::object::index;
use crate::{object::commit::parse_commit_by_hash, status, refs::parse_current_branch};

pub fn switch_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        eprintln!("Enter a branch name");
        exit(0);
    }
    if args.len() == 3 {
        switch_ref(&args[2]);
        exit(0);
    }
    match args[2].as_str() {
        "" => {}
        _ => {
            lrncore::logs::warning_log("Unknown command");
            exit(1);
        }
    }
}

/// Switch current head to specified branch
fn switch_ref(branch_name: &str) {
    // Check modified and staged files that are not commit yet
    let files_status = status::get_files_status();
    if !files_status.modified.is_empty() || !files_status.staged.is_empty() {
        println!(
            "error: Your local changes to the following files would be overwritten by checkout:"
        );
        for each in files_status.modified {
            println!("\t{}", each.file);
        }
        for each in files_status.staged {
            println!("\t{}", each.file);
        }
        println!("Please commit your changes or stash them before you switch branches.");
        println!("Aborting");
        exit(1);
    }
    if !fs::exists(format!(".lrngit/refs/heads/{branch_name}")).unwrap() {
        error_log("Branch does not exist");
        exit(1)
    }
    let mut head = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(".lrngit/HEAD")
        .expect("Unable to open file");

    let update_head = format!("ref: refs/heads/{branch_name}");
    head.write_all(update_head.as_bytes())
        .expect("Failed to write buffer in HEAD");
    index::recreate_index();
    update_workdir();
}

/// Update the working directory depending on the ref head
/// Check the root tree from the index and compare with the root tree from the workdir
///
/// Return an error if there's changes not commited (if the root trees are different) and print the
/// modified files not commited (by getting the sorted vector from status)
fn update_workdir() {
    let last_commit = parse_current_branch();
    let _parse_commit = parse_commit_by_hash(&last_commit);
    // let root_tree = parse_commit.tree;
}
