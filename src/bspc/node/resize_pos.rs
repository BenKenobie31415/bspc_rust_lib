pub enum ResizePos {
    Top,
    Left,
    Bottom,
    Right,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft
}

pub fn get_string(resize_pos: &ResizePos) -> String {
    match resize_pos {
        ResizePos::Top => "top".to_string(),
        ResizePos::Left => "left".to_string(),
        ResizePos::Bottom => "bottom".to_string(),
        ResizePos::Right => "right".to_string(),
        ResizePos::TopLeft => "top_left".to_string(),
        ResizePos::TopRight => "top_right".to_string(),
        ResizePos::BottomRight => "bottom_right".to_string(),
        ResizePos::BottomLeft => "bottom_left".to_string()
    }
}
