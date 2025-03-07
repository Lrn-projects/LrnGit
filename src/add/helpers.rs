use std::{fs, os::unix::fs::PermissionsExt};

pub fn define_tree_mode(path: &str) -> u32 {
    let metadata = fs::symlink_metadata(path).expect("Failed to read metadata");
    println!("{:?}", metadata);
    if metadata.file_type().is_symlink() {
        return 120000; // Symlink
    } else if metadata.file_type().is_dir() {
        return 040000; // Tree (directory)
    } else {
        let perm = metadata.permissions().mode();
        if perm & 0o111 != 0 {
            return 100755; // executable
        } else {
            return 100644; // RW
        }
    }
}
