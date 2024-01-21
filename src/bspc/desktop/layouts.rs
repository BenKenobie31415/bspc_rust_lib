pub enum Layout {
    Monocle,
    Tiled
}

pub fn get_string_from_layout(layout: &Layout) -> String {
    String::from(match layout {
        Layout::Monocle => "monocle",
        Layout::Tiled => "tiled"
    })
}
