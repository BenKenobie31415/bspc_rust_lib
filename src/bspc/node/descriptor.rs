use crate::bspc::node::path::Path;
pub enum NodeDescriptor {
    // TODO path
    Any,
    FirstAncestor,
    Last,
    Newest,
    Older,
    Newer,
    Focused,
    Pointed,
    Biggest,
    Smallest,
    Id(String),
    Path(Path)
}

impl NodeDescriptor {
    pub fn get_string(&self) -> String {
        String::from(match self {
            NodeDescriptor::Any => "any",
            NodeDescriptor::FirstAncestor => "first_ancestor",
            NodeDescriptor::Last => "last",
            NodeDescriptor::Newest => "newest",
            NodeDescriptor::Older => "older",
            NodeDescriptor::Newer => "newer",
            NodeDescriptor::Focused => "focused",
            NodeDescriptor::Pointed => "pointed",
            NodeDescriptor::Biggest => "biggest",
            NodeDescriptor::Smallest => "smallest",
            NodeDescriptor::Id(id) => id,
            NodeDescriptor::Path(path) => {return path.assemble();}
        })
    }
}