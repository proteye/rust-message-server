extern crate rusqlite;

use chrono::Utc;
use rusqlite::{params, Connection, Result};

pub struct User {
    id: Option<i32>,
    username: String,
}

pub struct Chat {
    id: Option<i32>,
    name: String,
    owner_id: i32,
    created_at: String,
    updated_at: String,
}

pub struct Message {
    id: Option<i32>,
    chat_id: i32,
    member_id: i32,
    text: String,
    created_at: String,
    updated_at: String,
}

pub fn open() -> Result<Connection> {
    let path = "./resources/rust_msg_server.sqlite3";
    Connection::open(&path)
}

pub fn init(conn: &Connection) -> Result<()> {
    create_table(
        conn,
        "CREATE TABLE user (
            id              INTEGER PRIMARY KEY,
            username        TEXT,
            created_at      TEXT NOT NULL,
            updated_at      TEXT NOT NULL
          )",
    )?;
    create_table(
        conn,
        "CREATE TABLE chat (
            id              INTEGER PRIMARY KEY,
            name            TEXT,
            owner_id        INTEGER,
            created_at      TEXT NOT NULL,
            updated_at      TEXT NOT NULL
          )",
    )?;
    create_table(
        conn,
        "CREATE TABLE message (
            id              INTEGER PRIMARY KEY,
            chat_id         INTEGER,
            member_id       INTEGER,
            text            TEXT,
            created_at      TEXT NOT NULL,
            updated_at      TEXT NOT NULL
          )",
    )?;
    Ok(())
}

fn create_table(conn: &Connection, sql: &str) -> Result<usize> {
    conn.execute(sql, params![])
}

fn add_user(conn: &Connection, user: &User) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO user(username, created_at, updated_at) VALUES (?1, ?2, ?3)",
        params![user.username, now, now],
    )?;
    Ok(())
}

fn add_chat(conn: &Connection, chat: &Chat) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO chat(name, owner_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        params![chat.name, chat.owner_id, now, now],
    )?;
    Ok(())
}

fn add_message(conn: &Connection, message: &Message) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO message(chat_id, member_id, text, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![message.chat_id, message.member_id, message.text, now, now],
    )?;
    Ok(())
}
