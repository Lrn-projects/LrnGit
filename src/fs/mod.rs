use std::{fs, path::PathBuf};

pub fn delete_path(path: &PathBuf) {
    fs::remove_dir_all(path).expect("Failed to remove path from disk");
}
