/*
Module handling all init command, init a local repository with all folder hierarchy
*/

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

// use crate::utils;

pub fn init_local_repo() {
    // create local repository directory
    let current_dir = env::current_dir();
    let current_repo: PathBuf;
    match current_dir {
        Ok(dir) => current_repo = dir.join(".lrngit"),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to get current directory: {}", e));
            return;
        }
    };
    if Path::new(".lrngit").exists() {
        lrncore::logs::info_log(&format!(
            "Reinitialized existing Git repository in {:?}",
            current_repo
        ));
        let remove_dir = fs::remove_dir_all(".lrngit");
        if let Err(e) = remove_dir {
            lrncore::logs::error_log(&format!(
                "Failed to remove existing .lrngit directory: {}",
                e
            ));
        }
    }
    let mut mkdir = Command::new("mkdir")
        .arg(".lrngit")
        .arg(".lrngit/hooks")
        .arg(".lrngit/info")
        .arg(".lrngit/logs")
        .arg(".lrngit/objects")
        .arg(".lrngit/refs")
        .spawn()
        .expect("Failed to create all directories");
    let wait_mkdir = mkdir.wait().expect("Failed to wait the mkdir command");
    if !wait_mkdir.success() {
        panic!("Failed to execute mkdir command");
    }
    let mut touch = Command::new("touch").arg(".lrngit/index").spawn().expect("Failed to create index file");
    let wait_touch = touch.wait().expect("Failed to wait the touch command");
    if !wait_touch.success() {
        panic!("Failed to execute touch command");
    }
}
