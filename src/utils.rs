use std::{env, fs, io::Read, path::Path, process::Command};

use flate2::read::GzDecoder;

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

pub fn add_folder(dir: &str) {
    if dir.is_empty() {
        return;
    }
    if Path::new(&format!(".lrngit/objects/{}", dir)).exists() {
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
        panic!("Failed to execute the mkdir command");
    }
}

pub fn read_blob_file() {
    let mut read_file = fs::File::open(".lrngit/objects/0c/89b40c61c2aae74496bf9566b76afc69ff8b66")
        .expect("Failed to open file");
    let mut buf = Vec::new();
    read_file
        .read_to_end(&mut buf)
        .expect("Failed to read file");
    let mut d = flate2::read::ZlibDecoder::new(buf.as_slice());
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
