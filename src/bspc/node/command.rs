use crate::bspc::node::state as NodeState;
use crate::bspc::node::flags as NodeFlag;
use crate::bspc::node::layers as NodeLayer;
use crate::bspc::node::resize_pos as ResizePos;

use crate::bspc::desktop::selection as DesktopSelection;
use crate::bspc::monitor::selection as MonitorSelection;
use crate::socket_communication::get_bspc_socket_path;
use crate::socket_communication::send_message;

use DesktopSelection::DesktopSelector;
use MonitorSelection::MonitorSelector;

use super::directions::Direction;
use super::selector::NodeSelector;

pub enum NodeCommand {
    Focus(NodeSelector),
    Activate(NodeSelector),
    ToDesktop(DesktopSelector, bool),
    ToMonitor(MonitorSelector, bool),
    ToNode(NodeSelector, bool),
    Swap(NodeSelector),
    PreselectDirection(Direction),
    Move(i32, i32),
    Resize(ResizePos::ResizePos, i32, i32),
    Ratio(f64),
    // TODO rotate
    // TODO flip
    Equalize,
    Balance,
    // TODO circulate
    State(NodeState::NodeState),
    Flag(NodeFlag::NodeFlag),
    Layer(NodeLayer::NodeLayer),
    InsertReceptacle,
    Close,
    Kill
}

impl NodeCommand {
    pub fn get_response(&self) -> Option<Vec<String>> {
        println!("sending message: {:?}", self.assemble());
        send_message(get_bspc_socket_path(), self.assemble())
    }

    fn assemble(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        result.push(String::from("node"));
        match self {
            NodeCommand::Focus(node_sel) => {
                result.push(String::from("--focus"));
                result.push(node_sel.assemble());
            }
            NodeCommand::Activate(node_sel) => {
                result.push(String::from("--activate"));
                result.push(node_sel.assemble());
            }
            NodeCommand::ToDesktop(desktop_sel, follow) => {
                result.push(String::from("--to-desktop"));
                result.push(desktop_sel.assemble());
                if *follow {
                    result.push(String::from("--follow"));
                }
            }
            NodeCommand::ToMonitor(monitor_sel, follow) => {
                result.push(String::from("--to-monitor"));
                result.push(monitor_sel.assemble());
                if *follow {
                    result.push(String::from("--follow"));
                }
            }
            NodeCommand::ToNode(node_sel, follow) => {
                result.push(String::from("--to-node"));
                result.push(node_sel.assemble());
                if *follow {
                    result.push(String::from("--follow"));
                }
            }
            NodeCommand::Swap(node_sel) => {
                result.push(String::from("--swap"));
                result.push(node_sel.assemble());
            }
            NodeCommand::PreselectDirection(direction) => {
                result.push(String::from("--presel-dir"));
                result.push(direction.get_string());
            }
            NodeCommand::Move(x, y) => {
                result.push(String::from("--move"));
                result.push(x.to_string());
                result.push(y.to_string());
            }
            NodeCommand::Resize(resize_pos, x, y) => {
                result.push(String::from("--resize"));
                result.push(ResizePos::get_string(resize_pos));
                result.push(x.to_string());
                result.push(y.to_string());
            }
            NodeCommand::Ratio(ratio) => {
                result.push(String::from("--ratio"));
                result.push(ratio.to_string());
            }
            NodeCommand::Equalize => {
                result.push(String::from("--equalize"));
            }
            NodeCommand::Balance => {
                result.push(String::from("--balance"));
            }
            NodeCommand::State(state) => {
                result.push(String::from("--state"));
                result.push(NodeState::get_string(state));
            }
            NodeCommand::Flag(flag) => {
                result.push(String::from("--flag"));
                result.push(NodeFlag::get_string(flag));
            }
            NodeCommand::Layer(layer) => {
                result.push(String::from("--layer"));
                result.push(NodeLayer::get_string(layer));
            }
            NodeCommand::InsertReceptacle => {
                result.push(String::from("--insert-receptacle"));
            }
            NodeCommand::Close => {
                result.push(String::from("--close"));
            }
            NodeCommand::Kill => {
                result.push(String::from("--kill"));
            }
        }
        result
    }
}
