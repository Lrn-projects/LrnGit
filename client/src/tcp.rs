use std::net::TcpStream;

use lrngitcore::remote::parse_local_config_url;

use crate::config::parse_local_config;

/// Connect to remote host and return stream
pub fn tcp_connect_to_remote() -> TcpStream {
    let local_config = parse_local_config();
    let prout = parse_local_config_url(&local_config.remotes.url);
    println!("debug: {:?}", prout);
    TcpStream::connect(local_config.remotes.url).expect("Failed to connect to remote server")
}
//
// pub fn ssh_connect_to_remote() {
//     let _global_config = parse_global_config();
// }
