use crate::bspc::{descriptor::Descriptor, modifier::Modifier, selector::{Assembleable, Selector}};

use super::{modifier::MonitorModifier, descriptor::MonitorDescriptor};

pub struct MonitorSelector {
    reference_selector: Option<Box<MonitorSelector>>,
    descriptor: Option<MonitorDescriptor>,
    modifiers: Vec<MonitorModifier>
}

impl Selector for MonitorSelector {
    type Descriptor = MonitorDescriptor;
    type Modifier = MonitorModifier;

    fn new() -> MonitorSelector {
        MonitorSelector {
            reference_selector: None,
            descriptor: None,
            modifiers: Vec::new()
        }
    }

    fn set_reference_selector(mut self, reference_selector: MonitorSelector) -> Self {
        self.reference_selector = Some(Box::new(reference_selector));
        self
    }
    fn set_descriptor(mut self, descriptor: MonitorDescriptor) -> Self {
        self.descriptor = Some(descriptor);
        self
    }
    fn add_modifier(mut self, modifier: MonitorModifier) -> Self {
        self.modifiers.push(modifier);
        self
    }

    fn get_query_prefix(&self) -> String {
        "--monitor".to_string()
    }
}

impl Assembleable for MonitorSelector {
    fn assemble(&self, default: Option<&MonitorDescriptor>) -> String {
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
        result
    }
}
