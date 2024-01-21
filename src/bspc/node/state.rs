pub enum NodeState {
    Tiled,
    PseudoTiled,
    Floating,
    Fullscreen
}

pub fn get_string(node_state: &NodeState) -> String {
    String::from(match node_state {
        NodeState::Tiled => "tiled",
        NodeState::PseudoTiled => "pseudo_tiled",
        NodeState::Floating => "floating",
        NodeState::Fullscreen => "fullscreen"
    })
}
