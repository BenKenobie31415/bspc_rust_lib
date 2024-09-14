use core::panic;

use serde_json::Value;

use crate::bspc::{
    desktop::{descriptor::DesktopDescriptor, modifier::DesktopModifier, selector::DesktopSelector}, node::{command::NodeCommand, descriptor::NodeDescriptor, modifier::NodeModifier, selector::NodeSelector}, query::QueryCommand, selector::Selector
};


// Node Queries
/// Returns id of focused node
pub fn get_focused_node() -> String {
    let nodes = QueryCommand::CNodes(NodeSelector::new().add_modifier(NodeModifier::Focused)).get_response();

    match nodes.len() {
        1 => return nodes[0].clone(),
        _ => panic!("bspwm should always have a focused node"),
    }
}

/// Returns id of the last focused node
pub fn get_last_node() -> String {
    let nodes = QueryCommand::CNodes(
        NodeSelector::new()
            .set_reference_selector(NodeSelector::new().set_descriptor(NodeDescriptor::Focused))
            .set_descriptor(NodeDescriptor::Last)).get_response();

    match nodes.len() {
        1 => return nodes[0].clone(),
        _ => return get_focused_node(),
    }
}

/// Returns ids of all nodes
pub fn get_all_nodes() -> Vec<String> {
    QueryCommand::Nodes(None, None, None).get_response()
}

/// Returns ids of all nodes on desktop
pub fn get_nodes(desktop_id: String) -> Vec<String> {
    QueryCommand::Nodes(
        None,
        Some(DesktopSelector::new().set_descriptor(DesktopDescriptor::Id(desktop_id))),
        None).get_response()
}

/// Returns ids of all windows on desktop
pub fn get_windows(desktop_id: String) -> Vec<String> {
    QueryCommand::Nodes(
        Some(NodeSelector::new().add_modifier(NodeModifier::Window)),
        Some(DesktopSelector::new().set_descriptor(DesktopDescriptor::Id(desktop_id))),
        None).get_response()
}

/// Returns ids of all windows
pub fn get_all_windows() -> Vec<String> {
    QueryCommand::CNodes(NodeSelector::new().add_modifier(NodeModifier::Window)).get_response()
}


// Desktop Queries
/// Returns id of focused desktop
/// # Arguments
/// - `name`: True if name should be returned, false for id
pub fn get_focused_desktop(name: bool) -> String {
    let desktops = QueryCommand::CDesktops(
        DesktopSelector::new().set_descriptor(DesktopDescriptor::Focused),
        name).get_response();

    match desktops.len() {
        1 => return desktops[0].clone(),
        _ => panic!("bspwm should always have a focused desktop"),
    }
}

/// Returns id of last focused desktop
/// # Arguments
/// - `name`: True if name should be returned, false for id
pub fn get_last_desktop(name: bool) -> String {
    let desktops = QueryCommand::CDesktops(
        DesktopSelector::new()
            .set_reference_selector(DesktopSelector::new().set_descriptor(DesktopDescriptor::Focused))
            .set_descriptor(DesktopDescriptor::Last),
        name).get_response();

    match desktops.len() {
        1 => return desktops[0].clone(),
        _ => return get_focused_desktop(name),
    }
}

/// Returns id of last focused node of desktop
pub fn get_last_focused_on_desktop(desktop_id: &str) -> Option<String> {
    let refeference_sel = NodeSelector::new().set_descriptor(NodeDescriptor::Focused);

    let nodes = QueryCommand::Nodes(
        Some(NodeSelector::new().set_reference_selector(refeference_sel).set_descriptor(NodeDescriptor::Last)),
        Some(DesktopSelector::new().set_descriptor(DesktopDescriptor::Id(desktop_id.to_string()))),
        None).get_response();

    match nodes.len() {
        1 => return Some(nodes[0].clone()),
        _ => return None,
    }
}

/// Returns ids of all desktops
pub fn get_all_desktops() -> Vec<String> {
    return QueryCommand::Desktops(None, None, None, false).get_response();
}

/// Returns names of all desktops
pub fn get_all_desktop_names() -> Vec<String> {
    return QueryCommand::Desktops(None, None, None, true).get_response();
}

/// Returns list of all names for the given ids in the same order
pub fn get_node_classes_from_id_list(nodes: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for id in nodes {
        result.push(get_class_name_from_id(id));
    }
    return result;
}

/// Returns x-class name
pub fn get_class_name_from_id(node_id: &str) -> String {
    if !get_all_windows().contains(&node_id.to_string()) {
        return "".to_string();
    }
    let json_tree = QueryCommand::Tree(
        Some(NodeSelector::new().set_descriptor(NodeDescriptor::Id(node_id.to_string()))),
        None,
        None,
    )
        .get_response();

    let json_tree = json_tree.get(0).expect("bspwm should return a valid json object");
    let json_value: Value = serde_json::from_str(json_tree).expect("error parsing json");
    if let Some(client) = json_value.get("client") {
        return client["className"].to_string().replace("\"", "");
    }
    "".to_string()
}

/// Returns ids of all empty desktops
pub fn get_empty_desktops() -> Vec<String> {
    QueryCommand::CDesktops(DesktopSelector::new().add_modifier(DesktopModifier::NotOccupied), false).get_response()
}

/// Returns name of desktop
pub fn get_desktop_name(desktop_id: String) -> String {
    let desktops = QueryCommand::Desktops(
        None,
        Some(DesktopSelector::new().set_descriptor(DesktopDescriptor::Id(desktop_id))),
        None, true).get_response();

    match desktops.len() {
        1 => return desktops[0].clone(),
        _ => return "".to_string(),
    }
}


// Node Commands
/// Moves focused node to the last focused desktop
/// # Arguments
/// - `follow`: True if focus should move to node at new desktop
pub fn move_focused_node_to_last_desktop(follow: bool) {
    NodeCommand::ToDesktop(
        NodeSelector::new(),
        DesktopSelector::new()
            .set_reference_selector(DesktopSelector::new().set_descriptor(DesktopDescriptor::Focused))
            .set_descriptor(DesktopDescriptor::Last),
        follow).get_response();
}

pub fn node_is_on_desktop(node_id: String, desktop_id: String) -> bool {
    QueryCommand::Nodes(None, Some(DesktopSelector::new().set_descriptor(DesktopDescriptor::Id(desktop_id))), None).get_response().contains(&node_id)
}
