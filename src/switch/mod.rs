use std::{
    env,
    fs::{self, File, OpenOptions},
    io::Write,
    process::exit,
};

use lrncore::logs::error_log;

pub fn switch_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        eprintln!("Enter a branch name");
        exit(0);
    }
    if args.len() == 3 {
        switch_ref(&args[2]);
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

fn switch_ref(branch_name: &str) {
    if !fs::exists(format!(".lrngit/refs/heads/{}", branch_name)).unwrap() {
        error_log("Branch does not exist");
        exit(1)
    }
    let mut head = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(".lrngit/HEAD")
        .expect("Unable to open file");

    let update_head = format!("ref: refs/heads/{}", branch_name);
    head.write_all(update_head.as_bytes())
        .expect("Failed to write buffer in HEAD");
}
