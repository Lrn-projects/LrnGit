use std::{fs, os::unix::fs::PermissionsExt};

pub const SYM: &str = "120000";
pub const DIR: &str = "040000";
pub const EXE: &str = "100755";
pub const RWO: &str = "100644";

pub fn define_tree_mode(path: &str) -> &str {
    let metadata = fs::symlink_metadata(path).expect("Failed to read metadata");
    if metadata.file_type().is_symlink() {
        return SYM; // Symlink
    } else if metadata.file_type().is_dir() {
        return DIR; // Tree (directory)
    } else {
        let perm = metadata.permissions().mode();
        if perm & 0o111 != 0 {
            return EXE; // executable
        } else {
            return RWO; // RW
        }
    }
}
