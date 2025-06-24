use std::{env, process::exit};

use crate::config::update_remote_url_local_config;

pub fn remote_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        lrncore::usage_exit::usage_and_exit("Invalid command", "use -m flag");
    }
    match args[2].as_str() {
        "add" => {
            let remote = args[3].as_str();
            update_remote_url_local_config(remote);
        }
        _ => {
            lrncore::logs::warning_log("Unknown command");
            exit(1);
        }
    }
}
