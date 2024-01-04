use std::thread;
use std::env;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::thread::JoinHandle;

use crate::bspc::events::Event;

pub struct SubscriptionHandler {
    threads_handles: Vec<JoinHandle<()>>,
}

impl SubscriptionHandler {
    pub fn new() -> SubscriptionHandler {
        SubscriptionHandler {
            threads_handles: Vec::new(),
        }
    }

    pub fn subscribe<T: Copy + Send + 'static>(&mut self, event: Event, callback: fn(Vec<&str>, T), callback_args: T) {
        self.threads_handles.push(subscribe::<T>(event, callback, callback_args));
    }

    pub fn await_threads(self) {
        for handle in self.threads_handles {
            handle.join().unwrap();
        }
    }
}

fn subscribe<T: Copy + Send + 'static>(event: Event, callback: fn(Vec<&str>, T), callback_args: T) -> JoinHandle<()> {
    let handle = thread::spawn(move || {
        add_subscriber::<T>(event, callback, callback_args);
    });
    handle
}

fn add_subscriber<T: Copy>(event: Event, callback: fn(Vec<&str>, T), callback_args: T) {
    let socket_path = match env::var("BSPWM_SOCKET") {
        Ok(val) => val,
        Err(_e) => String::from("/tmp/bspwm_0_0-socket"),
    };
    let mut client = UnixStream::connect(socket_path).expect("Failed to connect to socket");

    let subscribe_message = format!("subscribe\x00{}\x00", event.get_str());
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
            callback(args, callback_args);
        }
    }
}
