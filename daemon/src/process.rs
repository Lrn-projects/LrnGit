use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn spawn_service(name: &str, arg: &str) {
    let mut process = Command::new(name)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute asked lrngit-service");
    let mut stdin = process
        .stdin
        .take()
        .expect("Failed to open asked service stdin");
    stdin
        .write_all(arg.as_bytes())
        .expect("Failed to write in asked service stdin");
    drop(stdin);
    let wait = process.wait().expect("Failed to wait the process");
    if !wait.success() {
        panic!("PANIC")
    }
}
