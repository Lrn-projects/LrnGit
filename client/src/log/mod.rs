use std::{env, io::Read, process::exit};

use lrngitcore::objects::commit::{parse_commit, parse_commit_author, parse_init_commit, CommitObject, CommitUser, InitCommitContent};
use lrngitcore::objects::utils::get_file_by_hash;

use crate::{
    refs::parse_current_branch, utils
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
    let last_commit = parse_current_branch();
    let last_commit_buf = last_commit.as_bytes().to_vec();
    let commit_vec: Vec<CommitObject> = Vec::new();
    let mut commits_vec: (Vec<CommitObject>, InitCommitContent) = (
        commit_vec,
        InitCommitContent {
            tree: [0u8; 20],
            author: vec![],
            commiter: vec![],
            message: vec![],
        },
    );
    commits_vec = unwind_commits(last_commit_buf, commits_vec);
    let mut author: CommitUser;
    for each in commits_vec.0 {
        println!("commit: {}", str::from_utf8(&each.commit_hash).unwrap());
        author = parse_commit_author(each.commit_content.author);
        println!(
            "author: {} {}",
            String::from_utf8_lossy(&author.name),
            String::from_utf8_lossy(&author.email)
        );
        println!(
            "date: {} {}",
            utils::timestamp_to_datetime(author.timestamp),
            str::from_utf8(&author.timezone).expect("Failed to parse timezone to str")
        );
        println!(
            "\n\t{}",
            String::from_utf8_lossy(&each.commit_content.message)
        );
        println!();
    }
    let init_commit = commits_vec.1;
    author = parse_commit_author(init_commit.author);
    println!("commit: {}", hex::encode(init_commit.tree));
    println!(
        "author: {} {}",
        String::from_utf8_lossy(&author.name),
        String::from_utf8_lossy(&author.email)
    );
    println!(
        "date: {} {}",
        utils::timestamp_to_datetime(author.timestamp),
        str::from_utf8(&author.timezone).expect("Failed to parse timezone to str")
    );
    println!("\n\t{}", String::from_utf8_lossy(&init_commit.message));
}

fn unwind_commits(
    commit_hash: Vec<u8>,
    mut commits: (Vec<CommitObject>, InitCommitContent),
) -> (Vec<CommitObject>, InitCommitContent) {
    let mut commit_object = get_file_by_hash(
        str::from_utf8(&commit_hash).expect("Failed to convert buffer to str"),
    );
    let mut content_buf: Vec<u8> = Vec::new();
    commit_object
        .read_to_end(&mut content_buf)
        .expect("Failed to read commit content");
    // decode buffer using zlib
    let mut d = flate2::read::ZlibDecoder::new(content_buf.as_slice());
    let mut buffer = Vec::new();
    // read decoded file and fill buffer
    d.read_to_end(&mut buffer).unwrap();

    let parse_commit = parse_commit(buffer.clone());
    let init_commit: InitCommitContent;
    if parse_commit.is_err() {
        init_commit = parse_init_commit(buffer).unwrap();
        commits.1 = init_commit;
        return commits;
    };
    let commit_unwrapped = parse_commit.unwrap();
    let new_commit_object: CommitObject = CommitObject {
        commit_hash,
        commit_content: commit_unwrapped.clone(),
    };
    commits.0.push(new_commit_object);
    unwind_commits(commit_unwrapped.parent.to_vec(), commits)
}
