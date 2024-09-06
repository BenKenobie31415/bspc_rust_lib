pub enum Layout {
    Monocle,
    Tiled
}

impl Layout {
    pub fn get_string(&self) -> String {
        String::from(match self {
            Layout::Monocle => "monocle",
            Layout::Tiled => "tiled"
        })
    }
}
