use std::net::TcpListener;

mod client;

fn main() -> std::io::Result<()> {
    // Listening on port 7878
    let listener = TcpListener::bind("0.0.0.0:9418")?;
    println!("Server listening on port 9418");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                client::handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {e}");
            }
        }
    }
    Ok(())
}
