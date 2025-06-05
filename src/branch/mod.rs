use std::{
    env::{self},
    fs::{self, File},
    io::Write,
    process::exit,
};

use crate::refs::{parse_head, parse_current_branch};

pub fn branch_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        show_all_branch();
        exit(0);
    }
    if args.len() == 3 {
        create_new_branch(&args[2]);
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

fn create_new_branch(branch_name: &str) {
    let mut file = File::create(format!(".lrngit/refs/heads/{branch_name}"))
        .expect("Failed to create new branch");
    let last_commit = parse_current_branch();
    file.write_all(last_commit.as_bytes())
        .expect("Failed to write in the new branch file");
}

fn show_all_branch() {
    let current_branch = parse_head();
    let split_current_branch: Vec<&str> = current_branch.split("/").collect();
    let branchdir =
        fs::read_dir(".lrngit/refs/heads/").expect("Failed to get branch directory content");
    for path in branchdir {
        let branch_name = path.unwrap().file_name();
        let branch_name_str = branch_name.to_str().unwrap();
        if branch_name_str == split_current_branch[split_current_branch.len() - 1] {
            println!("*{branch_name_str}");
        } else {
            println!("{branch_name_str}");
        }
    }
}

