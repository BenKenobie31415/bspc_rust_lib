//! Package managing subscription to bspwm-events

use std::thread;
use std::io::prelude::*;
use std::thread::JoinHandle;

use crate::{bspc::events::Event, socket_communication::{assemble_message, get_client}};

pub struct SubscriptionHandler {
    thread_handles: Vec<JoinHandle<()>>,
}

impl SubscriptionHandler {
    pub fn new() -> SubscriptionHandler {
        SubscriptionHandler {
            thread_handles: Vec::new(),
        }
    }

    /// `callback` gets called each time the event occurs with the payload of the event and the `callback_args`. After `count` times, `callback` does not get called anymore. 'count' <= 0 is equivalent of infinite count.
    pub fn subscribe<T:Send + 'static>(&mut self, event: Event, callback: fn(Vec<&str>, &T), callback_args: T, count: i32) {
        self.thread_handles.push(add_subscriber(event, callback, callback_args, count));
    }

    /// Blocks and calls callback of subscriptions.
    pub fn await_threads(self) {
        for handle in self.thread_handles {
            handle.join().unwrap();
        }
    }
}

/// Adds the given event to the list of events that the `SubscriptionHandler` listnes for.
/// # Arguments
/// - `event`: The `Event` to listen for
/// - `callback`: The function to execute when the event happens
/// - `callback_args`: Additional arguments passed to the `callback`
/// - `count`: For how many events the callback should be triggered (`count` <= 0 => infinite)
fn add_subscriber<T:Send + 'static>(event: Event, callback: fn(Vec<&str>, &T), callback_args: T, count: i32) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut client = get_client();

        let mut command = vec!["subscribe".to_string(), event.get_str()];
        if count > 0 {
            command.push("--count".to_string());
            command.push(count.to_string());
        }
        let subscription_message = assemble_message(command);

        client
            .write_all(subscription_message.as_bytes())
            .expect("should always be able to write to bspwm socket");

        loop {
            let mut buffer = [0; 4096];
            let bytes_read = client
                .read(&mut buffer)
                .expect("should always be able to read bspwm socket");
            if bytes_read > 0 {
                let data = String::from_utf8_lossy(&buffer[..bytes_read]);
                let payload: Vec<&str> = data.trim_end_matches('\n').split(' ').skip(1).collect();
                callback(payload, &callback_args);
            }
        }
    })
}
