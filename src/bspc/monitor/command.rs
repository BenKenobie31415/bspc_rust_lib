use crate::socket_communication;

use super::selection::MonitorSelector;

pub enum MonitorCommand {
    /// Focuses the selected monitor.
    /// # Arugments
    /// - `monitor_sel`: The `MonitorSelector` that specifies the monitor to focus
    Focus(MonitorSelector),
    /// Swaps the selected monitor with the focused monitor.
    /// # Arguments
    /// - `monitor_sel`: The `MonitorSelector` that specifies the monitor to swap with the focused monitor
    Swap(MonitorSelector),
    /// Adds desktops to the selected monitor.
    /// # Arguments
    /// - `monitor_sel`: The monitor to add the desktops to
    /// - `desktop_names`: The names of the desktops to add
    AddDesktops(MonitorSelector, Vec<String>),
    /// Reorders desktops of the selected monitor.
    /// # Arguments
    /// - `monitor_sel`: The monitor to reorder the desktops on
    /// - `desktop_names`: The names of the desktops in the order to set
    ReorderDesktops(MonitorSelector, Vec<String>),
    /// Sets the rectangle of the selected monitor.
    /// # Arguments
    /// - `monitor_sel`: The monitor to set the rectangle for
    /// - `width`: The width of the rectangle
    /// - `height`: The height of the rectangle
    /// - `x_pos`: The x-position of the rectangle
    /// - `y_pos`: The y-position of the rectangle
    Rectangle(MonitorSelector, u32, u32, u32, u32),
    /// Renames the selected monitor.
    /// # Arguments
    /// - `monitor_sel`: The monitor to rename
    /// - `name`: The name to set for the monitor
    Rename(MonitorSelector, String),
    /// Removes the selected monitor.
    /// # Arguments
    /// - `monitor_sel`: The monitor to remove
    Remove(MonitorSelector),
}

impl MonitorCommand {
    pub fn get_response(&self) -> Option<Vec<String>> {
        socket_communication::send_message(self.assemble())
    }

    fn assemble(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        result.push(String::from("monitor"));
        match self {
            MonitorCommand::Focus(monitor_sel) => {
                result.push(String::from("--focus"));
                result.push(monitor_sel.assemble());
            }
            MonitorCommand::Swap(monitor_sel) => {
                result.push(String::from("--swap"));
                result.push(monitor_sel.assemble());
            }
            MonitorCommand::AddDesktops(monitor_sel, desktops) => {
                result.push(monitor_sel.assemble());
                result.push(String::from("--add-desktops"));
                result.push(desktops.join(" "));
            }
            MonitorCommand::ReorderDesktops(monitor_sel, desktops) => {
                result.push(monitor_sel.assemble());
                result.push(String::from("--reorder-desktops"));
                result.push(desktops.join(" "));
            }
            MonitorCommand::Rectangle(monitor_sel, width, height, x, y) => {
                result.push(monitor_sel.assemble());
                result.push(String::from("--rectangle"));
                result.push(format!("{}x{}+{}+{}", width, height, x, y));
            }
            MonitorCommand::Rename(monitor_sel, name) => {
                result.push(monitor_sel.assemble());
                result.push(String::from("--rename"));
                result.push(name.to_string());
            }
            MonitorCommand::Remove(monitor_sel) => {
                result.push(monitor_sel.assemble());
                result.push(String::from("--remove"));
            }
        }
        return result;
    }
}
