use std::{
    net::TcpStream,
    os::fd::{AsRawFd, FromRawFd},
    process::{Command, Stdio},
};

pub fn fork_service(name: &str, arg: &str, socket: &TcpStream) {
    let fd = socket.as_raw_fd();
    let mut process = Command::new(name)
        .arg(arg)
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()
        .expect("Failed to execute asked lrngit-service");
    let wait = process.wait().expect("Failed to wait the process");
    if !wait.success() {
        panic!("Process failed to execute")
    }
}
