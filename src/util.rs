use serde_json::Value;

use crate::bspc::{
    desktop::{descriptor::DesktopDescriptor, modifier::DesktopModifier, selector::DesktopSelector}, node::{descriptor::NodeDescriptor, modifier::NodeModifier, selector::NodeSelector}, query::QueryCommand
};

/// Gets the window class name for a given node id.
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

/// Gets the node id of the node that was last focused on the given desktop.
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

/// Queries the ids of every desktop.
pub fn get_all_desktops() -> Option<Vec<String>> {
    match QueryCommand::Desktops(None, None, None, false).get_response() {
        Some(response) => {
            if response.len() > 0 {
                return Some(response);
            }
            return None;
        }
        None => return None,
    };
}
/// Queries the names of every desktop.
pub fn get_all_desktop_names() -> Option<Vec<String>> {
    match QueryCommand::Desktops(None, None, None, true).get_response() {
        Some(response) => {
            if response.len() > 0 {
                return Some(response);
            }
            return None;
        }
        None => return None,
    }
}

/// Queries the currently focused node.
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
        None => return None,
    };
}

/// Gets all node classes for a given list of node ids.
pub fn get_node_classes_from_id_list(nodes: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for id in nodes {
        result.push(get_class_name_from_id(id));
    }
    return result;
}

/// Queries all windows for a given desktop id.
pub fn get_windows(desktop_id: String) -> Vec<String> {
    let mut result = Vec::new();
    match QueryCommand::Nodes(
        Some(NodeSelector::new().add_modifier(NodeModifier::Window)),
        Some(DesktopSelector::new().set_descriptor(DesktopDescriptor::Id(desktop_id))),
        None).get_response() {
        Some(response) => {
            for id in response {
                result.push(id);
            }
            return result;
        }
        None => {},
    }
    return result;
}

/// Queries all empty desktops
pub fn get_empty_desktops() -> Vec<String> {
    let mut result = Vec::new();
    match QueryCommand::Desktops(
        None,
        Some(DesktopSelector::new().add_modifier(DesktopModifier::NotOccupied)),
        None, false).get_response() {
        Some(response) => {
            for id in response {
                result.push(id);
            }
        }
        None => {}
    }
    return result;
}

/// Gets the name of a desktop for a given id.
pub fn get_desktop_name(desktop_id: String) -> String {
    match QueryCommand::Desktops(
        None,
        Some(DesktopSelector::new().set_descriptor(DesktopDescriptor::Id(desktop_id))),
        None, true).get_response() {
        Some(vec) => {
            if vec.len() == 0 {
                return String::from("");
            }
            return vec[0].clone();
        }
        None => return String::from(""),
    }
}

/// Gets the id of the focused desktop.
pub fn get_focused_desktop() -> Option<String> {
    match QueryCommand::Desktops(
        None,
        Some(DesktopSelector::new().set_descriptor(DesktopDescriptor::Focused)),
        None, false).get_response() {
        Some(response) => {
            return Some(response[0].clone());
        }
        None => return None,
    }
}

