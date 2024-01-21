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

pub fn get_string(flag: &NodeFlag) -> String {
    String::from(match flag {
        NodeFlag::Hidden => "hidden=on",
        NodeFlag::NotHidden => "hidden=off",
        NodeFlag::Sticky => "sticky=on",
        NodeFlag::NotSticky => "sticky=off",
        NodeFlag::Private => "private=on",
        NodeFlag::NotPrivate => "private=off",
        NodeFlag::Locked => "locked=on",
        NodeFlag::NotLocked => "locked=off",
        NodeFlag::Marked => "marked=on",
        NodeFlag::NotMarked => "marked=off"
    })
}
