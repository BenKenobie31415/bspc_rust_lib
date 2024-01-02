pub struct MonitorSelector {
    pub reference_selector: Option<String>,
    pub descriptor: Option<MonitorDescriptor>,
    pub modifiers: Vec<MonitorModifier>
}

pub(crate) fn assemble_selector(selector: MonitorSelector) -> String {
    let mut result: String = match selector.reference_selector {
        Some(reference_selector) => format!("{}#", reference_selector),
        None => String::new()
    };
    match selector.descriptor {
        Some(descriptor) => {
            result.push_str(&get_descriptor_string(descriptor));
        },
        None => {}
    }
    for modifier in selector.modifiers {
        result.push_str(&get_modifier_string(modifier));
    }
    result
}

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

pub enum MonitorModifier {
    Focused,
    NotFocused,
    Occupied,
    NotOccupied
}

pub fn get_descriptor_string(descriptor: MonitorDescriptor) -> String {
    match descriptor {
        MonitorDescriptor::Any => String::from("any"),
        MonitorDescriptor::Last => String::from("last"),
        MonitorDescriptor::Newest => String::from("newest"),
        MonitorDescriptor::Older => String::from("older"),
        MonitorDescriptor::Newer => String::from("newer"),
        MonitorDescriptor::Focused => String::from("focused"),
        MonitorDescriptor::Pointed => String::from("pointed"),
        MonitorDescriptor::Primary => String::from("primary"),
        MonitorDescriptor::Nth(n) => format!("^{}", n),
        MonitorDescriptor::Id(id) => format!("{}", id),
        MonitorDescriptor::Name(name) => format!("{}", name)
    }
}

pub fn get_modifier_string(modifier: MonitorModifier) -> String {
    match modifier {
        MonitorModifier::Focused => String::from(".focused"),
        MonitorModifier::NotFocused => String::from(".!focused"),
        MonitorModifier::Occupied => String::from(".occupied"),
        MonitorModifier::NotOccupied => String::from(".!occupied"),
    }
}
