use super::{descriptor::DesktopDescriptor, modifier::DesktopModifier};

pub struct DesktopSelector {
    pub reference_selector: Option<String>,
    pub descriptor: Option<DesktopDescriptor>,
    pub modifiers: Vec<DesktopModifier>
}

impl DesktopSelector {
    pub fn new() -> DesktopSelector {
        DesktopSelector {
            reference_selector: None,
            descriptor: None,
            modifiers: Vec::new()
        }
    }

    pub fn set_reference_selector(mut self, reference_selector: String) -> Self {
        self.reference_selector = Some(reference_selector);
        self
    }
    pub fn set_descriptor(mut self, descriptor: DesktopDescriptor) -> Self {
        self.descriptor = Some(descriptor);
        self
    }
    pub fn add_modifier(mut self, modifier: DesktopModifier) -> Self {
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
