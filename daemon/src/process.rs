use std::{
    net::TcpStream,
    os::fd::{FromRawFd, IntoRawFd},
    process::{Command, Stdio},
};

pub fn fork_service(name: &str, arg: &str, socket: TcpStream) {
    let fd = {
        let fd = socket.into_raw_fd();
        fd
    }; // socket drop
    let process = Command::new(name)
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()
        .expect("Failed to execute asked lrngit-service");
    let wait_process = process
        .wait_with_output()
        .expect("Failed to wait asked service");
    if !wait_process.stderr.is_empty() {
        println!("Error in the service asked ?");
    }
}
