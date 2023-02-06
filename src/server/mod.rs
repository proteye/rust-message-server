use crate::constants::MESSAGE_SEPARATOR;

use super::database;
use rusqlite::Connection;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn start() -> std::io::Result<()> {
    let db_conn = database::open();
    if db_conn.is_err() {
        println!("Error database::open(): {}", db_conn.unwrap_err());
        return Ok(());
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
        match stream {
            Ok(stream) => {
                handle_connection(&db_conn, stream);
            }
            Err(e) => {
                println!("Error stream: {}", e);
            }
        }
    }

    let result = db_conn.close();
    println!("Database closed: {}", result.is_ok());
    println!("Server stopped!");
    Ok(())
}

fn handle_connection(conn: &Connection, mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let cmd = &buffer[0];
    let msg = String::from_utf8_lossy(&buffer[1..]);
    let msg = msg.trim().trim_end_matches('\0');

    println!("recv: <{}>:'{}'", cmd, msg);

    stream.write(b"<result>:ok").unwrap();
    stream.flush().unwrap();

    if cmd == &0b0001 {
        let user = database::User {
            id: None,
            username: msg.to_string(),
        };

        let result = database::add_user(conn, &user);
        if result.is_err() {
            println!("Error user adding: {}", result.unwrap_err());
        } else {
            println!("User added: '{:?}'", result);
        }
    } else if cmd == &0b0010 {
        let splitted: Vec<&str> = msg.split(MESSAGE_SEPARATOR).collect();
        if splitted.len() < 2 {
            println!("Error message params: {}", msg);
            return;
        }

        let owner_id = splitted[0].parse();
        if owner_id.is_err() {
            println!("Error chat owner_id: {}", splitted[0]);
            return;
        }

        let chat = database::Chat {
            id: None,
            owner_id: owner_id.unwrap(),
            name: splitted[1].trim().to_string(),
            created_at: None,
            updated_at: None,
        };

        let result = database::add_chat(conn, &chat);
        if result.is_err() {
            println!("Error chat adding: {}", result.unwrap_err());
        } else {
            println!("Chat added: '{:?}'", result);
        }
    } else if cmd == &0b0100 {
        let splitted: Vec<&str> = msg.split(MESSAGE_SEPARATOR).collect();
        if splitted.len() < 3 {
            println!("Error message params: {}", msg);
            return;
        }

        let chat_id = splitted[0].parse();
        if chat_id.is_err() {
            println!("Error message chat_id: {}", splitted[0]);
            return;
        }

        let member_id = splitted[1].parse();
        if member_id.is_err() {
            println!("Error message member_id: {}", splitted[1]);
            return;
        }

        let message = database::Message {
            id: None,
            chat_id: chat_id.unwrap(),
            member_id: member_id.unwrap(),
            text: splitted[2].trim().to_string(),
            created_at: None,
            updated_at: None,
        };

        let result = database::add_message(conn, &message);
        if result.is_err() {
            println!("Error message adding: {}", result.unwrap_err());
        } else {
            println!("Message added: '{:?}'", result);
        }
    }
}
