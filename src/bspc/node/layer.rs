pub enum NodeLayer {
    Below,
    Normal,
    Above
}

pub fn get_string(layer: &NodeLayer) -> String {
    String::from(match layer {
        NodeLayer::Below => "below",
        NodeLayer::Normal => "normal",
        NodeLayer::Above => "above"
    })
}
