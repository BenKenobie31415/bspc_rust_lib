pub enum NodeState {
    Tiled,
    PseudoTiled,
    Floating,
    Fullscreen
}

impl NodeState{
    pub fn get_string(&self) -> String {
        String::from(match self {
            NodeState::Tiled => "tiled",
            NodeState::PseudoTiled => "pseudo_tiled",
            NodeState::Floating => "floating",
            NodeState::Fullscreen => "fullscreen"
        })
    }
}
