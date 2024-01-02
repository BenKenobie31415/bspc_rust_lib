pub struct DesktopSelector {
    pub reference_selector: Option<String>,
    pub descriptor: Option<DesktopDescriptor>,
    pub modifiers: Vec<DesktopModifier>
}

pub(crate) fn assemble_selector(selector: DesktopSelector) -> String {
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

pub enum DesktopDescriptor {
    // TODO path
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

pub enum DesktopModifier {
    Id(String),
    Name(String),
    Focused,
    NotFocused,
    Active,
    NotActive,
    Occupied,
    NotOccupied,
    Urgent,
    NotUrgent,
    Local,
    NotLocal
}

pub fn get_descriptor_string(descriptor: DesktopDescriptor) -> String {
    match descriptor {
        DesktopDescriptor::Any => String::from("any"),
        DesktopDescriptor::Last => String::from("last"),
        DesktopDescriptor::Newest => String::from("newest"),
        DesktopDescriptor::Older => String::from("older"),
        DesktopDescriptor::Newer => String::from("newer"),
        DesktopDescriptor::Focused => String::from("focused"),
        DesktopDescriptor::Nth(n) => format!("^{}", n),
        DesktopDescriptor::Id(id) => format!("{}", id),
        DesktopDescriptor::Name(name) => format!("{}", name)
    }
}

pub fn get_modifier_string(modifer: DesktopModifier) -> String {
    match modifer {
        DesktopModifier::Id(id) => id.clone(),
        DesktopModifier::Name(name) => name.clone(),
        DesktopModifier::Focused => String::from(".focused"),
        DesktopModifier::NotFocused => String::from(".!focused"),
        DesktopModifier::Active => String::from(".active"),
        DesktopModifier::NotActive => String::from(".!active"),
        DesktopModifier::Occupied => String::from(".occupied"),
        DesktopModifier::NotOccupied => String::from(".!occupied"),
        DesktopModifier::Urgent => String::from(".urgent"),
        DesktopModifier::NotUrgent => String::from(".!urgent"),
        DesktopModifier::Local => String::from(".local"),
        DesktopModifier::NotLocal => String::from(".!local")
    }
}
