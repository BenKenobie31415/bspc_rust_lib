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

impl ResizePos {
    pub fn get_string(&self) -> String {
        String::from(match self {
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
}
