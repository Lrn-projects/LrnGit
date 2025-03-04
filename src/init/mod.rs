use std::fs;

use crate::utils;

pub fn init_local_repo() {
    let lrngit_directory_name: &str = ".lrngit";
    // create local repository directory
    let create_lrp_dir = fs::create_dir(lrngit_directory_name);
    if let Err(err) = create_lrp_dir {
        lrncore::logs::error_log_with_code("Error initializing local repository", &err.to_string());
    }
    utils::change_wkdir(lrngit_directory_name);
    let create_hook_dir = fs::create_dir_all("hook");
    if let Err(err) = create_hook_dir {
        lrncore::logs::error_log_with_code("Error initializing local repository", &err.to_string());
    }
}
