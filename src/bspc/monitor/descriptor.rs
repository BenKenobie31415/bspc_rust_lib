use crate::bspc::{cycle_direction::CycleDir, descriptor::Descriptor};

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

impl Descriptor for MonitorDescriptor {
    fn get_string(&self) -> String {
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
            MonitorDescriptor::Id(id) => id.to_string(),
            MonitorDescriptor::Name(name) => name.to_string()
        }
    }
}
