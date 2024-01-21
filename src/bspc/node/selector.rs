use super::{modifier::NodeModifier, descriptor::NodeDescriptor};
pub struct NodeSelector {
    pub reference_selector: Option<String>,
    pub descriptor: Option<NodeDescriptor>,
    pub modifiers: Vec<NodeModifier>
}

impl NodeSelector {
    pub fn new() -> NodeSelector {
        NodeSelector {
            reference_selector: None,
            descriptor: None,
            modifiers: Vec::new()
        }
    }

    pub fn set_reference_selector(mut self, reference_selector: String) -> Self {
        self.reference_selector = Some(reference_selector);
        self
    }
    pub fn set_descriptor(mut self, descriptor: NodeDescriptor) -> Self {
        self.descriptor = Some(descriptor);
        self
    }
    pub fn add_modifier(mut self, modifier: NodeModifier) -> Self {
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
