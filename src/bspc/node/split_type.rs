pub enum SplitType {
    Horizontal,
    Vertical,
}

impl SplitType {
    pub fn get_string(&self) -> String {
        String::from(match self {
            SplitType::Horizontal => "horizontal",
            SplitType::Vertical => "vertical",
        })
    }
}

