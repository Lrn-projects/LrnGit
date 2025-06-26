use std::{
    net::TcpStream,
    os::fd::{AsRawFd, FromRawFd, IntoRawFd, RawFd},
    process::{Command, Stdio},
};

use nix::fcntl::{self, FcntlArg, FdFlag};

use std::os::fd::BorrowedFd;

fn make_fd_inheritable(fd: i32) {
    let borrowed_fd = unsafe { BorrowedFd::borrow_raw(fd) };
    let mut flags =
        FdFlag::from_bits_truncate(fcntl::fcntl(borrowed_fd, FcntlArg::F_GETFD).unwrap());
    flags.remove(FdFlag::FD_CLOEXEC);
    fcntl::fcntl(borrowed_fd, FcntlArg::F_SETFD(flags)).unwrap();
}

pub fn fork_service(name: &str, arg: &str, socket: TcpStream) {
    let fd = socket.as_raw_fd();
    make_fd_inheritable(fd);
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
