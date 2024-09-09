use std::os::unix::net::UnixStream;
use std::io::prelude::*;

pub(crate) fn send_message(command: Vec<String>) -> Option<Vec<String>> {
    //println!("sending message: {:?}", command);

    let command_string = assemble_message(command);

    let mut client = get_client();
    client
        .write_all(command_string.as_bytes())
        .expect("should always be able to write to bspwm socket");

    let mut response = Vec::new();
    client
        .read_to_end(&mut response)
        .expect("should always get a response from bspwm socket");

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

pub(crate) fn assemble_message(command: Vec<String>) -> String {
    format!("{}\x00", command.join("\x00"))
}

pub(crate) fn get_client() -> UnixStream {
    UnixStream::connect(get_bspwm_socket_path()).expect("bspwm socket should always be available")
}

fn get_bspwm_socket_path() -> String {
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
