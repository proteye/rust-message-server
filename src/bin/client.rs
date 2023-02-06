extern crate rust_msg_server;

use std::env;

use rust_msg_server::client;

fn main() {
    // Arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Error args: minimum 2 arguments");
        return;
    }
    let cmd = &args[1];
    let value = &args[2];
    // Send command
    let result = client::send(cmd, value);
    if result.is_err() {
        println!("Error client::send: {}", result.unwrap_err());
        return;
    }
    println!("recv: {}", result.unwrap());

    // Add user
    // let result = client::send("user", "Ramil");
    // if result.is_err() {
    //     println!("Error client::send: {}", result.unwrap_err());
    //     return;
    // }
    // println!("recv: {}", result.unwrap());
    // Add chat
    // let result = client::send("chat", "1;Common");
    // if result.is_err() {
    //     println!("Error client::send: {}", result.unwrap_err());
    //     return;
    // }
    // println!("recv: {}", result.unwrap());
    // Add message
    // let result = client::send("message", "1;2;Hello!");
    // if result.is_err() {
    //     println!("Error client::send: {}", result.unwrap_err());
    //     return;
    // }
    // println!("recv: {}", result.unwrap());
}
