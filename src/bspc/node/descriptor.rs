use crate::bspc::node::path::Path;

use super::direction::Direction;
pub enum NodeDescriptor {
    Dir(Direction),
    Path(Path),
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
}

impl NodeDescriptor {
    pub fn get_string(&self) -> String {
        match self {
            NodeDescriptor::Dir(direction) => direction.get_string(),
            NodeDescriptor::Path(path) => path.assemble(),
            NodeDescriptor::Any => "any".to_string(),
            NodeDescriptor::FirstAncestor => "first_ancestor".to_string(),
            NodeDescriptor::Last => "last".to_string(),
            NodeDescriptor::Newest => "newest".to_string(),
            NodeDescriptor::Older => "older".to_string(),
            NodeDescriptor::Newer => "newer".to_string(),
            NodeDescriptor::Focused => "focused".to_string(),
            NodeDescriptor::Pointed => "pointed".to_string(),
            NodeDescriptor::Biggest => "biggest".to_string(),
            NodeDescriptor::Smallest => "smallest".to_string(),
            NodeDescriptor::Id(id) => id.to_string(),
        }
    }
}
