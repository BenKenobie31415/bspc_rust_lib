pub enum NodeLayer {
    Below,
    Normal,
    Above
}

impl NodeLayer {
    pub fn get_string(&self) -> String {
        String::from(match self {
            NodeLayer::Below => "below",
            NodeLayer::Normal => "normal",
            NodeLayer::Above => "above"
        })
    }
}
