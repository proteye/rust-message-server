pub fn get_cmd_byte_code(cmd: &str) -> &[u8] {
    match cmd {
        "user" => &[0b0001],
        "chat" => &[0b0010],
        "message" => &[0b0100],
        _ => &[0b0000],
    }
}
