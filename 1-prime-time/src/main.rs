use anyhow::{Error, Result};
use primes::is_prime;
use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::io::ErrorKind;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
};

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    method: String,
    number: Number,
}
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    method: String,
    prime: bool,
}

fn read_until_char(stream: &mut TcpStream, char: u8) -> Result<Vec<u8>> {
    let mut value: Vec<u8> = Vec::new();
    let mut buf: Vec<u8> = vec![0; 1];

    loop {
        match stream.read_exact(&mut buf) {
            Ok(_) => (),
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                // If we reach EOF without, we're just done
                break;
            }
            Err(e) => return Err(Error::from(e)),
        }

        if buf[0] == char {
            break;
        }
        value.push(buf[0]);
    }
    Ok(value)
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    loop {
        let raw_value = match read_until_char(&mut stream, 10) {
            Ok(val) => val,
            Err(_) => {
                // TODO: Validate it's UnexpectedEOF!
                stream.write_all("Malformed request".as_bytes())?;
                break;
            }
        };

        // Reached EOF without any data, we're done!
        if raw_value.is_empty() {
            break;
        }

        let request: Request = match serde_json::from_slice(&raw_value) {
            Ok(value) => value,
            Err(_) => {
                stream.write_all("Malformed request".as_bytes())?;
                break;
            }
        };

        println!("Got request: {:?}", request);

        if request.method != "isPrime" {
            stream.write_all("Malformed request".as_bytes())?;
            break;
        }

        let prime = match request.number.as_u64() {
            Some(number) => is_prime(number),
            None => false,
        };

        let response = Response {
            method: "isPrime".to_string(),
            prime,
        };
        println!("Returning {:?}", response);

        stream.write_all(serde_json::to_string(&response)?.as_bytes())?;
        stream.write_all(&[10])?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:1234")?;

    println!("Listening on 0.0.0.0:1234...");
    for stream in listener.incoming() {
        thread::spawn(|| handle_client(stream?));
    }

    Ok(())
}
