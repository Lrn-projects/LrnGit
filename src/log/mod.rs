use std::{env, io::Read, process::exit};

use crate::{
    branch,
    commit::{self, CommitContent, InitCommitContent},
    utils,
};

pub fn log_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        log_commits();
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

/// log all commits
fn log_commits() {
    let last_commit = branch::parse_current_branch();
    let last_commit_buf = last_commit.as_bytes().to_vec();
    let mut commits_vec: (Vec<CommitContent>, InitCommitContent) = (
        vec![],
        InitCommitContent {
            tree: [0u8; 20],
            author: vec![],
            commiter: vec![],
            message: vec![],
        },
    );
    commits_vec = unwind_commits(last_commit_buf, commits_vec);
    println!("all commits {:?}", commits_vec);
}

fn unwind_commits(
    commit_hash: Vec<u8>,
    mut commits: (Vec<CommitContent>, InitCommitContent),
) -> (Vec<CommitContent>, InitCommitContent) {
    let mut commit_object = utils::get_file_by_hash(
        str::from_utf8(&commit_hash).expect("Failed to convert buffer to str"),
    );
    let mut content_buf: Vec<u8> = Vec::new();
    commit_object
        .read_to_end(&mut content_buf)
        .expect("Failed to read commit content");
    // decode buffer using zlib
    let mut d = flate2::read::ZlibDecoder::new(content_buf.as_slice());
    let mut buffer = Vec::new();
    // read decoded file and populate buffer
    d.read_to_end(&mut buffer).unwrap();

    let parse_commit = commit::parse_commit(buffer.clone());
    let init_commit: InitCommitContent;
    if parse_commit.is_err() {
        init_commit = commit::parse_init_commit(buffer).unwrap();
        commits.1 = init_commit;
        return commits;
    };
    let commit_unwrapped = parse_commit.unwrap();
    commits.0.push(commit_unwrapped.clone());
    unwind_commits(commit_unwrapped.parent.to_vec(), commits)
}
