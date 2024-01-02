use std::sync::mpsc;
use std::thread;
use std::env;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;

pub fn subscribe(event: String, callback: fn(Vec<&str>, Vec<&str>), callback_args: Vec<String>) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let callback_args_refs: Vec<&str> = callback_args.iter().map(|s| s.as_str()).collect();
        add_subscriber(&event, callback, callback_args_refs);

        tx.send(()).unwrap();
    });

    rx.recv().unwrap();
}

fn add_subscriber(event: &str, callback: fn(Vec<&str>, Vec<&str>), callback_args: Vec<&str>) {
    let socket_path = match env::var("BSPWM_SOCKET") {
        Ok(val) => val,
        Err(_e) => String::from("/tmp/bspwm_0_0-socket"),
    };
    let mut client = UnixStream::connect(socket_path).expect("Failed to connect to socket");

    let subscribe_message = format!("subscribe\x00{}\x00", event);
    client
        .write_all(subscribe_message.as_bytes())
        .expect("Failed to send subscribe message");

    loop {
        let mut buffer = [0; 4096];
        let bytes_read = client
            .read(&mut buffer)
            .expect("Failed to read from socket");
        if bytes_read > 0 {
            let data = String::from_utf8_lossy(&buffer[..bytes_read]);
            let args: Vec<&str> = data.trim_end_matches('\n').split(' ').skip(1).collect();
            callback(args, callback_args.clone());
        }
    }
}
