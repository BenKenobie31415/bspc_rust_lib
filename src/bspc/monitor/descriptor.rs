pub enum MonitorDescriptor {
    // TODO path
    // TODO cycle_dir
    Any,
    Last,
    Newest,
    Older,
    Newer,
    Focused,
    Pointed,
    Primary,
    Nth(u32),
    Id(String),
    Name(String)
}

impl MonitorDescriptor {
    pub fn get_string(&self) -> String {
        String::from(match self {
            MonitorDescriptor::Any => "any",
            MonitorDescriptor::Last => "last",
            MonitorDescriptor::Newest => "newest",
            MonitorDescriptor::Older => "older",
            MonitorDescriptor::Newer => "newer",
            MonitorDescriptor::Focused => "focused",
            MonitorDescriptor::Pointed => "pointed",
            MonitorDescriptor::Primary => "primary",
            MonitorDescriptor::Nth(n) => {return format!("^{}", n);},
            MonitorDescriptor::Id(id) => id,
            MonitorDescriptor::Name(name) => name
        })
    }
}
