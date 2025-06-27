use std::{
    os::fd::{FromRawFd, IntoRawFd},
    process::{Command, Stdio},
};

use nix::{unistd::close, unistd::dup};

pub fn fork_service(name: &str, arg: &str, socket: i32) {
    use std::os::fd::BorrowedFd;
    let fd_stdin =
        dup(unsafe { BorrowedFd::borrow_raw(socket) }).expect("Failed to dup fd for stdin");
    let fd_stdout =
        dup(unsafe { BorrowedFd::borrow_raw(socket) }).expect("Failed to dup fd for stdout");
    let process = Command::new(name)
        .arg(arg)
        .stdin(unsafe { Stdio::from_raw_fd(fd_stdin.into_raw_fd()) })
        .stdout(unsafe { Stdio::from_raw_fd(fd_stdout.into_raw_fd()) })
        .spawn()
        .expect("Failed to execute asked lrngit-service");
    process
        .wait_with_output()
        .expect("Failed to wait asked service");
    close(socket).expect("Failed to close fd");
}
