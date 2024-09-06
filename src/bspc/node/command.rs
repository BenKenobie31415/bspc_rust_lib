use crate::{bspc::{desktop::selector::DesktopSelector, monitor::selection::MonitorSelector}, socket_communication};

use super::{direction::Direction, flag::NodeFlag, layer::NodeLayer, resize_pos::ResizePos, selector::NodeSelector, state::NodeState};

/// Commands that act on nodes
pub enum NodeCommand {
    /// Focuses the selected node.
    /// # Arguments
    /// - `node_sel`: The node to focus
    Focus(NodeSelector),
    /// Activates the selected node.
    /// # Arguments
    /// - `node_sel`: The node to activate
    Activate(NodeSelector),
    /// Moves the selected node to the selected desktop.
    /// # Arguments
    /// - `node_sel`: The node to move
    /// - `desktop_sel`: The desktop to move the node to
    /// - `follow`: True if the focus should follow the moved node
    ToDesktop(NodeSelector, DesktopSelector, bool),
    /// Moves the selected node to the selected monitor.
    /// # Arguments
    /// - `node_sel`: The node to move
    /// - `monitor_sel`: The monitor to move the node to
    /// - `follow`: True if the focus should follow the moved node
    ToMonitor(NodeSelector, MonitorSelector, bool),
    /// Moves the first selected node to the second selected node.
    /// # Arguments
    /// - `node_sel_a`: The node to move
    /// - `node_sel_b`: The node to move the first node to
    /// - `follow`: True if the focus should follow the moved node
    ToNode(NodeSelector, NodeSelector, bool),
    /// Swaps the two selected nodes.
    /// # Arguments
    /// - `node_sel_a`: One of the nodes to swap
    /// - `node_sel_b`: The other of the nodes to swap
    Swap(NodeSelector, NodeSelector),
    //TODO cancel
    /// Starts preselection in the selected node in the given direction.
    /// #Arguments
    /// - `node_sel`: The node to split
    /// - `direction`: The direction to preselect in
    PreselectDirection(NodeSelector, Direction),
    /// Moves the selected node by the given amount and direction.
    /// # Arguments
    /// - `node_sel`: The node to move
    /// - `dx`: How many pixels to move the window to the right
    /// - `dy`: How many pixels to move the window downwards
    Move(NodeSelector, i32, i32),
    /// Rezises the selected node from the given position by the given values.
    /// # Arguments
    /// - `node_sel`: The node to resize
    /// - `resize_pos`: The position to start resizing from
    /// - `dx`: How much to resize to the right
    /// - `dx`: How much to resize downwards
    Resize(NodeSelector, ResizePos, i32, i32),
    /// Sets the splitting ratio of the selected node.
    /// # Arguments
    /// - `node_sel`: The node to set the splitting ratio for.
    /// - `ratio`: New splitting ratio (0,1)
    Ratio(NodeSelector, f64),
    // TODO rotate
    // TODO flip
    /// Resets the splitting ratios of the tree rooted at the selected node.
    /// # Arguments
    /// - `node_sel`: The node that is the root of the tree to equalize
    Equalize(NodeSelector),
    /// Adjusts splitting ratios so that all windows in the tree rooted in the selected node occupy the same area.
    /// # Arguments
    /// - `node_sel`: The node that is the root of the tree to balance
    Balance(NodeSelector),
    // TODO circulate
    /// Sets the state of the selected node.
    /// # Arguments
    /// - `node_sel`: The node to set the state for
    /// - `state`: New flag of the focused node
    State(NodeSelector, NodeState),
    /// Sets/toggles flag for the selected node.
    /// # Arguments
    /// - `node_sel`: The node to set/toggle the flag for
    /// - `flag`: The flag to set/toggle
    Flag(NodeSelector, NodeFlag),
    /// Sets the layer for the selected node.
    /// # Arguments
    /// - `node_sel`: The node to set the layer for
    /// - `layer`: The layer to set
    Layer(NodeSelector, NodeLayer),
    InsertReceptacle,
    /// Closes the selected node.
    /// # Arguments
    /// - `node_sel`: The root of the tree to close all windows in
    Close(NodeSelector),
    /// Kills the selected node.
    /// # Arguments
    /// - `node_sel`: The root of the tree to kill all windows in
    Kill(NodeSelector),
}

impl NodeCommand {
    pub fn get_response(&self) -> Option<Vec<String>> {
        socket_communication::send_message(self.assemble())
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
            NodeCommand::ToDesktop(node_sel, desktop_sel, follow) => {
                result.push(node_sel.assemble());
                result.push(String::from("--to-desktop"));
                result.push(desktop_sel.assemble());
                if *follow {
                    result.push(String::from("--follow"));
                }
            }
            NodeCommand::ToMonitor(node_sel, monitor_sel, follow) => {
                result.push(node_sel.assemble());
                result.push(String::from("--to-monitor"));
                result.push(monitor_sel.assemble());
                if *follow {
                    result.push(String::from("--follow"));
                }
            }
            NodeCommand::ToNode(node_sel1, node_sel2, follow) => {
                result.push(node_sel1.assemble());
                result.push(String::from("--to-node"));
                result.push(node_sel2.assemble());
                if *follow {
                    result.push(String::from("--follow"));
                }
            }
            NodeCommand::Swap(node_sel1, node_sel2) => {
                result.push(node_sel1.assemble());
                result.push(String::from("--swap"));
                result.push(node_sel2.assemble());
            }
            NodeCommand::PreselectDirection(node_sel, direction) => {
                result.push(node_sel.assemble());
                result.push(String::from("--presel-dir"));
                result.push(direction.get_string());
            }
            NodeCommand::Move(node_sel, x, y) => {
                result.push(node_sel.assemble());
                result.push(String::from("--move"));
                result.push(x.to_string());
                result.push(y.to_string());
            }
            NodeCommand::Resize(node_sel, resize_pos, x, y) => {
                result.push(node_sel.assemble());
                result.push(String::from("--resize"));
                result.push(resize_pos.get_string());
                result.push(x.to_string());
                result.push(y.to_string());
            }
            NodeCommand::Ratio(node_sel, ratio) => {
                result.push(node_sel.assemble());
                result.push(String::from("--ratio"));
                result.push(ratio.to_string());
            }
            NodeCommand::Equalize(node_sel) => {
                result.push(node_sel.assemble());
                result.push(String::from("--equalize"));
            }
            NodeCommand::Balance(node_sel) => {
                result.push(node_sel.assemble());
                result.push(String::from("--balance"));
            }
            NodeCommand::State(node_sel, state) => {
                result.push(node_sel.assemble());
                result.push(String::from("--state"));
                result.push(state.get_string());
            }
            NodeCommand::Flag(node_sel, flag) => {
                result.push(node_sel.assemble());
                result.push(String::from("--flag"));
                result.push(flag.get_string());
            }
            NodeCommand::Layer(node_sel, layer) => {
                result.push(node_sel.assemble());
                result.push(String::from("--layer"));
                result.push(layer.get_string());
            }
            NodeCommand::InsertReceptacle => {
                result.push(String::from("--insert-receptacle"));
            }
            NodeCommand::Close(node_sel) => {
                result.push(node_sel.assemble());
                result.push(String::from("--close"));
            }
            NodeCommand::Kill(node_sel) => {
                result.push(node_sel.assemble());
                result.push(String::from("--kill"));
            }
        }
        result
    }
}
