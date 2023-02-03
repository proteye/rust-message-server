use super::database;
use rusqlite::Connection;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn start() {
    let db_conn = database::open();
    if db_conn.is_err() {
        println!("Error database::open(): {}", db_conn.unwrap_err());
        return;
    }
    let db_conn = db_conn.unwrap();
    let result = database::init(&db_conn);
    if result.is_err() {
        println!("Error database::init: {}", result.unwrap_err());
    } else {
        println!("Database connected: {}", db_conn.is_autocommit());
    }

    let addr = "127.0.0.1:60080";
    let listener = TcpListener::bind(addr).unwrap();
    println!("Server started on {}", addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(&db_conn, stream);
    }

    let result = db_conn.close();
    println!("Database closed: {}", result.is_ok());
    println!("Server stopped!");
}

fn handle_connection(conn: &Connection, mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let cmd = &buffer[0];
    let msg = String::from_utf8_lossy(&buffer[1..]);
    println!("recv: <{}>:'{}'", cmd, msg);

    stream.write(b"<result>:ok").unwrap();
    stream.flush().unwrap();

    if cmd == &0b0001 {
        let username = &msg.trim().trim_end_matches('\0');
        let user = database::User {
            id: None,
            username: username.to_string(),
        };

        let result = database::add_user(conn, &user);
        if result.is_err() {
            println!("Error user adding: {}", result.unwrap_err());
        } else {
            println!("User added: '{:?}'", result);
        }
    }
}
