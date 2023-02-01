use std::io::prelude::*;
use std::net::TcpStream;

pub fn send(cmd: &str) -> std::io::Result<String> {
    let addr = "127.0.0.1:60080";
    let mut stream = TcpStream::connect(addr)?;

    stream.write(cmd.as_bytes())?;
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}
