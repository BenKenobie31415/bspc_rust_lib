use crate::{bspc::monitor::selection as MonitorSelection, socket_communication::{send_message, get_bspc_socket_path}};

pub enum MonitorCommand {
    /// Focuses the given monitor.
    /// # Arugments
    /// - `monitor_sel`: The `MonitorSelector` that specifies the monitor to focus
    Focus(MonitorSelection::MonitorSelector),
    /// Swaps the given monitor with the focused monitor.
    /// # Arguments
    /// - `monitor_sel`: The `MonitorSelector` that specifies the monitor to swap with the focused monitor
    Swap(MonitorSelection::MonitorSelector),
    /// Adds desktops to the focused monitor.
    /// # Arguments
    /// - `desktop_names`: The names of the desktops to add
    AddDesktops(Vec<String>),
    /// Reorders desktops of the focused monitor.
    ReorderDesktops(Vec<String>),
    /// Sets the rectangle of the selected monitor.
    Rectangle(u32, u32, u32, u32),
    /// Renames the focused monitor.
    Rename(String),
    /// Removes the focused monitor.
    Remove
}

impl MonitorCommand {
    pub fn get_response(&self) -> Option<Vec<String>> {
        send_message(get_bspc_socket_path(), self.assemble())
    }

    fn assemble(&self) -> Vec<String> {
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
}
