use crate::{bspc::monitor::selection as MonitorSelection, socket_communication::{send_message, get_bspc_socket_path}};

pub enum MonitorCommand {
    Focus(MonitorSelection::MonitorSelector),
    Swap(MonitorSelection::MonitorSelector),
    AddDesktops(Vec<String>),
    ReorderDesktops(Vec<String>),
    Rectangle(u32, u32, u32, u32),
    Rename(String),
    Remove
}

impl MonitorCommand {
    pub fn assemble(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        result.push(String::from("monitor"));
        match self {
            MonitorCommand::Focus(selector) => {
                result.push(String::from("--focus"));
                result.push(selector.assemble());
            }
            MonitorCommand::Swap(selector) => {
                result.push(String::from("--swap"));
                result.push(selector.assemble());
            }
            MonitorCommand::AddDesktops(desktops) => {
                result.push(String::from("--add-desktops"));
                result.push(desktops.join(" "));
            }
            MonitorCommand::ReorderDesktops(desktops) => {
                result.push(String::from("--reorder-desktops"));
                result.push(desktops.join(" "));
            }
            MonitorCommand::Rectangle(width, height, x, y) => {
                result.push(String::from("--rectangle"));
                result.push(format!("{}x{}+{}+{}", width, height, x, y));
            }
            MonitorCommand::Rename(name) => {
                result.push(String::from("--rename"));
                result.push(name.to_string());
            }
            MonitorCommand::Remove => {
                result.push(String::from("--remove"));
            }
        }
        result
    }

    pub fn get_response(&self) -> Option<Vec<String>> {
        send_message(get_bspc_socket_path(), self.assemble())
    }
}
