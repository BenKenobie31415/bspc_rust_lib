pub enum NodeLayer {
    Below,
    Normal,
    Above
}

pub fn get_string(layer: &NodeLayer) -> String {
    match layer {
        NodeLayer::Below => String::from("below"),
        NodeLayer::Normal => String::from("normal"),
        NodeLayer::Above => String::from("above")
    }
}
