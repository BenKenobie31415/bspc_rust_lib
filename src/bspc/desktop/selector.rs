use super::{descriptor::DesktopDescriptor, modifier::DesktopModifier};

pub struct DesktopSelector {
    pub reference_selector: Option<Box<DesktopSelector>>,
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

    pub fn set_reference_selector(mut self, reference_selector: DesktopSelector) -> Self {
        self.reference_selector = Some(Box::new(reference_selector));
        return self;
    }
    pub fn set_descriptor(mut self, descriptor: DesktopDescriptor) -> Self {
        self.descriptor = Some(descriptor);
        return self;
    }
    pub fn add_modifier(mut self, modifier: DesktopModifier) -> Self {
        self.modifiers.push(modifier);
        return self;
    }

    pub(crate) fn assemble(&self, default: Option<&DesktopDescriptor>) -> String {
        let mut result: String = match &self.reference_selector {
            Some(reference_selector) => format!("{}#", reference_selector.assemble(None)),
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
        return result;
    }
}
