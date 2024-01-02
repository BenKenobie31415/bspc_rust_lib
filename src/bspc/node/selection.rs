pub struct NodeSelector {
    pub reference_selector: Option<String>,
    pub descriptor: Option<NodeDescriptor>,
    pub modifiers: Vec<NodeModifiers>
}

pub(crate) fn assemble_selector(selector: NodeSelector) -> String {
    let mut result: String = match selector.reference_selector {
        Some(reference_selector) => format!("{}#", reference_selector),
        None => String::new()
    };
    match selector.descriptor {
        Some(descriptor) => {
            result.push_str(&get_descriptor_string(descriptor));
        },
        None => {}
    }
    for modifier in selector.modifiers {
        result.push_str(&get_modifier_string(modifier));
    }
    result
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

pub enum NodeModifiers {
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

pub fn get_descriptor_string(descriptor: NodeDescriptor) -> String {
    match descriptor {
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

pub fn get_modifier_string(modifier: NodeModifiers) -> String {
    let result = match modifier {
        NodeModifiers::Id(id) => format!("{}", id),

        NodeModifiers::Focused => ".focused".to_string(),
        NodeModifiers::NotFocused => ".!focused".to_string(),
        NodeModifiers::Active => ".active".to_string(),
        NodeModifiers::NotActive => ".!active".to_string(),
        NodeModifiers::Atuomatic => ".automatic".to_string(),
        NodeModifiers::NotAutomatic => ".!automatic".to_string(),
        NodeModifiers::Local => ".local".to_string(),
        NodeModifiers::NotLocal => ".!local".to_string(),
        NodeModifiers::Leaf => ".leaf".to_string(),
        NodeModifiers::NotLeaf => ".!leaf".to_string(),
        NodeModifiers::Window => ".window".to_string(),
        NodeModifiers::NotWindow => ".!window".to_string(),

        NodeModifiers::Tiled => ".tiled".to_string(),
        NodeModifiers::NotTiled => ".!tiled".to_string(),
        NodeModifiers::PseudoTiled => ".pseudo_tiled".to_string(),
        NodeModifiers::NotPseudoTiled => ".!pseudo_tiled".to_string(),
        NodeModifiers::Floating => ".floating".to_string(),
        NodeModifiers::NotFloating => ".!floating".to_string(),
        NodeModifiers::Fullscreen => ".fullscreen".to_string(),
        NodeModifiers::NotFullscreen => ".!fullscreen".to_string(),

        NodeModifiers::Hidden => ".hidden".to_string(),
        NodeModifiers::NotHidden => ".!hidden".to_string(),
        NodeModifiers::Sticky => ".sticky".to_string(),
        NodeModifiers::NotSticky => ".!sticky".to_string(),
        NodeModifiers::Private => ".private".to_string(),
        NodeModifiers::NotPrivate => ".!private".to_string(),
        NodeModifiers::Locked => ".locked".to_string(),
        NodeModifiers::NotLocked => ".!locked".to_string(),
        NodeModifiers::Marked => ".marked".to_string(),
        NodeModifiers::NotMarked => ".!marked".to_string(),
        NodeModifiers::Urgent => ".urgent".to_string(),
        NodeModifiers::NotUrgent => ".!urgent".to_string(),

        NodeModifiers::Below => ".below".to_string(),
        NodeModifiers::NotBelow => ".!below".to_string(),
        NodeModifiers::Normal => ".normal".to_string(),
        NodeModifiers::NotNormal => ".!normal".to_string(),
        NodeModifiers::Above => ".above".to_string(),
        NodeModifiers::NotAbove => ".!above".to_string(),

        NodeModifiers::Horizontal => ".horizontal".to_string(),
        NodeModifiers::NotHorizontal => ".!horizontal".to_string(),
        NodeModifiers::Vertical => ".vertical".to_string(),
        NodeModifiers::NotVertical => ".!vertical".to_string()
    };
    result
}

