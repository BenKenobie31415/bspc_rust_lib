pub struct MonitorSelector {
    pub reference_selector: Option<String>,
    pub descriptor: Option<MonitorDescriptor>,
    pub modifiers: Vec<MonitorModifier>
}

impl MonitorSelector {
    pub fn new() -> MonitorSelector {
        MonitorSelector {
            reference_selector: None,
            descriptor: None,
            modifiers: Vec::new()
        }
    }

    pub fn set_reference_selector(mut self, reference_selector: String) -> Self {
        self.reference_selector = Some(reference_selector);
        self
    }
    pub fn set_descriptor(mut self, descriptor: MonitorDescriptor) -> Self {
        self.descriptor = Some(descriptor);
        self
    }
    pub fn add_modifier(mut self, modifier: MonitorModifier) -> Self {
        self.modifiers.push(modifier);
        self
    }

    pub(crate) fn assemble(&self) -> String {
        let mut result: String = match &self.reference_selector {
            Some(reference_selector) => format!("{}#", reference_selector),
            None => String::new()
        };
        match &self.descriptor {
            Some(descriptor) => {
                result.push_str(&descriptor.get_string());
            },
            None => {}
        }
        for modifier in &self.modifiers {
            result.push_str(&modifier.get_string());
        }
        result
    }
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

impl MonitorDescriptor {
    pub fn get_string(&self) -> String {
        match self {
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
}

impl MonitorModifier {
    pub fn get_string(&self) -> String {
        match self {
            MonitorModifier::Focused => String::from(".focused"),
            MonitorModifier::NotFocused => String::from(".!focused"),
            MonitorModifier::Occupied => String::from(".occupied"),
            MonitorModifier::NotOccupied => String::from(".!occupied"),
        }
    }
}
