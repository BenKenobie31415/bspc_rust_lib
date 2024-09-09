use crate::{bspc::{desktop::selector::DesktopSelector, monitor::selector::MonitorSelector, selector::Assembleable}, socket_communication};

use super::{circulation_direction::CircDirection, descriptor::NodeDescriptor, direction::Direction, flag::NodeFlag, layer::NodeLayer, resize_pos::ResizePos, selector::NodeSelector, split_type::SplitType, state::NodeState};

static DEFAULT_DESCRIPTOR: Option<&NodeDescriptor> = Some(&NodeDescriptor::Focused);

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
    /// Starts preselection in the selected node in the given direction.
    /// # Arguments
    /// - `node_sel`: The node to split
    /// - `direction`: The direction to preselect in
    /// - `toggle`: Wether to cancel preselection if the current preselection direction is chosen again
    PreselectDirection(NodeSelector, Direction, bool),
    /// Cancels preselection in the selected node.
    /// # Arguments
    /// - `node_sel`: The node to cancel preselection for
    PreselectCancel(NodeSelector),
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
    /// Rotates the tree rooted at the selected node by the given amount.
    /// # Arguments
    /// - `node_sel`: The node that is the root of the tree to rotate
    /// - `angle`: The angle to rotate the tree by (value in [0, 90, 180, 270] that is closest to given angle is used)
    Rotate(NodeSelector, i32),
    /// Flips the tree rooted at the selected node.
    /// # Arguments
    /// - `node_sel`: The node that is the root of the tree to flip
    /// - `flip_axis`: The axis to flip the tree on
    Flip(NodeSelector, SplitType),
    /// Resets the splitting ratios of the tree rooted at the selected node.
    /// # Arguments
    /// - `node_sel`: The node that is the root of the tree to equalize
    Equalize(NodeSelector),
    /// Adjusts splitting ratios so that all windows in the tree rooted in the selected node occupy the same area.
    /// # Arguments
    /// - `node_sel`: The node that is the root of the tree to balance
    Balance(NodeSelector),
    /// Circulates the windows of the tree rooted at the selected node.
    /// # Arguments
    /// - `node_sel`: The node that is the root of the tree to rotate
    /// - `circ_dir`: The direction in which to circulate
    Circulate(NodeSelector, CircDirection),
    /// Sets the state of the selected node.
    /// # Arguments
    /// - `node_sel`: The node to set the state for
    /// - `state`: New flag of the focused node
    State(NodeSelector, NodeState, bool),
    /// Sets/toggles flag for the selected node.
    /// # Arguments
    /// - `node_sel`: The node to set/toggle the flag for
    /// - `flag`: The flag to set/toggle
    /// - `toggle`: If toggle is true and the chosen state is the current state of the selected node, the node is set to its previous state
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
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--focus"));
            }
            NodeCommand::Activate(node_sel) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--activate"));
            }
            NodeCommand::ToDesktop(node_sel, desktop_sel, follow) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--to-desktop"));
                result.push(desktop_sel.assemble(None));
                if *follow {
                    result.push(String::from("--follow"));
                }
            }
            NodeCommand::ToMonitor(node_sel, monitor_sel, follow) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--to-monitor"));
                result.push(monitor_sel.assemble(None));
                if *follow {
                    result.push(String::from("--follow"));
                }
            }
            NodeCommand::ToNode(node_sel1, node_sel2, follow) => {
                result.push(node_sel1.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--to-node"));
                result.push(node_sel2.assemble(None));
                if *follow {
                    result.push(String::from("--follow"));
                }
            }
            NodeCommand::Swap(node_sel1, node_sel2) => {
                result.push(node_sel1.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--swap"));
                result.push(node_sel2.assemble(None));
            }
            NodeCommand::PreselectDirection(node_sel, direction, toggle) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--presel-dir"));
                let prefix = match toggle {
                    true => "~",
                    false => "",
                };
                result.push(format!("{}{}", prefix, direction.get_string()));
            }
            NodeCommand::PreselectCancel(node_sel) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--presel-dir"));
                result.push(String::from("cancel"));
            }
            NodeCommand::Move(node_sel, x, y) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--move"));
                result.push(x.to_string());
                result.push(y.to_string());
            }
            NodeCommand::Resize(node_sel, resize_pos, x, y) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--resize"));
                result.push(resize_pos.get_string());
                result.push(x.to_string());
                result.push(y.to_string());
            }
            NodeCommand::Ratio(node_sel, ratio) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--ratio"));
                result.push(ratio.to_string());
            }
            NodeCommand::Rotate(node_sel, angle) => {
                let rot_angle = match angle.rem_euclid(360) {
                    0..=44 => 0,
                    45..=134 => 90,
                    135..=224 => 180,
                    225..=314 => 270,
                    315..=360 => 0,
                    _ => -1,
                };
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--rotate"));
                result.push(rot_angle.to_string());
            }
            NodeCommand::Flip(node_sel, axis) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--flip"));
                result.push(axis.get_string());
            }
            NodeCommand::Equalize(node_sel) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--equalize"));
            }
            NodeCommand::Balance(node_sel) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--balance"));
            }
            NodeCommand::Circulate(node_sel, circ_dir) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--circulate"));
                result.push(circ_dir.get_string());
            }
            NodeCommand::State(node_sel, state, toggle) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--state"));
                let prefix = match toggle {
                    true => "~",
                    false => "",
                };
                result.push(format!("{}{}", prefix, state.get_string()));
            }
            NodeCommand::Flag(node_sel, flag) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--flag"));
                result.push(flag.get_string());
            }
            NodeCommand::Layer(node_sel, layer) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--layer"));
                result.push(layer.get_string());
            }
            NodeCommand::InsertReceptacle => {
                result.push(String::from("--insert-receptacle"));
            }
            NodeCommand::Close(node_sel) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--close"));
            }
            NodeCommand::Kill(node_sel) => {
                result.push(node_sel.assemble(DEFAULT_DESCRIPTOR));
                result.push(String::from("--kill"));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::bspc::{desktop::{descriptor::DesktopDescriptor, selector::DesktopSelector}, node::{command::NodeCommand, descriptor::NodeDescriptor, selector::NodeSelector}, selector::Selector};

    #[test]
    fn focus_older() {
        let cmd = NodeCommand::Focus(NodeSelector::new().set_descriptor(NodeDescriptor::Older)).assemble();

        assert_eq!(cmd, vec!["node", "older", "--focus"]);
    }

    #[test]
    fn move_focused_to_third_desktop() {
        let cmd = NodeCommand::ToDesktop(
            NodeSelector::new(),
            DesktopSelector::new().set_descriptor(DesktopDescriptor::Nth(3)),
        true).assemble();

        assert_eq!(cmd, vec!["node", "focused", "--to-desktop", "^3", "--follow"]);
    }

    #[test]
    fn close_focused() {
        let cmd = NodeCommand::Close(NodeSelector::new().set_descriptor(NodeDescriptor::Focused)).assemble();

        assert_eq!(cmd, vec!["node", "focused", "--close"]);
    }

    #[test]
    fn ratio() {
        let cmd = NodeCommand::Ratio(NodeSelector::new(), 0.1).assemble();
        
        assert_eq!(cmd, vec!["node", "focused", "--ratio", "0.1"]);
    }
    
    #[test]
    fn rotation() {
        let cmd = NodeCommand::Rotate(NodeSelector::new(), -134).assemble();

        assert_eq!(cmd, vec!["node", "focused", "--rotate", "270"]);
    }
}
