pub enum MonitorDescriptor {
    Any,
    Last,
    Newest,
    // TODO older
    // TODO newer
    Focused,
    Pointed,
    Primary,
    Nth(u32),
    Id(String),
    Name(String)
}

pub fn get_string(descriptor: MonitorDescriptor) -> String {
    match descriptor {
        MonitorDescriptor::Any => String::from("any"),
        MonitorDescriptor::Last => String::from("last"),
        MonitorDescriptor::Newest => String::from("newest"),
        MonitorDescriptor::Focused => String::from("focused"),
        MonitorDescriptor::Pointed => String::from("pointed"),
        MonitorDescriptor::Primary => String::from("primary"),
        MonitorDescriptor::Nth(n) => format!("^{}", n),
        MonitorDescriptor::Id(id) => format!("{}", id),
        MonitorDescriptor::Name(name) => format!("{}", name)
    }
}
