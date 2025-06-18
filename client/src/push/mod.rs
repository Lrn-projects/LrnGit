use std::{env, io::Write, process::exit};

use crate::{refs::parse_current_branch, remote};

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
    let mut stream = remote::connect_to_remote();
    let buff = format!("have {:?}", last_commit);
    stream.write_all(buff.as_bytes()).unwrap();
}
