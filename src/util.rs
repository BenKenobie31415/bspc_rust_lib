use serde_json::Value;

use crate::bspc::{query::QueryCommand, node::selection::{NodeSelector, NodeDescriptor}};

pub fn get_class_name_from_id(node_id: &str) -> String {
    let json_tree = QueryCommand::Tree(
        Some(NodeSelector::new().set_descriptor(NodeDescriptor::Id(node_id.to_string()))),
        None, None).get_response().expect("error");

    let json_tree = json_tree.get(0).expect("error");
    let json_value: Value = serde_json::from_str(json_tree).expect("error parsing json");
    if let Some(client) = json_value.get("client") {
        return client["className"].to_string().replace("\"", "");
    }
    "".to_string()
}

