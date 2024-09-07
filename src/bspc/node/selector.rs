use super::{modifier::NodeModifier, descriptor::NodeDescriptor};

pub struct NodeSelector {
    reference_selector: Option<Box<NodeSelector>>,
    descriptor: Option<NodeDescriptor>,
    modifiers: Vec<NodeModifier>
}

impl NodeSelector {
    pub fn new() -> NodeSelector {
        NodeSelector {
            reference_selector: None,
            descriptor: None,
            modifiers: Vec::new()
        }
    }

    pub fn set_reference_selector(mut self, reference_selector: NodeSelector) -> Self {
        self.reference_selector = Some(Box::new(reference_selector));
        return self;
    }
    pub fn set_descriptor(mut self, descriptor: NodeDescriptor) -> Self {
        self.descriptor = Some(descriptor);
        return self;
    }
    pub fn add_modifier(mut self, modifier: NodeModifier) -> Self {
        self.modifiers.push(modifier);
        return self;
    }

    pub(crate) fn assemble(&self, default: Option<&NodeDescriptor>) -> String {
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
