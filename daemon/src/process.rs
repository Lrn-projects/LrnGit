use std::{ffi::CString, net::TcpStream, os::fd::AsRawFd};

use nix::{
    libc::{self, execvp},
    sys::wait::waitpid,
    unistd::{ForkResult, fork},
};

pub fn fork_service(name: &str, arg: &str, socket: TcpStream) {
    // let fd = socket.as_raw_fd();
    // let mut process = Command::new(name)
    //     .arg(arg)
    //     .stdin(unsafe { Stdio::from_raw_fd(fd) })
    //     .spawn()
    //     .expect("Failed to execute asked lrngit-service");
    // let wait = process.wait().expect("Failed to wait the process");
    // if !wait.success() {
    //     panic!("Process failed to execute");
    // }
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Continuing execution in parent process, new child has pid: {child}");
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            // Unsafe to use `println!` (or `unwrap`) here. See Safety.
            let cmd = CString::new(name).unwrap();
            let args = [
                CString::new(arg).unwrap(),
            ];
            // Prepare argv: [program, arg1, ..., null]
            let mut c_args: Vec<*const libc::c_char> = args.iter().map(|s| s.as_ptr()).collect();
            c_args.push(std::ptr::null());
            unsafe {
                execvp(cmd.as_ptr(), c_args.as_ptr());
                // If execvp returns, it failed
                panic!("exec failed");
            }
        }
        Err(_) => println!("Fork failed"),
    }
}
