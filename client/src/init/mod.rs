/*
Module handling all init command, init a local repository with all folder hierarchy
*/

use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
    process::{exit, Command},
};

use crate::{
    config,
    object::index,
    refs::{
        init_head,
        origin::{init_origin_head, init_origin_main, init_remote_origin},
    },
};

pub fn init_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        init_local_repo();
        exit(0);
    }
    match args[2].as_str() {
        "--bare" => {
            println!("not implemented")
        }
        _ => {
            lrncore::logs::warning_log("Unknown command");
            exit(1);
        }
    }
}

pub fn init_local_repo() {
    // create local repository directory
    let current_dir = env::current_dir();
    let current_repo: PathBuf = match current_dir {
        Ok(dir) => dir.join(".lrngit"),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to get current directory: {e}"));
            return;
        }
    };
    if Path::new(".lrngit").exists() {
        lrncore::logs::info_log(&format!(
            "Reinitialized existing Git repository in {current_repo:?}"
        ));
        let remove_dir = fs::remove_dir_all(".lrngit");
        if let Err(e) = remove_dir {
            lrncore::logs::error_log(&format!("Failed to remove existing .lrngit directory: {e}"));
        }
    }
    let mut mkdir = Command::new("mkdir")
        .arg(".lrngit")
        .arg(".lrngit/hooks")
        .arg(".lrngit/info")
        .arg(".lrngit/logs")
        .arg(".lrngit/objects")
        .arg(".lrngit/refs")
        .arg(".lrngit/refs/heads")
        .arg(".lrngit/refs/tags")
        .arg(".lrngit/refs/remotes")
        .arg(".lrngit/refs/remotes/origin")
        .spawn()
        .expect("Failed to create all directories");
    let wait_mkdir = mkdir.wait().expect("Failed to wait the mkdir command");
    if !wait_mkdir.success() {
        panic!("Failed to execute mkdir command");
    }
    // Init head file
    init_head();
    // Init remote origin dir and files
    init_remote_origin();
    // Init ORIG_HEAD file
    init_origin_head();
    // Init default origin branch(main)
    init_origin_main();
    // Init repository local config
    config::init_config_repo();
    // Init index
    index::init_index();
}
