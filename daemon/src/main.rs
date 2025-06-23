use std::{
    net::TcpListener,
    thread::Builder,
};

mod client;
pub mod receive_pack;

fn main() -> std::io::Result<()> {
    // Listening on port 9418
    let listener = TcpListener::bind("0.0.0.0:9418")?;
    println!("Server listening on port 9418");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                Builder::new()
                    .name(stream.peer_addr().expect("Failed to get address from incoming connection").to_string())
                    .spawn(move || {
                        println!("New connection from: {:?}", stream.peer_addr().expect("Failed to get stream address").to_string());
                        client::handle_client(stream);
                    })
                    .expect("Failed to create new thread");
            }
            Err(e) => {
                eprintln!("Connection failed: {e}");
            }
        }
    }
    Ok(())
}
