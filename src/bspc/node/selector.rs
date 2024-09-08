use crate::bspc::{descriptor::Descriptor, modifier::Modifier, selector::{Assembleable, Selector}};

use super::{modifier::NodeModifier, descriptor::NodeDescriptor};

pub struct NodeSelector {
    reference_selector: Option<Box<NodeSelector>>,
    descriptor: Option<NodeDescriptor>,
    modifiers: Vec<NodeModifier>
}

impl Selector for NodeSelector {
    type Descriptor = NodeDescriptor;
    type Modifier = NodeModifier;

    fn new() -> NodeSelector {
        NodeSelector {
            reference_selector: None,
            descriptor: None,
            modifiers: Vec::new()
        }
    }

    fn set_reference_selector(mut self, reference_selector: NodeSelector) -> Self {
        self.reference_selector = Some(Box::new(reference_selector));
        return self;
    }
    fn set_descriptor(mut self, descriptor: NodeDescriptor) -> Self {
        self.descriptor = Some(descriptor);
        return self;
    }
    fn add_modifier(mut self, modifier: NodeModifier) -> Self {
        self.modifiers.push(modifier);
        return self;
    }

    fn get_query_prefix(&self) -> String {
        "--node".to_string()
    }
}

impl Assembleable for NodeSelector {
    fn assemble(&self, default: Option<&NodeDescriptor>) -> String {
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
