use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(mut stream: TcpStream) {
    let mut buf = vec![];
    stream.read_to_end(&mut buf).expect("Failed to read");
    stream.write_all(&buf).expect("Failed to write");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:1234")?;

    println!("Listening on 0.0.0.0:1234...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }

    Ok(())
}
