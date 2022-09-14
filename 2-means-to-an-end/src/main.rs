use anyhow::{Error, Result};
use std::io::ErrorKind;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
};

#[derive(Debug)]
struct Message {
    type_: Type,
    a: i32,
    b: i32,
}

impl Message {
    fn new(bytes: &[u8; 9]) -> Result<Self> {
        let type_ = match bytes[0] {
            73 => Type::Insert,
            81 => Type::Query,
            _ => return Err(Error::msg("Unknown message type")),
        };
        let mut a_arr = [0; 4];
        a_arr.copy_from_slice(&bytes[1..5]);
        let a = i32::from_be_bytes(a_arr);

        let mut b_arr = [0; 4];
        b_arr.copy_from_slice(&bytes[5..9]);
        let b = i32::from_be_bytes(b_arr);

        Ok(Message { type_, a, b })
    }
}

#[derive(Debug, PartialEq)]
enum Type {
    Insert,
    Query,
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    loop {
        let mut buf = [0; 9];
        match stream.read_exact(&mut buf) {
            Ok(_) => (),
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                // Uh oh
                break;
            }
            Err(e) => return Err(Error::from(e)),
        }

        let message = Message::new(&buf)?;

        stream.write_all(&[0])?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_insert() {
        let payload = &[73, 0, 0, 48, 57, 0, 0, 0, 101];

        let message = Message::new(payload).unwrap();

        assert_eq!(message.type_, Type::Insert);
    }

    #[test]
    fn test_message_type_query() {
        let payload = &[81, 0, 0, 48, 57, 0, 0, 0, 101];

        let message = Message::new(payload).unwrap();

        assert_eq!(message.type_, Type::Query);
    }

    #[test]
    fn test_message_loads_big_endian_numbers() {
        let payload = &[73, 0, 0, 48, 57, 0, 0, 0, 101];

        let message = Message::new(payload).unwrap();

        assert_eq!(message.a, 12345);
        assert_eq!(message.b, 101);
    }
}
