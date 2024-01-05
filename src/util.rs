use serde_json::Value;

use crate::bspc::{
    node::selection::{NodeDescriptor, NodeModifier, NodeSelector},
    query::QueryCommand, desktop::selection::{DesktopSelector, DesktopDescriptor},
};

pub fn get_class_name_from_id(node_id: &str) -> String {
    let json_tree = QueryCommand::Tree(
        Some(NodeSelector::new().set_descriptor(NodeDescriptor::Id(node_id.to_string()))),
        None,
        None,
    )
    .get_response();

    let json_tree = match json_tree {
        Some(json_tree) => json_tree,
        None => return "Node does not exist".to_string(),
    };

    let json_tree = json_tree.get(0).expect("error");
    let json_value: Value = serde_json::from_str(json_tree).expect("error parsing json");
    if let Some(client) = json_value.get("client") {
        return client["className"].to_string().replace("\"", "");
    }
    "".to_string()
}

pub fn get_last_focused_on_desktop(desktop_id: &str) -> Option<String> {
    let json_tree = QueryCommand::Tree(
        None,
        Some(DesktopSelector::new().set_descriptor(DesktopDescriptor::Id(desktop_id.to_string()))),
        None)
    .get_response()
    .expect("error");

    let json_tree = json_tree.get(0).expect("error getting first element");
    let json_value: Value = serde_json::from_str(json_tree).expect("error parsing json");
    let id = json_value["focusedNodeId"].to_string().replace("\"", "");
    if id == "0" {
        return None;
    }
    Some(format!("{}", id))
}

pub fn get_all_desktops() -> Option<Vec<String>> {
    match QueryCommand::Desktops(None, None, None, false).get_response() {
        Some(response) => {
            if response.len() > 0 {
                return Some(response);
            }
            return None;
        }
        None => None,
    }
}

pub fn get_focused_node() -> Option<String> {
    match QueryCommand::Nodes(
        Some(NodeSelector::new().add_modifier(NodeModifier::Focused)),
        None,
        None,
    ).get_response() {
        Some(response) => {
            if response.len() > 0 {
                return Some(response[0].to_string());
            }
            return None;
        }
        None => None,
    }
}
