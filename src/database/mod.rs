extern crate rusqlite;

use rusqlite::{params, Connection, Result};

pub fn open() -> Result<Connection> {
    let path = "./resources/rust_msg_server.db3";
    Connection::open(&path)
}

pub fn init(db: &Connection) -> Result<()> {
    create_table(
        db,
        "CREATE TABLE user (
            id              INTEGER PRIMARY KEY,
            name            TEXT,
            created_at      TEXT NOT NULL,
            updated_at      TEXT NOT NULL
          )",
    )?;
    create_table(
        db,
        "CREATE TABLE chat (
            id              INTEGER PRIMARY KEY,
            name            TEXT,
            owner_id        INTEGER,
            member_id       INTEGER,
            created_at      TEXT NOT NULL,
            updated_at      TEXT NOT NULL
          )",
    )?;
    create_table(
        db,
        "CREATE TABLE message (
            id              INTEGER PRIMARY KEY,
            chat_id         INTEGER,
            text            TEXT,
            created_at      TEXT NOT NULL,
            updated_at      TEXT NOT NULL
          )",
    )?;
    Ok(())
}

fn create_table(db: &Connection, sql: &str) -> Result<usize> {
    db.execute(sql, params![])
}
