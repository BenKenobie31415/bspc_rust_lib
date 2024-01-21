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
    String::from(match resize_pos {
        ResizePos::Top => "top",
        ResizePos::Left => "left",
        ResizePos::Bottom => "bottom",
        ResizePos::Right => "right",
        ResizePos::TopLeft => "top_left",
        ResizePos::TopRight => "top_right",
        ResizePos::BottomRight => "bottom_right",
        ResizePos::BottomLeft => "bottom_left"
    })
}
