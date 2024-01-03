use crate::bspc::desktop::selection as DesktopSel;
use crate::bspc::monitor::selection as MonitorSel;
use crate::socket_communication::{send_message, get_bspc_socket_path};

use DesktopSel::DesktopSelector;
use MonitorSel::MonitorSelector;

pub enum DesktopCommand {
    Focus(DesktopSelector),
    Activate(DesktopSelector),
    ToMonitor(MonitorSelector, bool),
    Swap(DesktopSelector, bool),
    // TODO layout
    Rename(String),
    // TODO bubble
    Remove
}

impl DesktopCommand {
    pub fn assemble(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        result.push(String::from("desktop"));
        match self {
            DesktopCommand::Focus(desktop_sel) => {
                result.push(String::from("--focus"));
                result.push(desktop_sel.assemble());
            }
            DesktopCommand::Activate(desktop_sel) => {
                result.push(String::from("--activate"));
                result.push(desktop_sel.assemble());
            }
            DesktopCommand::ToMonitor(monitor_sel, follow) => {
                result.push(String::from("--to-monitor"));
                result.push(monitor_sel.assemble());
                if *follow {
                    result.push(String::from("--follow"));
                }
            }
            DesktopCommand::Swap(desktop_sel, follow) => {
                result.push(String::from("--swap"));
                result.push(desktop_sel.assemble());
                if *follow {
                    result.push(String::from("--follow"));
                }
            }
            DesktopCommand::Rename(name) => {
                result.push(String::from("--rename"));
                result.push(name.to_string());
            }
            DesktopCommand::Remove => {
                result.push(String::from("--remove"));
            }
        }
        result
    }

    pub fn get_response(&self) -> Option<Vec<String>> {
        send_message(get_bspc_socket_path(), self.assemble())
    }
}
