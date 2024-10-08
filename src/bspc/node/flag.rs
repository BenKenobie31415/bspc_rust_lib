pub enum NodeFlag {
    Hidden,
    NotHidden,
    HiddenToggle,
    Sticky,
    NotSticky,
    StickyToggle,
    Private,
    NotPrivate,
    PrivateToggle,
    Locked,
    NotLocked,
    LockedToggle,
    Marked,
    NotMarked,
    MarkedToggle,
    Urgent,
    NotUrgent,
    UrgentToggle,
}

impl NodeFlag {
    pub fn get_string(&self) -> String {
        String::from(match self {
            NodeFlag::Hidden => "hidden=on",
            NodeFlag::NotHidden => "hidden=off",
            NodeFlag::HiddenToggle => "hidden",
            NodeFlag::Sticky => "sticky=on",
            NodeFlag::NotSticky => "sticky=off",
            NodeFlag::StickyToggle => "sticky",
            NodeFlag::Private => "private=on",
            NodeFlag::NotPrivate => "private=off",
            NodeFlag::PrivateToggle => "private",
            NodeFlag::Locked => "locked=on",
            NodeFlag::NotLocked => "locked=off",
            NodeFlag::LockedToggle => "locked",
            NodeFlag::Marked => "marked=on",
            NodeFlag::NotMarked => "marked=off",
            NodeFlag::MarkedToggle => "marked",
            NodeFlag::Urgent => "urgent=on",
            NodeFlag::NotUrgent => "urgent=off",
            NodeFlag::UrgentToggle => "urgent",
        })
    }
}
