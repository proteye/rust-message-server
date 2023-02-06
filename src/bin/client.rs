extern crate rust_msg_server;

use rust_msg_server::client;

fn main() {
    // Add user
    let result = client::send("user", "Ramil");
    if result.is_err() {
        println!("Error client::send: {}", result.unwrap_err());
        return;
    }
    println!("recv: {}", result.unwrap());
    // Add chat
    let result = client::send("chat", "1;Common");
    if result.is_err() {
        println!("Error client::send: {}", result.unwrap_err());
        return;
    }
    println!("recv: {}", result.unwrap());
    // Add message
    let result = client::send("message", "1;2;Hello!");
    if result.is_err() {
        println!("Error client::send: {}", result.unwrap_err());
        return;
    }
    println!("recv: {}", result.unwrap());
}
