/// For documentation on the event-payload and the causes of an event see the bspc-manpage.
pub enum Event {
    //TODO Report,
    MonitorAdd,
    MonitorRename,
    MonitorRemove,
    
    MonitorSwap,
    MonitorFocus,
    MonitorGeometry,

    DesktopAdd,
    DesktopRename,
    DesktopRemove,
    DesktopSwap,
    DesktopTransfer,
    DesktopFocus,
    DesktopActivate,
    DesktopLayout,

    NodeAdd,
    NodeRemove,
    NodeSwap,
    NodeTransfer,
    NodeFocus,
    NodeActivate,
    NodePresel,
    NodeStack,
    NodeGeometry,
    NodeState,
    NodeFlag,
    NodeLayer,

    PointerAction
}

impl Event {
    pub fn get_str(&self) -> String {
        match self {
            Event::MonitorAdd => "monitor_add".to_string(),
            Event::MonitorRename => "monitor_rename".to_string(),
            Event::MonitorRemove => "monitor_remove".to_string(),
            Event::MonitorSwap => "monitor_swap".to_string(),
            Event::MonitorFocus => "monitor_focus".to_string(),
            Event::MonitorGeometry => "monitor_geometry".to_string(),

            Event::DesktopAdd => "desktop_add".to_string(),
            Event::DesktopRename => "desktop_rename".to_string(),
            Event::DesktopRemove => "desktop_remove".to_string(),
            Event::DesktopSwap => "desktop_swap".to_string(),
            Event::DesktopTransfer => "desktop_transfer".to_string(),
            Event::DesktopFocus => "desktop_focus".to_string(),
            Event::DesktopActivate => "desktop_activate".to_string(),
            Event::DesktopLayout => "desktop_layout".to_string(),

            Event::NodeAdd => "node_add".to_string(),
            Event::NodeRemove => "node_remove".to_string(),
            Event::NodeSwap => "node_swap".to_string(),
            Event::NodeTransfer => "node_transfer".to_string(),
            Event::NodeFocus => "node_focus".to_string(),
            Event::NodeActivate => "node_activate".to_string(),
            Event::NodePresel => "node_presel".to_string(),
            Event::NodeStack => "node_stack".to_string(),
            Event::NodeGeometry => "node_geometry".to_string(),
            Event::NodeState => "node_state".to_string(),
            Event::NodeFlag => "node_flag".to_string(),
            Event::NodeLayer => "node_layer".to_string(),

            Event::PointerAction => "pointer_action".to_string(),
        }
    }

    /// Returns how many values are contained within the payload that is given by the bspc-event
    pub fn get_payload_count(&self) -> usize {
        match self {
            Event::MonitorAdd => 3,
            Event::MonitorRename => 3,
            Event::MonitorRemove => 1,
            Event::MonitorSwap => 2,
            Event::MonitorFocus => 1,
            Event::MonitorGeometry => 2,

            Event::DesktopAdd => 3,
            Event::DesktopRename => 4,
            Event::DesktopRemove => 2,
            Event::DesktopSwap => 4,
            Event::DesktopTransfer => 3,
            Event::DesktopFocus => 2,
            Event::DesktopActivate => 2,
            Event::DesktopLayout => 3,

            Event::NodeAdd => 4,
            Event::NodeRemove => 3,
            Event::NodeSwap => 6,
            Event::NodeTransfer => 6,
            Event::NodeFocus => 3,
            Event::NodeActivate => 3,
            Event::NodePresel => 4,
            Event::NodeStack => 3,
            Event::NodeGeometry => 4,
            Event::NodeState => 5,
            Event::NodeFlag => 5,
            Event::NodeLayer => 4,

            Event::PointerAction => 5,
        }
    }
}
