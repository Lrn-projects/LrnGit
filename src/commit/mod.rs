use std::{env, process::exit, time::SystemTime};

use chrono::{Local, Offset, TimeZone};
use serde::{Deserialize, Serialize};

use crate::{
    add::{self, index},
    config, utils,
};

#[derive(Debug, Serialize, Deserialize)]
struct Commit {
    // "commit <size>\0" in binary
    header: Vec<u8>,
    content: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommitContent {
    tree: [u8; 20],
    author: Vec<u8>,
    commiter: Vec<u8>,
    message: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommitUser {
    name: Vec<u8>,
    email: Vec<u8>,
    timestamp: i64,
    timezone: Vec<u8>,
}

pub fn commit_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        lrncore::usage_exit::usage_and_exit("Invalid command", "use -m flag");
    }
    match args[2].as_str() {
        "-m" => {
            let message = args[3].as_str();
            println!("{}", message);
            new_commit(message);
        }
        _ => {
             lrncore::logs::warning_log("Unknown command");
             exit(1);
         }
    }
}

pub fn new_commit(commit_message: &str) {
    let config = index::parse_index();
    let mut root_tree: [u8; 20] = [0; 20];
    for each in config.entries {
        let path = String::from_utf8_lossy(&each.path);
        let mut folder_vec: Vec<&str> = if path.contains("/") {
            let folder_split: Vec<&str> = path.split("/").collect();
            folder_split
        } else {
            vec![&path]
        };
        let file = folder_vec.pop().unwrap();
        add::recursive_add(
            folder_vec,
            each.hash,
            file.to_string(),
            String::new(),
            &mut root_tree,
        );
    }
    println!("root tree: {:x?}", root_tree);
    create_commit_object(root_tree, commit_message);
}

fn create_commit_object(root_tree_hash: [u8; 20], commit_message: &str) {
    let global_config = config::parse_global_config();
    let offset = Local::now().offset().fix().local_minus_utc();
    let sign = if offset >= 0 { "+" } else { "-" };
    let hours = offset.abs() / 3600;
    let minutes = (offset.abs() % 3600) / 60;
    let tz_str = format!("{}{:02}{:02}", sign, hours, minutes);
    let commiter: CommitUser = CommitUser {
        name: global_config.user.name.as_bytes().to_vec(),
        email: global_config.user.email.as_bytes().to_vec(),
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        timezone: tz_str.as_bytes().to_vec(),
    };
    let commiter_bytes: Vec<u8> = bincode::serialize(&commiter).expect("Failed to serialize CommitUser struct");
    let commit_content: CommitContent = CommitContent {
        tree: root_tree_hash,
        author: commiter_bytes.clone(),
        commiter: commiter_bytes,
        message: commit_message.as_bytes().to_vec(),
    };
    let commit_content_bytes: Vec<u8> = bincode::serialize(&commit_content).expect("Failed to serialize commit content");
    let commit: Commit = Commit {
        header: utils::git_object_header("commit", commit_content_bytes.len()),
        content: commit_content_bytes,
    };
    let commit_bytes: Vec<u8> = bincode::serialize(&commit).expect("Failed to serialize commit");

}
