pub enum CycleDir {
    Next,
    Prev,
}

impl CycleDir {
    pub fn get_string(&self) -> String {
        String::from(
            match self {
                CycleDir::Next => "next",
                CycleDir::Prev => "prev",
            }
        )
    }
}
