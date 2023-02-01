use super::database;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn start() {
    let db = database::open();
    if db.is_err() {
        println!("Error database::open(): {}", db.unwrap_err());
        return;
    }
    let db = db.unwrap();
    let result = database::init(&db);
    if result.is_err() {
        println!("Error database::init: {}", result.unwrap_err());
    }
    println!("Database connected: {}", db.is_autocommit());

    let addr = "127.0.0.1:60080";
    let listener = TcpListener::bind(addr).unwrap();
    println!("Server started on {}", addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    let result = db.close();
    println!("Database closed: {}", result.is_ok());
    println!("Server stopped!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    println!("recv: {}", String::from_utf8_lossy(&buffer));
    stream.write(b"<result>:ok").unwrap();
    stream.flush().unwrap();
}
