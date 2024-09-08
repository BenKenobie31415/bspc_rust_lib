use crate::bspc::{descriptor::Descriptor, modifier::Modifier, selector::{Assembleable, Selector}};

use super::{descriptor::DesktopDescriptor, modifier::DesktopModifier};

pub struct DesktopSelector {
    reference_selector: Option<Box<DesktopSelector>>,
    descriptor: Option<DesktopDescriptor>,
    modifiers: Vec<DesktopModifier>
}

impl Selector for DesktopSelector {
    type Descriptor = DesktopDescriptor;
    type Modifier = DesktopModifier;

    fn new() -> DesktopSelector {
        DesktopSelector {
            reference_selector: None,
            descriptor: None,
            modifiers: Vec::new()
        }
    }

    fn set_reference_selector(mut self, reference_selector: DesktopSelector) -> Self {
        self.reference_selector = Some(Box::new(reference_selector));
        return self;
    }
    fn set_descriptor(mut self, descriptor: DesktopDescriptor) -> Self {
        self.descriptor = Some(descriptor);
        return self;
    }
    fn add_modifier(mut self, modifier: DesktopModifier) -> Self {
        self.modifiers.push(modifier);
        return self;
    }

    fn get_query_prefix(&self) -> String {
        "--desktop".to_string()
    }
}

impl Assembleable for DesktopSelector {
    fn assemble(&self, default: Option<&DesktopDescriptor>) -> String {
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
