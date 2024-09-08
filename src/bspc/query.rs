use crate::socket_communication;

use super::{desktop::selector::DesktopSelector, monitor::selector::MonitorSelector, node::selector::NodeSelector, selector::{Assembleable, Selector}};

/// Enum containing the different bspc commands to query information about the state of bspwm.
/// Generally acts like the bspc-query command concerning defaults and other behaviour.
pub enum QueryCommand {
    /// # Arguments
    /// - `node_sel`: The `NodeSelector`
    /// - `desktop_sel`: The `DesktopSelector`
    /// - `monitor_sel`: The `MonitorSelector`
    Nodes(Option<NodeSelector>, Option<DesktopSelector>, Option<MonitorSelector>),
    /// Concise version of the `Nodes` query.
    /// # Arguments
    /// - `node_sel`: The `NodeSelector`
    CNodes(NodeSelector),
    /// # Arguments
    /// - `node_sel`: The `NodeSelector`
    /// - `desktop_sel`: The `DesktopSelector`
    /// - `monitor_sel`: The `MonitorSelector`
    Desktops(Option<NodeSelector>, Option<DesktopSelector>, Option<MonitorSelector>, bool),
    ///Concise version of the `Desktops` query.
    /// # Arguments
    /// - `desktop_sel`: The `DesktopSelector`
    CDesktops(DesktopSelector, bool),
    /// # Arguments
    /// - `node_sel`: The `NodeSelector`
    /// - `desktop_sel`: The `DesktopSelector`
    /// - `monitor_sel`: The `MonitorSelector`
    Monitors(Option<NodeSelector>, Option<DesktopSelector>, Option<MonitorSelector>, bool),
    ///Concise version of the `Monitors` query.
    /// # Arguments
    /// - `monitor_sel`: The `MonitorSelector`
    CMonitors(MonitorSelector, bool),
    Tree(Option<NodeSelector>, Option<DesktopSelector>, Option<MonitorSelector>),
}

impl QueryCommand {
    fn assemble(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        result.push(String::from("query"));
        match self {
            QueryCommand::Nodes(node_sel, desktop_sel, monitor_sel) => {
                result.push(String::from("--nodes"));
                push_selector_optional(&mut result, node_sel);
                push_selector_optional(&mut result, desktop_sel);
                push_selector_optional(&mut result, monitor_sel);
            }
            QueryCommand::CNodes(node_sel) => {
                result.push(String::from("--nodes"));
                push_selector(&mut result, node_sel);
            }
            QueryCommand::Desktops(node_sel, desktop_sel, monitor_sel, names) => {
                result.push(String::from("--desktops"));
                push_selector_optional(&mut result, node_sel);
                push_selector_optional(&mut result, desktop_sel);
                push_selector_optional(&mut result, monitor_sel);
                if *names {
                    result.push(String::from("--names"));
                }
            }
            QueryCommand::CDesktops(desktop_sel, names) => {
                result.push(String::from("--desktops"));
                push_selector(&mut result, desktop_sel);
                if *names {
                    result.push(String::from("--names"));
                }
            }
            QueryCommand::Monitors(node_sel, desktop_sel, monitor_sel, names) => {
                result.push(String::from("--monitors"));
                push_selector_optional(&mut result, node_sel);
                push_selector_optional(&mut result, desktop_sel);
                push_selector_optional(&mut result, monitor_sel);
                if *names {
                    result.push(String::from("--names"));
                }
            }
            QueryCommand::CMonitors(monitor_sel, names) => {
                result.push(String::from("--monitors"));
                push_selector(&mut result, monitor_sel);
                if *names {
                    result.push(String::from("--names"));
                }
            }
            QueryCommand::Tree(node_sel, desktop_sel, monitor_sel) => {
                result.push(String::from("--tree"));
                push_selector_optional(&mut result, node_sel);
                push_selector_optional(&mut result, desktop_sel);
                push_selector_optional(&mut result, monitor_sel);
            }
        }
        result
    }

    /// Executes the command and returns the result of the query returned by bspc
    pub fn get_response(&self) -> Vec<String> {
        match socket_communication::send_message(self.assemble()) {
            Some(message) => {
                if message.len() > 0 {
                    return message;
                }
            }
            None => {}
        }
        return Vec::new();
    }
}

fn push_selector<T: Selector + Assembleable>(current_sel: &mut Vec<String>, sel: &T) {
    let assembled_selector = sel.assemble(None);
    if !assembled_selector.is_empty() {
        current_sel.push(sel.get_query_prefix());
        current_sel.push(assembled_selector);
    }
}
fn push_selector_optional<T: Selector + Assembleable>(current_sel: &mut Vec<String>, sel: &Option<T>) {
    match sel {
        Some(selector) => {
            push_selector(current_sel, selector);
        }
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::bspc::{desktop::{modifier::DesktopModifier, selector::DesktopSelector}, monitor::{descriptor::MonitorDescriptor, selector::MonitorSelector}, node::{modifier::NodeModifier, selector::NodeSelector}, query::QueryCommand, selector::Selector};

    #[test]
    fn all_windows() {
        let cmd = QueryCommand::CNodes(NodeSelector::new().add_modifier(NodeModifier::Window)).assemble();

        assert_eq!(cmd, vec!["query", "--nodes", "--node", ".window"]);
    }

    #[test]
    fn occupied_desktop_names() {
        let cmd = QueryCommand::CDesktops(DesktopSelector::new().add_modifier(DesktopModifier::Occupied), true).assemble();

        assert_eq!(cmd, vec!["query", "--desktops", "--desktop", ".occupied", "--names"]);
    }

    #[test]
    fn focused_monitor_name() {
        let cmd = QueryCommand::CMonitors(MonitorSelector::new().set_descriptor(MonitorDescriptor::Focused), true).assemble();

        assert_eq!(cmd, vec!["query", "--monitors", "--monitor", "focused", "--names"]);
    }

    #[test]
    fn primary_monitor_id_if_() {
        let cmd = QueryCommand::Monitors(
            Some(NodeSelector::new().add_modifier(NodeModifier::Window)),
            None,
            Some(MonitorSelector::new().set_descriptor(MonitorDescriptor::Primary)), false).assemble();

        assert_eq!(cmd, vec!["query", "--monitors", "--node", ".window", "--monitor", "primary"]);
    }
}
