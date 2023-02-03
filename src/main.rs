extern crate rust_msg_server;

use rust_msg_server::server;

fn main() -> std::io::Result<()> {
    server::start()
}
