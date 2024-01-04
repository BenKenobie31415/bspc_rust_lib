pub mod bspc;
pub mod socket_communication;
pub mod subscription;
pub mod util;

// TODO make commands structs (maybe not sure just an idea)

#[cfg(test)]
mod tests {
    use std::thread::JoinHandle;

    use crate::{
        bspc::{
            node::selection::{NodeModifier, NodeSelector},
            query::QueryCommand, events::Event,
        },
        socket_communication::{get_bspc_socket_path, send_message}, util::get_class_name_from_id, subscription::SubscriptionHandler,
    };

    #[test]
    fn test() {
        let command: QueryCommand = QueryCommand::Nodes(
            Some(NodeSelector {
                reference_selector: None,
                descriptor: None,
                modifiers: vec![NodeModifier::Focused]}), None, None);
        let command = command.assemble();
        let result = send_message(get_bspc_socket_path(), command);
        let node: String;
        match result {
            Some(output) => {
                node = output[0].clone();
            },
            None => {
                node = "".to_string();
            }
        }
        let node_name = get_class_name_from_id(&node);
        println!("node: {:?}", node_name);
    }

    #[test]
    fn test2() {
        let mut sub_handler = SubscriptionHandler::new();

        sub_handler.subscribe(Event::NodeAdd, callback, "node add");
        sub_handler.subscribe(Event::NodeRemove, callback, "node remove");
        sub_handler.subscribe(Event::NodeFocus, callback, "node focus");
        sub_handler.subscribe(Event::DesktopFocus, callback, "desktop focus");

        sub_handler.await_threads();
    }

    fn callback(args: Vec<&str>, callback_args: &str) {
        println!("callback: {:?}", args);
        println!("callback_args: {:?}", callback_args);
    }
}
