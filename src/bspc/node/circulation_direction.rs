pub enum CircDirection {
    Forward,
    Backward,
}

impl CircDirection {
    pub fn get_string(&self) -> String {
        String::from(
            match self {
                CircDirection::Forward => "forward",
                CircDirection::Backward => "backward",
            }
        )
    }
}
