use std::{io::Write, net::TcpStream, process::exit};

use lrngitcore::remote::parse_local_config_url;

use crate::config::parse_local_config;

/// Connect to remote host and return stream
pub fn tcp_connect_to_remote(service: &str) -> TcpStream {
    if service != "lrngit-receive-pack" || service != "lrngit-upload-pack" {
        exit(1)
    }
    let local_config = parse_local_config();
    let url = parse_local_config_url(&local_config.remotes.url);
    let mut stream = TcpStream::connect(url.url).expect("Failed to connect to remote server");
    let service_string = format!("{} {}", service, url.path);
    let service_bytes: &[u8] = service_string.as_bytes();
    stream
        .write_all(service_bytes)
        .expect("Failed to stream requested service to server");
    stream
}

// pub fn ssh_connect_to_remote() {
//     let _global_config = parse_global_config();
// }
