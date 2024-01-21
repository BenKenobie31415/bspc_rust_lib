use crate::bspc::desktop::selection::DesktopSelector;

pub struct Path {
    pub desktop_selector: Option<DesktopSelector>,
    pub jumps: Vec<Jump>
}

impl Path {
    pub fn new() -> Path {
        Path {
            desktop_selector: None,
            jumps: Vec::new()
        }
    }

    pub fn set_desktop_selector(mut self, desktop_selector: DesktopSelector) -> Self {
        self.desktop_selector = Some(desktop_selector);
        self
    }

    pub fn add_jump(mut self, jump: Jump) -> Self {
        self.jumps.push(jump);
        self
    }

    pub fn assemble(&self) -> String {
        let mut result = String::from("@");

        match &self.desktop_selector {
            Some(selector) => result.push_str(&selector.assemble()),
            None => ()
        }

        for jump in &self.jumps {
            result.push_str(&format!("/{}", jump.get_string()));
        }
        result
    }
}

pub enum Jump {
    First,
    Second,
    Brother,
    Parent
}

impl Jump {
    pub fn get_string(&self) -> String {
        String::from(match self {
            Jump::First => "first",
            Jump::Second => "second",
            Jump::Brother => "brother",
            Jump::Parent => "parent"
        })
    }
}
