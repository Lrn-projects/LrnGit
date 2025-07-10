use std::fs::File;

use crate::fs::write_files;

/// Init the remote origin file with default ref
pub fn init_remote_origin() {
    let origin_path: &str = ".lrngit/refs/remotes/origin/HEAD";
    let init_vec: Vec<u8> = "ref: refs/remotes/origin/main".as_bytes().to_vec();
    let init_slice: &[u8] = &init_vec.as_slice(); 
    write_files(init_slice, origin_path);
}

/// Init the origin head file to easily keep track of current version on the remote server
pub fn init_origin_head() {
    let origin_head_path: &str = ".lrngit/ORIG_HEAD";
    File::create(origin_head_path).expect("Failed to init origin head");
}

pub fn init_origin_main() {
    let path: &str = ".lrngit/refs/remotes/origin/main";
    File::create(path).expect("Failed to init origin main branch");
}
