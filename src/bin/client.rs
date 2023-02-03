extern crate rust_msg_server;

use rust_msg_server::client;

fn main() {
    let result = client::send("user", "Ramil");
    if result.is_err() {
        println!("Error client::send: {}", result.unwrap_err());
        return;
    }
    println!("recv: {}", result.unwrap());
}
