extern crate rusqlite;

use chrono::Utc;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
}

#[derive(Debug)]
pub struct Chat {
    pub id: Option<i32>,
    pub owner_id: i32,
    pub name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug)]
pub struct Message {
    pub id: Option<i32>,
    pub chat_id: i32,
    pub member_id: i32,
    pub text: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
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
            owner_id        INTEGER,
            name            TEXT,
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

pub fn add_user(conn: &Connection, user: &User) -> Result<User> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO user(username, created_at, updated_at) VALUES (?1, ?2, ?3)",
        params![user.username.trim_end(), now, now],
    )?;
    let mut stmt = conn.prepare("SELECT id, username FROM user ORDER BY id DESC LIMIT 1")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
        })
    })?;
    user_iter.last().unwrap()
}

pub fn add_chat(conn: &Connection, chat: &Chat) -> Result<Chat> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO chat(owner_id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        params![chat.owner_id, chat.name, now, now],
    )?;
    let mut stmt = conn.prepare(
        "SELECT id, owner_id, name, created_at, updated_at FROM chat ORDER BY id DESC LIMIT 1",
    )?;
    let chat_iter = stmt.query_map([], |row| {
        Ok(Chat {
            id: row.get(0)?,
            owner_id: row.get(1)?,
            name: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;
    chat_iter.last().unwrap()
}

pub fn add_message(conn: &Connection, message: &Message) -> Result<Message> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO message(chat_id, member_id, text, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![message.chat_id, message.member_id, message.text, now, now],
    )?;
    let mut stmt = conn.prepare(
        "SELECT id, chat_id, member_id, text, created_at, updated_at FROM message ORDER BY id DESC LIMIT 1",
    )?;
    let message_iter = stmt.query_map([], |row| {
        Ok(Message {
            id: row.get(0)?,
            chat_id: row.get(1)?,
            member_id: row.get(2)?,
            text: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    })?;
    message_iter.last().unwrap()
}
