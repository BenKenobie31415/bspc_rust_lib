pub mod bspc;
pub(crate) mod socket_communication;
pub mod subscription;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::bspc::{cycle_direction::CycleDir, desktop::{command::DesktopCommand, descriptor::DesktopDescriptor, modifier::DesktopModifier, selector::DesktopSelector}, monitor::{command::MonitorCommand, descriptor::MonitorDescriptor, selector::MonitorSelector}, node::{command::NodeCommand, descriptor::NodeDescriptor, modifier::NodeModifier, selector::NodeSelector}, query::QueryCommand};


    #[test]
    fn node_focus_older() {
        let cmd = NodeCommand::Focus(NodeSelector::new().set_descriptor(NodeDescriptor::Older)).assemble();

        assert_eq!(cmd, vec!["node", "older", "--focus"]);
    }

    #[test]
    fn node_move_focused_to_third_desktop() {
        let cmd = NodeCommand::ToDesktop(
            NodeSelector::new(),
            DesktopSelector::new().set_descriptor(DesktopDescriptor::Nth(3)),
        true).assemble();

        assert_eq!(cmd, vec!["node", "focused", "--to-desktop", "^3", "--follow"]);
    }

    #[test]
    fn node_close_focused() {
        let cmd = NodeCommand::Close(NodeSelector::new().set_descriptor(NodeDescriptor::Focused)).assemble();

        assert_eq!(cmd, vec!["node", "focused", "--close"]);
    }

    #[test]
    fn node_ratio() {
        let cmd = NodeCommand::Ratio(NodeSelector::new(), 0.1).assemble();
        
        assert_eq!(cmd, vec!["node", "focused", "--ratio", "0.1"]);
    }

    #[test]
    fn desktop_focus_fifth() {
        let cmd = DesktopCommand::Focus(DesktopSelector::new().set_descriptor(DesktopDescriptor::Nth(5))).assemble();

        assert_eq!(cmd, vec!["desktop", "^5", "--focus"]);
    }

    #[test]
    fn desktop_rename_focused() {
        let cmd = DesktopCommand::Rename(DesktopSelector::new(), "testname".to_string()).assemble();

        assert_eq!(cmd, vec!["desktop", "focused", "--rename", "testname"]);
    }

    #[test]
    fn desktop_bubble_next() {
        let cmd = DesktopCommand::Bubble(DesktopSelector::new(), CycleDir::Next).assemble();

        assert_eq!(cmd, vec!["desktop", "focused", "--bubble", "next"]);
    }

    #[test]
    fn monitor_remove() {
        let cmd = MonitorCommand::Remove(MonitorSelector::new().set_descriptor(MonitorDescriptor::Name("DP1-1".to_string()))).assemble();

        assert_eq!(cmd, vec!["monitor", "DP1-1", "--remove"]);
    }

    #[test]
    fn query_all_windows() {
        let cmd = QueryCommand::CNodes(Some(NodeSelector::new().add_modifier(NodeModifier::Window))).assemble();

        assert_eq!(cmd, vec!["query", "--nodes", "--node", ".window"]);
    }

    #[test]
    fn query_occupied_desktop_names() {
        let cmd = QueryCommand::CDesktops(Some(DesktopSelector::new().add_modifier(DesktopModifier::Occupied)), true).assemble();

        assert_eq!(cmd, vec!["query", "--desktops", "--desktop", ".occupied", "--names"]);
    }

    #[test]
    fn query_focused_monitor_name() {
        let cmd = QueryCommand::CMonitors(Some(MonitorSelector::new().set_descriptor(MonitorDescriptor::Focused)), true).assemble();

        assert_eq!(cmd, vec!["query", "--monitors", "--monitor", "focused", "--names"]);
    }

    #[test]
    fn query_primary_monitor_id_if_() {
        let cmd = QueryCommand::Monitors(
            Some(NodeSelector::new().add_modifier(NodeModifier::Window)),
            None,
            Some(MonitorSelector::new().set_descriptor(MonitorDescriptor::Primary)), false).assemble();

        assert_eq!(cmd, vec!["query", "--monitors", "--node", ".window", "--monitor", "primary"]);
    }
}
