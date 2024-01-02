pub enum Layout {
    Monocle,
    Tiled
}

pub fn get_string_from_layout(layout: &Layout) -> String {
    match layout {
        Layout::Monocle => String::from("monocle"),
        Layout::Tiled => String::from("tiled")
    }
}
