use serde_json::Value;

use crate::{bspc::{query::QueryCommand, node::selection::{NodeSelector, NodeDescriptor}}, socket_communication::{send_message, get_bspc_socket_path}};

pub fn get_class_name_from_id(node_id: &str) -> String {
    let command: QueryCommand = QueryCommand::Tree(
        Some(NodeSelector {
            reference_selector: None,
            descriptor: Some(NodeDescriptor::Id(node_id.to_string())),
            modifiers: vec![],
        }), None, None);
    let command = command.assemble();
    let json_tree = send_message(get_bspc_socket_path(), command).expect("error");
    let json_tree = json_tree.get(0).expect("error");
    let json_value: Value = serde_json::from_str(json_tree).expect("error parsing json");
    if let Some(client) = json_value.get("client") {
        return client["className"].to_string().replace("\"", "");
    }
    "".to_string()
}

