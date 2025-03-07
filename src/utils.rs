use std::{env, process::Command};

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
    let new_dir_path = format!(".lrngit/objects/{}", dir);
    Command::new("mkdir")
        .arg(new_dir_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to create all directories");
}

//TODO
// utils function to concatenate element of vec
// vec["src", "main.rs"] -> "src/main.rs"
pub fn concat_elem_vec(vec: Vec<&str>) -> String {
    let mut string: String = "".to_string();
    // let last_elem: &str = "";
    for each in vec {
        if each.contains(".") {
            string += &format!("{}", each);

            return string;
        }
        string += &format!("{}/", each);
    }
    return "".to_string();
}

// and reverse function
