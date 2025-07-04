/*
Module handling all init command, init a local repository with all folder hierarchy
*/

use std::{
    env, fs::{self}, path::{Path, PathBuf}, process::Command
};

use crate::{config, object::index, refs::init_head};

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
        .spawn()
        .expect("Failed to create all directories");
    let wait_mkdir = mkdir.wait().expect("Failed to wait the mkdir command");
    if !wait_mkdir.success() {
        panic!("Failed to execute mkdir command");
    }
    init_head();
    config::init_config_repo();
    index::init_index();
}

