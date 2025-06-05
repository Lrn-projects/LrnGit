use std::{path::Path, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectHeader {
    pub types: Vec<u8>,
    pub size: usize,
}

// create a new folder in objects
pub fn add_folder(dir: &str) {
    if dir.is_empty() {
        return;
    }
    if Path::new(&format!(".lrngit/objects/{dir}")).exists() {
        return;
    }
    let new_dir_path = format!(".lrngit/objects/{dir}");
    let mut mkdir = Command::new("mkdir")
        .arg(new_dir_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to create all directories");
    let wait_mkdir = mkdir.wait().expect("Failed to wait the mkdir command");
    if !wait_mkdir.success() {
        panic!("Failed to execute the mkdir command");
    }
}


