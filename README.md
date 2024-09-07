# bspc_rust_lib

The goal of this library is to offer a nice way to use as many bspc feature as possible through rust.
This is a project I do because I use it myself but if you have something to contribute or discover an issue, please use the github-repo to contact me.

## Examples

### Subscription Example
```rust, no_run
use bspc_rust_lib::{bspc::{desktop::{modifier::DesktopModifier, selection::DesktopSelector}, events::Event, node::{command::NodeCommand, modifier::NodeModifier, selector::NodeSelector}, query::QueryCommand}, subscription::SubscriptionHandler};

fn main() {
    let mut sub_handler = SubscriptionHandler::new();

    sub_handler.subscribe(Event::NodeAdd, callback, "added node".to_string());
    sub_handler.subscribe(Event::DesktopFocus, callback2, 73);

    sub_handler.await_threads();
}

fn callback(payload: Vec<&str>, callback_args: &String) {
    let output = QueryCommand::Nodes(
        Some(NodeSelector::new().set_reference_selector(NodeSelector::new().set_descriptor(NodeDescriptor::Focused)).set_descriptor(NodeDescriptor::Older)),
        None,
        None).get_response().expect("bspc should answer if bspwm runs");
    println!("previously focused node: {:?}", output);
    println!("payload: {:?}", payload[Event::NodeAdd.get_payload_count() - 1]);
    println!("callback_args: {:?}", callback_args);
}

fn callback2(payload: Vec<&str>, callback_args: &i32) {
    println!("payload: {:?}", payload);
    println!("callback_args: {:?}", callback_args);
}
```
This program runs the callback `callback` each time a node gets added and passes the string "added node" to it.

`callback`:
1. Queries and prints the previously focused node.
2. Prints the last element of the payload (the id of the newly added node).
3. Prints the argument additionally passed to it ("node added").

This program also runs the callback `callback2` each time a new desktop gets focused and passes the integer 73 to it.

`callback2`:
1. Prints the complete payload of the `desktop_focus` event ("man bspc" section `Events` -> [<monitor_id>, <desktop_id>]).
2. Prints the argument additionally passed to it (73).

### Node Query Example

```rust, no_run
let windows = QueryCommand::Nodes(
    Some(NodeSelector::new().add_modifier(NodeModifier::Window)),
    Some(DesktopSelector::new().set_descriptor(DesktopDescriptor::Focused)),
    None).get_response();
match windows {
    Some(window_ids) => {
        let window_names = get_node_classes_from_id_list(&window_ids);
        println!("{:?}", window_names);
    }
    None => {}
}
```
This snippet:
1. Queries all nodes that are windows and are on the currently focused desktop.
2. Gets the x-`window_class_names` for all queried nodes and prints them.

The same result is achievable by using the provided utility functions:
```rust, no_run
let focused_desktop = get_focused_desktop();
match focused_desktop {
    Some(desktop_id) => {
        let windows = get_windows(desktop_id);
        println!("{:?}", get_node_classes_from_id_list(&windows));
    }
    None => {}
}

```

