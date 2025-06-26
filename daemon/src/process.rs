use std::{
    net::TcpStream,
    os::fd::{AsRawFd, FromRawFd, IntoRawFd},
    process::{Command, Stdio},
};

use nix::unistd::dup;

pub fn fork_service(name: &str, arg: &str, socket: TcpStream) {
    let fd = socket.as_raw_fd();

    use std::os::fd::BorrowedFd;
    let fd_stdin = dup(unsafe { BorrowedFd::borrow_raw(fd) }).expect("Failed to dup fd for stdin");
    let fd_stdout =
        dup(unsafe { BorrowedFd::borrow_raw(fd) }).expect("Failed to dup fd for stdout");
    let fd_stderr =
        dup(unsafe { BorrowedFd::borrow_raw(fd) }).expect("Failed to dup fd for stderr");

    let process = Command::new(name)
        .stdin(unsafe { Stdio::from_raw_fd(fd_stdin.into_raw_fd()) })
        .stdout(unsafe { Stdio::from_raw_fd(fd_stdout.into_raw_fd()) })
        .stderr(unsafe { Stdio::from_raw_fd(fd_stderr.into_raw_fd()) })
        .spawn()
        .expect("Failed to execute asked lrngit-service");
    // let wait_process = process
    //     .wait_with_output()
    //     .expect("Failed to wait asked service");
    std::mem::forget(socket);
}
