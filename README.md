# bspc_rust_lib

The goal of this library is to offer a nice way to use as many bspc feature as possible through rust.
This is a project I do because I use bspwm and want to practice rust.
If you have something to contribute or discover an issue, please use the github-repo to contact me.

In general:
The man pages for bspc are a bit lackluster and I couldn't find really good explainations some things. For example, I don't fully understand how the order in which constraints for nodes/desktops/monitors are set in queries affects the result, only that it does.
So again, if I did something dumb and you notice, please let me know.

## Examples

### Subscription Example
```rust, no_run
use bspc_rust_lib::bspc::{events::Event, node::{descriptor::NodeDescriptor, selector::NodeSelector}, query::QueryCommand, selector::Selector, subscription::SubscriptionHandler};

fn main() {
    let mut sub_handler = SubscriptionHandler::new();

    sub_handler.subscribe(Event::NodeAdd, callback, "added node", 0);
    sub_handler.subscribe(Event::DesktopFocus, callback2, 73, 3);

    sub_handler.await_threads();
}

fn callback(payload: Vec<&str>, callback_args: &&str) {
    let prev_node = QueryCommand::CNodes(NodeSelector::new()
        .set_reference_selector(NodeSelector::new().set_descriptor(NodeDescriptor::Focused))
        .set_descriptor(NodeDescriptor::Last)).get_response();

    println!("previously focused node: {:?}", prev_node);
    println!("added node: {}", payload[Event::NodeAdd.get_payload_count() - 1]);
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

This program also runs the callback `callback2` each time a new desktop gets focused for a total of 3 times and passes the `i32` `73` to it.

`callback2`:
1. Prints the complete payload of the `desktop_focus` event ("man bspc" section `Events` -> [<monitor_id>, <desktop_id>]).
2. Prints the argument additionally passed to it (`73`).

### Node Query Example

```rust, no_run
let windows = QueryCommand::Nodes(
    Some(NodeSelector::new().add_modifier(NodeModifier::Window)),
    Some(DesktopSelector::new().set_descriptor(DesktopDescriptor::Focused)),
    None).get_response();
println!("{:?}", util::get_node_classes_from_id_list(&windows));
}
```
This snippet:
1. Queries all nodes that are windows and are on the currently focused desktop.
2. Gets the x-`window_class_names` for all queried nodes and prints them.

The same result is achievable by using the provided utility functions:
```rust, no_run
let focused_desktop = util::get_focused_desktop(false);
let windows = util::get_windows(focused_desktop);
println!("{:?}", util::get_node_classes_from_id_list(&windows));
```

