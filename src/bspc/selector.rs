pub trait Selector {
    type Descriptor;
    type Modifier;
    

    fn new() -> Self;
    fn set_reference_selector(self, reference_selector: Self) -> Self;
    fn set_descriptor(self, descriptor: Self::Descriptor) -> Self;
    fn add_modifier(self, modifier: Self::Modifier) -> Self;
    fn get_query_prefix(&self) -> String;
}

pub(crate) trait Assembleable: Selector {
    fn assemble(&self, default: Option<&Self::Descriptor>) -> String;
}
