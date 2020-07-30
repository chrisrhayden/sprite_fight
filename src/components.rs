use std::collections::HashMap;

use crate::tileset::SpriteCode;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum EntitySize {
    Nothing,
    Small,
    Medium,
    Large,
}

#[derive(Clone)]
pub struct Render {
    pub size: EntitySize,
    pub index: usize,
    pub visible: bool,
    pub sprite_code: SpriteCode,
}

pub struct Selected {
    pub entity: Option<usize>,
}

pub enum AiType {
    Basic,
}

pub struct Ai {
    pub ai_type: AiType,
}

pub struct Name {
    pub value: String,
}

pub struct Health {
    pub max_value: isize,
    pub cur_value: isize,
}

pub struct ComponentStore {
    pub health: HashMap<usize, Health>,
    pub render: HashMap<usize, Render>,
    pub selected: HashMap<usize, Selected>,
    pub name: HashMap<usize, Name>,
    pub ai: HashMap<usize, Ai>,
}

impl Default for ComponentStore {
    fn default() -> Self {
        Self {
            health: HashMap::new(),
            render: HashMap::new(),
            selected: HashMap::new(),
            name: HashMap::new(),
            ai: HashMap::new(),
        }
    }
}
