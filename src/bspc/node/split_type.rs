pub enum SplitType {
    Horizontal,
    Vertical,
}

pub fn get_string(split_type: &SplitType) -> String {
    String::from(match split_type {
        SplitType::Horizontal => "horizontal",
        SplitType::Vertical => "vertical",
    })
}

