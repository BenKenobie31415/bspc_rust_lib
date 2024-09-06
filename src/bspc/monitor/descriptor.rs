use crate::bspc::cycle_direction::CycleDir;

pub enum MonitorDescriptor {
    CycleDir(CycleDir),
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
        match self {
            MonitorDescriptor::CycleDir(cycle_dir) => cycle_dir.get_string(),
            MonitorDescriptor::Any => "any".to_string(),
            MonitorDescriptor::Last => "last".to_string(),
            MonitorDescriptor::Newest => "newest".to_string(),
            MonitorDescriptor::Older => "older".to_string(),
            MonitorDescriptor::Newer => "newer".to_string(),
            MonitorDescriptor::Focused => "focused".to_string(),
            MonitorDescriptor::Pointed => "pointed".to_string(),
            MonitorDescriptor::Primary => "primary".to_string(),
            MonitorDescriptor::Nth(n) => format!("^{}", n),
            MonitorDescriptor::Id(id) => id,
            MonitorDescriptor::Name(name) => name
        }
    }
}
