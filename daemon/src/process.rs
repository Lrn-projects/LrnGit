use std::{
    ffi::CString,
    fs,
    net::TcpStream,
    os::fd::{AsRawFd, IntoRawFd},
};

use nix::{
    libc::{self, dup2, execvp},
    sys::wait::waitpid,
    unistd::{ForkResult, close, fork},
};

pub fn fork_service(name: &str, arg: &str, socket: TcpStream) {
    let fd = {
        let fd = socket.into_raw_fd();
        fd
    }; // ici, socket est drop
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Continuing execution in parent process, new child has pid: {child}");
            waitpid(child, None).unwrap();
            match fs::read_dir("/proc/self/fd") {
                Ok(it) => it,
                Err(err) => {
                    panic!("{err:?}")
                }
            }
            .for_each(|entry| {
                let entry = entry;
                println!("PARENT still has fd: {:?}", entry.unwrap().path());
            });
        }
        Ok(ForkResult::Child) => {
            // Unsafe to use `println!` (or `unwrap`) here. See Safety.
            if unsafe { dup2(fd, 0) } == -1 {
                panic!("dup2 stdin failed");
            }
            if unsafe { dup2(fd, 1) } == -1 {
                panic!("dup2 stdout failed");
            }
            if unsafe { dup2(fd, 2) } == -1 {
                panic!("dup2 stderr failed");
            }
            close(fd).expect("Failed to close file descriptor");
            let cmd = CString::new(name).unwrap();
            let args = [CString::new(arg).unwrap()];
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
