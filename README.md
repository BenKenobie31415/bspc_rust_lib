# bspc_rust_lib

The goal of this library is to offer a nice way to use as many bspc feature as possible through rust.
This is a project I do because I use it myself but if you have something to contribute or discover an issue, please use the github-repo to contact me.

## Examples

```rust, no_run
use bspc_rust_lib::{bspc::{desktop::{modifier::DesktopModifier, selection::DesktopSelector}, events::Event, node::{command::NodeCommand, modifier::NodeModifier, selector::NodeSelector}, query::QueryCommand}, subscription::SubscriptionHandler};

fn main() {
    let mut sub_handler = SubscriptionHandler::new();

    sub_handler.subscribe(Event::NodeAdd, callback, "added node".to_string());
    sub_handler.subscribe(Event::DesktopFocus, callback2, 73);

    sub_handler.await_threads();
    
    let nodes = QueryCommand::Nodes(
        Some(NodeSelector::new().add_modifier(NodeModifier::Window)),
        None,
        None).get_response();
    match nodes {
        Some(list) => {
            let chosen = list[1].clone();
            let desktop_sel = DesktopSelector::new().add_modifier(DesktopModifier::Name("5".to_string()));
            NodeCommand::ToDesktop(
                DesktopSelector::new().add_modifier(DesktopModifier::Name("5".to_string())),
                true).get_response();
        }
        None => {}
    }
}

fn callback(payload: Vec<&str>, callback_args: &String) {
    let output = QueryCommand::Nodes(
        Some(NodeSelector::new().add_modifier(NodeModifier::Focused)),
        None,
        None).get_response().expect("error");
    println!("focused node: {:?}", output);
    println!("payload: {:?}", payload[Event::NodeAdd.get_payload_count() - 1]);
    println!("callback_args: {:?}", callback_args);
}

fn callback2(payload: Vec<&str>, callback_args: &i32) {
    println!("payload: {:?}", payload);
    println!("callback_args: {:?}", callback_args);
}
```

