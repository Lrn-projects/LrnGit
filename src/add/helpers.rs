/*
Helper module for the add module, contain useful pub function
*/

use std::{fs, os::unix::fs::PermissionsExt};


pub const SYM: u32 = 0o120000;
pub const DIR: u32 = 0o040000;
pub const EXE: u32 = 0o100755;
pub const RWO: u32 = 0o100644;

/// The function `define_tree_mode` determines the mode of a file (symlink, directory, executable, or
/// read-write).
///
/// Arguments:
///
/// * `path`: The function `define_tree_mode` takes a path as input and determines the mode of the file
/// or directory located at that path. The mode can be one of the following:
///
/// Returns:
///
/// The function `define_tree_mode` is returning a string based on the type of file at the given path.
/// The possible return values are:
/// - "SYM" for symbolic link
/// - "DIR" for directory
/// - "EXE" for executable file
/// - "RWO" for read-write file
pub fn define_tree_mode(path: &str) -> u32 {
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

