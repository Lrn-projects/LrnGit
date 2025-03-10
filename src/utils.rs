use std::{env, fs, io::Read, path::Path, process::Command};

use blob::{Blob, Standard};

pub fn lrngit_usage() -> &'static str {
    let usage = r"
lrngit's cli.


Usage: lrngit command [options]


Commands:
    init            Init a local repository
    add             Add file to local repository
    help            Show this help message

Options:

    -h, --help      Show command usage
    -v, --version   Show the current version of LrnGit
";

    return usage;
}

pub fn change_wkdir(dir: &str) {
    env::set_current_dir(dir).expect("Failed to change directory");
}

//TODO
//Fix when path exist
pub fn add_folder(dir: &str) {
    if Path::new(dir).exists() {
        return;
    }
    let new_dir_path = format!(".lrngit/objects/{}", dir);
    let mut mkdir = Command::new("mkdir")
        .arg(new_dir_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to create all directories");
    let wait_mkdir = mkdir.wait().expect("Failed to wait the mkdir command");
    if !wait_mkdir.success() {
        panic!("Failed ot execute the mkdir command");
    }
}

pub fn read_blob_file() {
    let read_file = fs::File::open(".lrngit/objects/a9/4a8fe5ccb19ba61c4c0873d391e987982fbbd3")
        .expect("Failed to open file");
    println!("file: {:?}", read_file);
}
