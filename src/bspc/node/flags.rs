pub enum NodeFlag {
    Hidden,
    NotHidden,
    Sticky,
    NotSticky,
    Private,
    NotPrivate,
    Locked,
    NotLocked,
    Marked,
    NotMarked
}

pub fn get_string(flag: NodeFlag) -> String {
    match flag {
        NodeFlag::Hidden => String::from("hidden=on"),
        NodeFlag::NotHidden => String::from("hidden=off"),
        NodeFlag::Sticky => String::from("sticky=on"),
        NodeFlag::NotSticky => String::from("sticky=off"),
        NodeFlag::Private => String::from("private=on"),
        NodeFlag::NotPrivate => String::from("private=off"),
        NodeFlag::Locked => String::from("locked=on"),
        NodeFlag::NotLocked => String::from("locked=off"),
        NodeFlag::Marked => String::from("marked=on"),
        NodeFlag::NotMarked => String::from("marked=off")
    }
}
