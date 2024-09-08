use crate::bspc::descriptor::Descriptor;

pub enum DesktopDescriptor {
    Any,
    Last,
    Newest,
    Older,
    Newer,
    Focused,
    Nth(u32),
    Id(String),
    Name(String)
}

impl Descriptor for DesktopDescriptor {
    fn get_string(&self) -> String {
        String::from(match self {
            DesktopDescriptor::Any => "any",
            DesktopDescriptor::Last => "last",
            DesktopDescriptor::Newest => "newest",
            DesktopDescriptor::Older => "older",
            DesktopDescriptor::Newer => "newer",
            DesktopDescriptor::Focused => "focused",
            DesktopDescriptor::Nth(n) => {return format!("^{}", n)},
            DesktopDescriptor::Id(id) => id,
            DesktopDescriptor::Name(name) => name
        })
    }
}
