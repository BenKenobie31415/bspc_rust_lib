pub mod bspc;
pub mod socket_communication;
pub mod subscription;
pub mod util;

// TODO make commands structs (maybe not sure just an idea)

#[cfg(test)]
mod tests {
    use std::{thread::JoinHandle, collections::HashMap};

    use crate::{
        bspc::{
            node::{command::NodeCommand, descriptor::NodeDescriptor, directions::Direction, modifier::NodeModifier, path::{Path, Jump}, selector::NodeSelector},
            query::QueryCommand, events::Event, desktop::{selection::DesktopSelector, descriptor::DesktopDescriptor},
        },
        socket_communication::{get_bspc_socket_path, send_message}, util::{get_class_name_from_id, get_focused_node, get_last_focused_on_desktop}, subscription::SubscriptionHandler,
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

        sub_handler.subscribe(Event::NodeAdd, callback, "node add".to_string());
        sub_handler.subscribe(Event::NodeRemove, callback, "node remove".to_string());
        sub_handler.subscribe(Event::NodeFocus, callback, "node focus".to_string());
        sub_handler.subscribe(Event::DesktopFocus, callback, "desktop focus".to_string());

        sub_handler.await_threads();
    }

    fn callback(args: Vec<&str>, callback_args: &String) {
        println!("callback: {:?}", args);
        println!("callback_args: {:?}", callback_args);
    }

    #[test]
    fn test3() {
        match get_focused_node() {
            Some(id) => println!("id of focused node: {}", get_class_name_from_id(&id)),
            None => println!("no node focused")
        }
    }

    #[test]
    fn test4() {
        let desktop_id = QueryCommand::Desktops(
            None,
            Some(DesktopSelector::new().set_descriptor(DesktopDescriptor::Name("1".to_string()))), 
            None,
            false).get_response().expect("error");
        let desktop_id = desktop_id.get(0).expect("error");
        let last_selected = get_last_focused_on_desktop(&desktop_id).expect("idk");
        println!("last selected: {}", get_class_name_from_id(&last_selected));
    }

    #[test]
    fn test5() {
        let output = QueryCommand::Nodes(
            Some(NodeSelector::new().set_descriptor(NodeDescriptor::Path(Path::new().add_jump(Jump::Brother)))),
            None,
            None).get_response().expect("error");

        println!("output: {:?}", output);
    }

    #[test]
    fn test6() {
        NodeCommand::PreselectDirection(Direction::North).get_response();
    }
}
