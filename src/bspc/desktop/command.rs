use crate::{bspc::{cycle_direction::CycleDir, monitor::selector::MonitorSelector, selector::Assembleable}, socket_communication};

use super::{descriptor::DesktopDescriptor, layout::Layout, selector::DesktopSelector};

static DEFAULT_DESCRIPTOR: Option<&DesktopDescriptor> = Some(&DesktopDescriptor::Focused);

pub enum DesktopCommand {
    /// Focuses the selected desktop.
    /// # Arguments
    /// - `desktop_sel`: The desktop to focus
    Focus(DesktopSelector),
    /// Activates the selected desktop.
    /// # Arguments
    /// - `desktop_sel`: The desktop to activate
    Activate(DesktopSelector),
    /// Moves the selected desktop to the selected monitor
    /// # Arguments
    /// - `desktop_sel`: The desktop to move
    /// - `monitor_sel`: The monitor to move the desktop to
    /// - `follow`: True if the focus should follow the moved desktop
    ToMonitor(DesktopSelector, MonitorSelector, bool),
    /// Swaps the selected desktops
    /// # Arguments
    /// - `desktop_sel_a`: One of the desktops
    /// - `desktop_sel_b`: The other desktop
    /// - `follow`: True if the focus should remain on the same desktop
    Swap(DesktopSelector, DesktopSelector, bool),
    /// Sets the layout of the selected desktop.
    /// # Arguments
    /// - `desktop_sel`: The desktop to set the layout for
    /// - `layout`: The layout to set
    Layout(DesktopSelector, Layout),
    /// Cycles the layout of the selected desktop.
    /// # Arguments
    /// - `desktop_sel`: The desktop to cycle the layout for
    /// - `cycle_dir`: The direction to cycle in
    Layout2(DesktopSelector, CycleDir),
    /// Renames the selected desktop to the specified name.
    /// # Arguments
    /// - `desktop_sel`: The desktop to rename
    /// - `name`: The new name of the desktop
    Rename(DesktopSelector, String),
    /// Moves the selected desktop in the given direction through the list of desktops
    /// # Arguments
    /// - `desktop_sel`: The desktop to move
    /// - `cycle_dir`: The direction to move the desktop in
    Bubble(DesktopSelector, CycleDir),
    /// Removes the selected desktop.
    /// # Arguments
    /// - `desktop_sel`: The desktop to remove
    Remove(DesktopSelector),
}

impl DesktopCommand {
    pub fn get_response(&self) -> Option<Vec<String>> {
        socket_communication::send_message(self.assemble())
    }

    fn assemble(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        result.push(String::from("desktop"));
        match self {
            DesktopCommand::Focus(desktop_sel) => {
                result.push(desktop_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--focus"));
            }
            DesktopCommand::Activate(desktop_sel) => {
                result.push(desktop_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--activate"));
            }
            DesktopCommand::ToMonitor(desktop_sel, monitor_sel, follow) => {
                result.push(desktop_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--to-monitor"));
                result.push(monitor_sel.assemble(None));
                if *follow {
                    result.push(String::from("--follow"));
                }
            }
            DesktopCommand::Swap(desktop_sel1, desktop_sel2, follow) => {
                result.push(desktop_sel1.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--swap"));
                result.push(desktop_sel2.assemble(None));
                if *follow {
                    result.push(String::from("--follow"));
                }
            }
            DesktopCommand::Layout(desktop_sel, layout) => {
                result.push(desktop_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--layout"));
                result.push(layout.get_string());
            }
            DesktopCommand::Layout2(desktop_sel, cycle_dir) => {
                result.push(desktop_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--layout"));
                result.push(cycle_dir.get_string());
            }
            DesktopCommand::Rename(desktop_sel, name) => {
                result.push(desktop_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--rename"));
                result.push(name.to_string());
            }
            DesktopCommand::Bubble(desktop_sel, cycle_dir) => {
                result.push(desktop_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--bubble"));
                result.push(cycle_dir.get_string());
            }
            DesktopCommand::Remove(desktop_sel) => {
                result.push(desktop_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--remove"));
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::bspc::{cycle_direction::CycleDir, desktop::{command::DesktopCommand, descriptor::DesktopDescriptor, selector::DesktopSelector}, selector::Selector};


    #[test]

    fn focus_fifth() {
        let cmd = DesktopCommand::Focus(DesktopSelector::new().set_descriptor(DesktopDescriptor::Nth(5))).assemble();

        assert_eq!(cmd, vec!["desktop", "^5", "--focus"]);
    }

    #[test]
    fn rename_focused() {
        let cmd = DesktopCommand::Rename(DesktopSelector::new(), "testname".to_string()).assemble();

        assert_eq!(cmd, vec!["desktop", "focused", "--rename", "testname"]);
    }

    #[test]
    fn bubble_next() {
        let cmd = DesktopCommand::Bubble(DesktopSelector::new(), CycleDir::Next).assemble();

        assert_eq!(cmd, vec!["desktop", "focused", "--bubble", "next"]);
    }
}
