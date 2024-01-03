pub struct NodeSelector {
    pub reference_selector: Option<String>,
    pub descriptor: Option<NodeDescriptor>,
    pub modifiers: Vec<NodeModifier>
}

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
    Id(String)
}

pub enum NodeModifier {
    Id(String),

    Focused,
    NotFocused,
    Active,
    NotActive,
    Atuomatic,
    NotAutomatic,
    Local,
    NotLocal,
    Leaf,
    NotLeaf,
    Window,
    NotWindow,

    Tiled,
    NotTiled,
    PseudoTiled,
    NotPseudoTiled,
    Floating,
    NotFloating,
    Fullscreen,
    NotFullscreen,

    Hidden,
    NotHidden,
    Sticky,
    NotSticky,
    Private,
    NotPrivate,
    Locked,
    NotLocked,
    Marked,
    NotMarked,
    Urgent,
    NotUrgent,

    Below,
    NotBelow,
    Normal,
    NotNormal,
    Above,
    NotAbove,

    Horizontal,
    NotHorizontal,
    Vertical,
    NotVertical
}

impl NodeSelector {
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

impl NodeDescriptor {
    pub fn get_string(&self) -> String {
        match self {
            NodeDescriptor::Any => String::from("any"),
            NodeDescriptor::FirstAncestor => String::from("first_ancestor"),
            NodeDescriptor::Last => String::from("last"),
            NodeDescriptor::Newest => String::from("newest"),
            NodeDescriptor::Older => String::from("older"),
            NodeDescriptor::Newer => String::from("newer"),
            NodeDescriptor::Focused => String::from("focused"),
            NodeDescriptor::Pointed => String::from("pointed"),
            NodeDescriptor::Biggest => String::from("biggest"),
            NodeDescriptor::Smallest => String::from("smallest"),
            NodeDescriptor::Id(id) => format!("{}", id)
        }
    }
}

impl NodeModifier {
    pub fn get_string(&self) -> String {
        match self {
            NodeModifier::Id(id) => format!("{}", id),

            NodeModifier::Focused => ".focused".to_string(),
            NodeModifier::NotFocused => ".!focused".to_string(),
            NodeModifier::Active => ".active".to_string(),
            NodeModifier::NotActive => ".!active".to_string(),
            NodeModifier::Atuomatic => ".automatic".to_string(),
            NodeModifier::NotAutomatic => ".!automatic".to_string(),
            NodeModifier::Local => ".local".to_string(),
            NodeModifier::NotLocal => ".!local".to_string(),
            NodeModifier::Leaf => ".leaf".to_string(),
            NodeModifier::NotLeaf => ".!leaf".to_string(),
            NodeModifier::Window => ".window".to_string(),
            NodeModifier::NotWindow => ".!window".to_string(),

            NodeModifier::Tiled => ".tiled".to_string(),
            NodeModifier::NotTiled => ".!tiled".to_string(),
            NodeModifier::PseudoTiled => ".pseudo_tiled".to_string(),
            NodeModifier::NotPseudoTiled => ".!pseudo_tiled".to_string(),
            NodeModifier::Floating => ".floating".to_string(),
            NodeModifier::NotFloating => ".!floating".to_string(),
            NodeModifier::Fullscreen => ".fullscreen".to_string(),
            NodeModifier::NotFullscreen => ".!fullscreen".to_string(),

            NodeModifier::Hidden => ".hidden".to_string(),
            NodeModifier::NotHidden => ".!hidden".to_string(),
            NodeModifier::Sticky => ".sticky".to_string(),
            NodeModifier::NotSticky => ".!sticky".to_string(),
            NodeModifier::Private => ".private".to_string(),
            NodeModifier::NotPrivate => ".!private".to_string(),
            NodeModifier::Locked => ".locked".to_string(),
            NodeModifier::NotLocked => ".!locked".to_string(),
            NodeModifier::Marked => ".marked".to_string(),
            NodeModifier::NotMarked => ".!marked".to_string(),
            NodeModifier::Urgent => ".urgent".to_string(),
            NodeModifier::NotUrgent => ".!urgent".to_string(),

            NodeModifier::Below => ".below".to_string(),
            NodeModifier::NotBelow => ".!below".to_string(),
            NodeModifier::Normal => ".normal".to_string(),
            NodeModifier::NotNormal => ".!normal".to_string(),
            NodeModifier::Above => ".above".to_string(),
            NodeModifier::NotAbove => ".!above".to_string(),

            NodeModifier::Horizontal => ".horizontal".to_string(),
            NodeModifier::NotHorizontal => ".!horizontal".to_string(),
            NodeModifier::Vertical => ".vertical".to_string(),
            NodeModifier::NotVertical => ".!vertical".to_string()
        }
    }
}
