use crate::bspc::modifier::Modifier;

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

    SameClass,
    NotSameClass,
    DescendantOf,
    NotDescendantOf,
    AncestorOf,
    NotAncestorOf,

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

impl Modifier for NodeModifier {
    fn get_string(&self) -> String {
        String::from(match self {
            NodeModifier::Id(id) => id,

            NodeModifier::Focused => ".focused",
            NodeModifier::NotFocused => ".!focused",
            NodeModifier::Active => ".active",
            NodeModifier::NotActive => ".!active",
            NodeModifier::Atuomatic => ".automatic",
            NodeModifier::NotAutomatic => ".!automatic",
            NodeModifier::Local => ".local",
            NodeModifier::NotLocal => ".!local",
            NodeModifier::Leaf => ".leaf",
            NodeModifier::NotLeaf => ".!leaf",
            NodeModifier::Window => ".window",
            NodeModifier::NotWindow => ".!window",

            NodeModifier::Tiled => ".tiled",
            NodeModifier::NotTiled => ".!tiled",
            NodeModifier::PseudoTiled => ".pseudo_tiled",
            NodeModifier::NotPseudoTiled => ".!pseudo_tiled",
            NodeModifier::Floating => ".floating",
            NodeModifier::NotFloating => ".!floating",
            NodeModifier::Fullscreen => ".fullscreen",
            NodeModifier::NotFullscreen => ".!fullscreen",

            NodeModifier::SameClass => ".same_class",
            NodeModifier::NotSameClass => ".!same_class",
            NodeModifier::DescendantOf => ".decendant_of",
            NodeModifier::NotDescendantOf => ".!decendant_of",
            NodeModifier::AncestorOf => ".ancestor_of",
            NodeModifier::NotAncestorOf => ".!ancestor_of",

            NodeModifier::Hidden => ".hidden",
            NodeModifier::NotHidden => ".!hidden",
            NodeModifier::Sticky => ".sticky",
            NodeModifier::NotSticky => ".!sticky",
            NodeModifier::Private => ".private",
            NodeModifier::NotPrivate => ".!private",
            NodeModifier::Locked => ".locked",
            NodeModifier::NotLocked => ".!locked",
            NodeModifier::Marked => ".marked",
            NodeModifier::NotMarked => ".!marked",
            NodeModifier::Urgent => ".urgent",
            NodeModifier::NotUrgent => ".!urgent",

            NodeModifier::Below => ".below",
            NodeModifier::NotBelow => ".!below",
            NodeModifier::Normal => ".normal",
            NodeModifier::NotNormal => ".!normal",
            NodeModifier::Above => ".above",
            NodeModifier::NotAbove => ".!above",

            NodeModifier::Horizontal => ".horizontal",
            NodeModifier::NotHorizontal => ".!horizontal",
            NodeModifier::Vertical => ".vertical",
            NodeModifier::NotVertical => ".!vertical"
        })
    }
}
