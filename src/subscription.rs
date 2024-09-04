//! Package managing subscription to bspc-events

use std::thread;
use std::env;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::thread::JoinHandle;

use crate::bspc::events::Event;

/// Struct for managing eventcallbacks
/// # Example
/// ```
/// let subscription_handler = SubscriptionHandler::new();
/// subscription_handler.subscribe(Event::NodeAdd, callback, 73);
/// subscription_handler.await_threads();
///
/// fn callback(payload: Vec<&str>, callback_args: &i32) {
///     println!("Received event payload {:?} together with additional arguments {}", payload, callback_args);
/// }
/// ```
pub struct SubscriptionHandler {
    thread_handles: Vec<JoinHandle<()>>,
}

impl SubscriptionHandler {
    /// Returns a new SubscriptionHandler
    pub fn new() -> SubscriptionHandler {
        SubscriptionHandler {
            thread_handles: Vec::new(),
        }
    }

    /// Tells the SubscriptionHandler to call the function `callback` when the `event` gets triggered by bspc.
    ///
    /// # Arguments
    /// - `event`: The bspc event which should trigger the callback
    /// - `callback`: The function to be called when the bspc event gets fired.
    /// The callback needs to take a `Vec<&str>` as arguments which will be the payload of the event. It also has to take '&T' which will be given the `callback_args`.
    /// - `callback_args`: Additional arguments that can be passed to the callback for more flexibility
    ///
    /// # Panics
    /// Only panics when the bspc socket is not available and/or there are issues writing/reading from it
    ///
    /// # Example
    /// ```
    /// let subscription_handler = SubscriptionHandler::new();
    /// subscription_handler.subscribe(Event::NodeAdd, callback, 73);
    /// subscription_handler.await_threads();
    ///
    /// fn callback(payload: Vec<&str>, callback_args: &i32) {
    ///     println!("Received event payload {:?} together with additional arguments {}", payload, callback_args);
    /// }
    /// ```
    pub fn subscribe<T:Send + 'static>(&mut self, event: Event, callback: fn(Vec<&str>, &T), callback_args: T) {
        self.thread_handles.push(subscribe::<T>(event, callback, callback_args));
    }

    /// Blocks and manages the subscriptions
    pub fn await_threads(self) {
        for handle in self.thread_handles {
            handle.join().unwrap();
        }
    }
}

fn subscribe<T:Send + 'static>(event: Event, callback: fn(Vec<&str>, &T), callback_args: T) -> JoinHandle<()> {
    let handle = thread::spawn(move || {
        add_subscriber::<T>(event, callback, callback_args);
    });
    handle
}

fn add_subscriber<T>(event: Event, callback: fn(Vec<&str>, &T), callback_args: T) {
    let socket_path = match env::var("BSPWM_SOCKET") {
        Ok(val) => val,
        Err(_e) => String::from("/tmp/bspwm_0_0-socket"),
    };
    let mut client = UnixStream::connect(socket_path).expect("should be able to connect to socket");

    let subscribe_message = format!("subscribe\x00{}\x00", event.get_str());
    client
        .write_all(subscribe_message.as_bytes())
        .expect("should be able to write subscription message to the socket");

    loop {
        let mut buffer = [0; 4096];
        let bytes_read = client
            .read(&mut buffer)
            .expect("socket should alway be able to be read");
        if bytes_read > 0 {
            let data = String::from_utf8_lossy(&buffer[..bytes_read]);
            let payload: Vec<&str> = data.trim_end_matches('\n').split(' ').skip(1).collect();
            callback(payload, &callback_args);
        }
    }
}
