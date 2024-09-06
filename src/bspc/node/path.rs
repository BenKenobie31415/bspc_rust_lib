use std::{borrow::BorrowMut, collections::VecDeque};

use crate::bspc::desktop::selector::DesktopSelector;

use super::direction::Direction;

pub struct Path {
    pub desktop_selector: Option<DesktopSelector>,
    pub jumps: Vec<Jump>,
    pub absolute: bool,
}

impl Path {
    pub fn new() -> Path {
        Path {
            desktop_selector: None,
            jumps: Vec::new(),
            absolute: false,
        }
    }

    pub fn set_desktop_selector(mut self, desktop_selector: DesktopSelector) -> Self {
        self.desktop_selector = Some(desktop_selector);
        return self;
    }

    pub fn set_absolute(mut self, absolute: bool) -> Self {
        self.absolute = absolute;
        return self;
    }

    pub fn add_jump(mut self, jump: Jump) -> Self {
        self.jumps.push(jump);
        return self;
    }

    pub fn assemble(&self) -> String {
        let mut result = String::from("@");

        match &self.desktop_selector {
            Some(selector) => {
                result.push_str(&selector.assemble());
                result.push_str(":");
            }
            None => ()
        }

        for i in 0..self.jumps.len() {
            let next_jump = &self.jumps[i];
            if i == 0 && self.absolute || i != 0 {
                result.push_str("/");
            }
            result.push_str(&format!("{}", next_jump.get_string()));
        }
        if self.jumps.len() == 0 && self.absolute {
            result.push_str("/");
        }
        return result;
    }
}

pub enum Jump {
    First,
    Second,
    Brother,
    Parent,
    //TODO not entirely clear how to use this correctly
    Dir(Direction),
}

impl Jump {
    pub fn get_string(&self) -> String {
        String::from(match self {
            Jump::First => "first".to_string(),
            Jump::Second => "second".to_string(),
            Jump::Brother => "brother".to_string(),
            Jump::Parent => "parent".to_string(),
            Jump::Dir(dir) => dir.get_string(),
        })
    }
}
