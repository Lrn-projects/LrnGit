use std::{
    env::{self},
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process::exit,
};

pub fn branch_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        show_all_branch();
        exit(0);
    }
    if args.len() == 3 {
        create_new_branch(&args[2]);
        exit(0);
    }
    match args[2].as_str() {
        "" => {}
        _ => {
            lrncore::logs::warning_log("Unknown command");
            exit(1);
        }
    }
}

fn create_new_branch(branch_name: &str) {
    let mut file = File::create(format!(".lrngit/refs/heads/{branch_name}"))
        .expect("Failed to create new branch");
    let last_commit = parse_current_branch();
    file.write_all(last_commit.as_bytes())
        .expect("Failed to write in the new branch file");
}

fn show_all_branch() {
    let current_branch = parse_head();
    let split_current_branch: Vec<&str> = current_branch.split("/").collect();
    let branchdir =
        fs::read_dir(".lrngit/refs/heads/").expect("Failed to get branch directory content");
    for path in branchdir {
        let branch_name = path.unwrap().file_name();
        let branch_name_str = branch_name.to_str().unwrap();
        if branch_name_str == split_current_branch[split_current_branch.len() - 1] {
            println!("*{branch_name_str}");
        } else {
            println!("{branch_name_str}");
        }
    }
}

pub fn init_head() {
    let mut file = File::create(".lrngit/HEAD").expect("Failed to create HEAD file");
    file.write_all("ref: refs/heads/main".as_bytes())
        .expect("Failed to write in HEAD file");
}

/// get content of the HEAD file, ref of the current branch
fn parse_head() -> String {
    let mut head = File::open(".lrngit/HEAD").expect("Failed to open HEAD file");
    let mut content: String = String::new();
    head.read_to_string(&mut content)
        .expect("Failed to read HEAD file content");
    let split_content: Vec<&str> = content.split("ref: ").collect();
    split_content[1].to_string()
}

/// get last commit from the current HEAD
pub fn parse_current_branch() -> String {
    let head = parse_head();
    let branch_string = ".lrngit/".to_string() + &head;
    let branch_path = Path::new(&branch_string);
    if !Path::exists(branch_path) {
        return "".to_string();
    }
    let mut parse_branch = File::open(".lrngit/".to_string() + &head)
        .unwrap_or_else(|_| panic!("Failed to open {head} file"));
    let mut content: String = String::new();
    parse_branch
        .read_to_string(&mut content)
        .expect("Failed to read current branch content");
    content
}

pub fn init_refs(commit_hash: &[u8]) {
    let head_content = parse_head();
    let mut file = File::create(".lrngit/".to_string() + head_content.as_str())
        .expect("Failed to create refs/head file");
    file.write_all(commit_hash)
        .expect("Failed to write commit to branch file");
}
