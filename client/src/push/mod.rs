use std::{env, io::Write, process::exit};

use crate::{
    pack::upload::create_upload_pack,
    refs::{parse_current_branch, parse_head},
    remote,
};

pub fn push_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        push_remote_branch();
        exit(0);
    }
    if args.len() == 3 {
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

fn push_remote_branch() {
    let last_commit = parse_current_branch();
    let refs = &parse_head();
    let mut stream = remote::tcp_connect_to_remote();
    let pack = create_upload_pack(refs, last_commit.as_bytes().to_vec());
    for each in pack {
        // Stream each element in upload pack to server
        stream.write_all(&each).unwrap();
    }
}
