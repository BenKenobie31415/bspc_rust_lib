use super::{modifier::MonitorModifier, descriptor::MonitorDescriptor};

pub struct MonitorSelector {
    reference_selector: Option<String>,
    descriptor: Option<MonitorDescriptor>,
    modifiers: Vec<MonitorModifier>
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

    pub(crate) fn assemble(&self, default: Option<&MonitorDescriptor>) -> String {
        let mut result: String = match &self.reference_selector {
            Some(reference_selector) => format!("{}#", reference_selector),
            None => String::new()
        };
        match &self.descriptor {
            Some(descriptor) => {
                result.push_str(&descriptor.get_string());
            },
            None => {
                match default {
                    Some(default_value) => {
                        result.push_str(&default_value.get_string());
                    }
                    None => {}
                }
            }
        }
        for modifier in &self.modifiers {
            result.push_str(&modifier.get_string());
        }
        result
    }
}
