use crate::bspc::desktop::selection as DesktopSel;
use crate::bspc::monitor::selection as MonitorSel;

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

pub fn assemble_desktop_command(command: DesktopCommand) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    result.push(String::from("desktop"));
    match command {
        DesktopCommand::Focus(selector) => {
            result.push(String::from("--focus"));
            result.push(DesktopSel::assemble_selector(selector));
        }
        DesktopCommand::Activate(selector) => {
            result.push(String::from("--activate"));
            result.push(DesktopSel::assemble_selector(selector));
        }
        DesktopCommand::ToMonitor(selector, follow) => {
            result.push(String::from("--to-monitor"));
            result.push(MonitorSel::assemble_selector(selector));
            if follow {
                result.push(String::from("--follow"));
            }
        }
        DesktopCommand::Swap(selector, follow) => {
            result.push(String::from("--swap"));
            result.push(DesktopSel::assemble_selector(selector));
            if follow {
                result.push(String::from("--follow"));
            }
        }
        DesktopCommand::Rename(name) => {
            result.push(String::from("--rename"));
            result.push(name);
        }
        DesktopCommand::Remove => {
            result.push(String::from("--remove"));
        }
    }
    result
}
