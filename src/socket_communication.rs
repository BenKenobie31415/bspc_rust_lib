use std::os::unix::net::UnixStream;
use std::io::prelude::*;

pub(crate) fn send_message(command: Vec<String>) -> Option<Vec<String>> {
    let socket_path = get_bspc_socket_path();
    //println!("sending message: {:?}", command);
    let mut client = UnixStream::connect(socket_path).expect("Failed to connect to socket");

    let command_string = format!("{}\x00", command.join("\x00"));

    client
        .write_all(command_string.as_bytes())
        .expect("Failed to send command");

    let mut response = Vec::new();
    client
        .read_to_end(&mut response)
        .expect("Failed to load response");

    if !response.is_empty() {
        let response_string = String::from_utf8_lossy(&response);
        let lines: Vec<String> = response_string.lines().map(String::from).collect();
        if lines.len() == 1 && lines[0] == "\u{7}" {
            return None;
        }
        Some(lines)
    } else {
        None
    }
}

fn get_bspc_socket_path() -> String {
    let socket_path = String::from("/tmp/bspwm_0_0-socket");
    match std::env::var("BSPWM_SOCKET") {
        Ok(path) => {
            if path != "" {
                return path;
            }
        }
        Err(_) => {}
    }
    socket_path
}
