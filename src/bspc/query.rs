use crate::socket_communication::{send_message, get_bspc_socket_path};

use super::node::selector as NodeSelection;
use super::desktop::selection as DesktopSelection;
use super::monitor::selection as MonitorSelection;
use NodeSelection::NodeSelector;
use DesktopSelection::DesktopSelector;
use MonitorSelection::MonitorSelector;


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
    CNodes(Option<NodeSelector>),
    /// # Arguments
    /// - `node_sel`: The `NodeSelector`
    /// - `desktop_sel`: The `DesktopSelector`
    /// - `monitor_sel`: The `MonitorSelector`
    Desktops(Option<NodeSelector>, Option<DesktopSelector>, Option<MonitorSelector>, bool),
    ///Concise version of the `Desktops` query.
    /// # Arguments
    /// - `desktop_sel`: The `DesktopSelector`
    CDesktops(Option<DesktopSelector>, bool),
    /// # Arguments
    /// - `node_sel`: The `NodeSelector`
    /// - `desktop_sel`: The `DesktopSelector`
    /// - `monitor_sel`: The `MonitorSelector`
    Monitors(Option<NodeSelector>, Option<DesktopSelector>, Option<MonitorSelector>, bool),
    ///Concise version of the `Monitors` query.
    /// # Arguments
    /// - `monitor_sel`: The `MonitorSelector`
    CMonitors(Option<MonitorSelector>, bool),
    Tree(Option<NodeSelector>, Option<DesktopSelector>, Option<MonitorSelector>),
}

impl QueryCommand {
    fn assemble(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        result.push(String::from("query"));
        match self {
            QueryCommand::Nodes(node_sel, desktop_sel, monitor_sel) => {
                result.push(String::from("--nodes"));
                push_node_selector(&mut result, node_sel);
                push_desktop_selector(&mut result, desktop_sel);
                push_monitor_selector(&mut result, monitor_sel);
            }
            QueryCommand::CNodes(node_sel) => {
                result.push(String::from("--nodes"));
                push_node_selector(&mut result, node_sel);
            }
            QueryCommand::Desktops(node_sel, desktop_sel, monitor_sel, names) => {
                result.push(String::from("--desktops"));
                push_node_selector(&mut result, node_sel);
                push_desktop_selector(&mut result, desktop_sel);
                push_monitor_selector(&mut result, monitor_sel);
                if *names {
                    result.push(String::from("--names"));
                }
            }
            QueryCommand::CDesktops(desktop_sel, names) => {
                result.push(String::from("--desktops"));
                push_desktop_selector(&mut result, desktop_sel);
                if *names {
                    result.push(String::from("--names"));
                }
            }
            QueryCommand::Monitors(node_sel, desktop_sel, monitor_sel, names) => {
                result.push(String::from("--monitors"));
                push_node_selector(&mut result, node_sel);
                push_desktop_selector(&mut result, desktop_sel);
                push_monitor_selector(&mut result, monitor_sel);
                if *names {
                    result.push(String::from("--names"));
                }
            }
            QueryCommand::CMonitors(monitor_sel, names) => {
                result.push(String::from("--monitors"));
                push_monitor_selector(&mut result, monitor_sel);
                if *names {
                    result.push(String::from("--names"));
                }
            }
            QueryCommand::Tree(node_sel, desktop_sel, monitor_sel) => {
                result.push(String::from("--tree"));
                push_node_selector(&mut result, node_sel);
                push_desktop_selector(&mut result, desktop_sel);
                push_monitor_selector(&mut result, monitor_sel);
            }
        }
        result
    }

    /// Executes the command and returns the result of the query returned by bspc
    pub fn get_response(&self) -> Option<Vec<String>> {
        match send_message(get_bspc_socket_path(), self.assemble()) {
            Some(message) => {
                if message.len() > 0 {
                    return Some(message);
                }
                return None;
            },
            None => None
        }
    }
}

fn push_node_selector(curr_selector: &mut Vec<String>, node_sel: &Option<NodeSelector>) {
    match node_sel {
        Some(sel) => {
            let assembled_selector = sel.assemble();
            if !assembled_selector.is_empty() {
                curr_selector.push("-n".to_string());
                curr_selector.push(assembled_selector);
            }
        }
        None => {}
    }
}

fn push_desktop_selector(curr_selector: &mut Vec<String>, desktop_sel: &Option<DesktopSelector>) {
    match desktop_sel {
        Some(sel) => {
            let assembled_selector = sel.assemble();
            if !assembled_selector.is_empty() {
                curr_selector.push("-d".to_string());
                curr_selector.push(assembled_selector);
            }
        }
        None => {}
    }
}

fn push_monitor_selector(curr_selector: &mut Vec<String>, monitor_sel: &Option<MonitorSelector>) {
    match monitor_sel {
        Some(sel) => {
            let assembled_selector = sel.assemble();
            if !assembled_selector.is_empty() {
                curr_selector.push("-m".to_string());
                curr_selector.push(assembled_selector);
            }
        }
        None => {}
    }
}
