use std::io::prelude::*;
use std::net::TcpStream;

use crate::helpers::get_cmd_byte_code;

pub fn send(cmd: &str, value: &str) -> std::io::Result<String> {
    let addr = "127.0.0.1:60080";
    let mut stream = TcpStream::connect(addr)?;

    let cmd_byte_code: &[u8] = get_cmd_byte_code(cmd);
    let command_buf = &[cmd_byte_code, value.as_bytes()].concat();

    stream.write(command_buf)?;
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}
