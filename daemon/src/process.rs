use std::{
    net::TcpStream,
    os::fd::{FromRawFd, IntoRawFd, RawFd},
    process::{Command, Stdio},
};

use nix::fcntl::{self, FcntlArg};

fn make_fd_inheritable(fd: RawFd) {
    use std::os::fd::BorrowedFd;
    // SAFETY: fd must be valid and open for the duration of this function
    let borrowed_fd = unsafe { BorrowedFd::borrow_raw(fd) };
    let flags = fcntl::fcntl(borrowed_fd, fcntl::FcntlArg::F_GETFD).unwrap();
    let mut flags = fcntl::FdFlag::from_bits_truncate(flags);
    flags.remove(fcntl::FdFlag::FD_CLOEXEC);
    fcntl::fcntl(borrowed_fd, fcntl::FcntlArg::F_SETFD(flags)).unwrap();
}

pub fn fork_service(name: &str, arg: &str, socket: TcpStream) {
    let fd = {
        let fd = socket.into_raw_fd();
        fd
    }; // socket drop
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
