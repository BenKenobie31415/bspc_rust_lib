use crate::bspc::monitor::selection as MonitorSelection;

pub enum MonitorCommand {
    Focus(MonitorSelection::MonitorSelector),
    Swap(MonitorSelection::MonitorSelector),
    AddDesktops(Vec<String>),
    ReorderDesktops(Vec<String>),
    Rectangle(u32, u32, u32, u32),
    Rename(String),
    Remove
}

pub fn assemble_monitor_command(command: MonitorCommand) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    result.push(String::from("monitor"));
    match command {
        MonitorCommand::Focus(selector) => {
            result.push(String::from("--focus"));
            result.push(MonitorSelection::assemble_selector(selector));
        }
        MonitorCommand::Swap(selector) => {
            result.push(String::from("--swap"));
            result.push(MonitorSelection::assemble_selector(selector));
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
            result.push(name);
        }
        MonitorCommand::Remove => {
            result.push(String::from("--remove"));
        }
    }
    result
}
