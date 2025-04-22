use std::{env, io::Read, process::exit};

use crate::{branch, commit, utils};

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

fn log_commits() {
    let last_commit = branch::parse_current_branch();
    let mut commit_object = utils::get_file_by_hash(&last_commit);
    let mut content_buf: Vec<u8> = Vec::new();
    commit_object
        .read_to_end(&mut content_buf)
        .expect("Failed to read commit content");
    let mut d = flate2::read::ZlibDecoder::new(content_buf.as_slice());
    let mut buffer = Vec::new();
    d.read_to_end(&mut buffer).unwrap();

    let commit = commit::parse_commit(buffer);
    println!("debug: {:?}", String::from_utf8_lossy(&commit.unwrap().parent));
}
