use super::node::selection as NodeSelection;
use super::desktop::selection as DesktopSelection;
use super::monitor::selection as MonitorSelection;
use NodeSelection::NodeSelector;
use DesktopSelection::DesktopSelector;
use MonitorSelection::MonitorSelector;

pub enum QueryCommand {
    Nodes(NodeSelector),
    Desktops(DesktopSelector),
    Monitors(MonitorSelector),
    Tree
}

pub fn assemble_query_command(command: QueryCommand) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    result.push(String::from("query"));
    match command {
        QueryCommand::Nodes(selector) => {
            result.push(String::from("--nodes"));
            let asm_selector = NodeSelection::assemble_selector(selector);
            if !asm_selector.is_empty() {
                result.push(String::from("-n"));
                result.push(asm_selector);
            }
        }
        QueryCommand::Desktops(selector) => {
            result.push(String::from("--desktops"));
            let asm_selector = DesktopSelection::assemble_selector(selector);
            if !asm_selector.is_empty() {
                result.push(String::from("-d"));
                result.push(asm_selector);
            }
        }
        QueryCommand::Monitors(selector) => {
            result.push(String::from("--monitors"));
            let asm_selector = MonitorSelection::assemble_selector(selector);
            if !asm_selector.is_empty() {
                result.push(String::from("-m"));
                result.push(asm_selector);
            }
        }
        QueryCommand::Tree => {
            result.push(String::from("--tree"));
        }
    }
    result
}
