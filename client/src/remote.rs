use std::net::TcpStream;

use crate::config::parse_local_config;

/// Connect to remote host and return stream
pub fn connect_to_remote() -> TcpStream {
    let local_config = parse_local_config();
    TcpStream::connect(local_config.remotes.url).expect("Failed to connect to remote server")
}
