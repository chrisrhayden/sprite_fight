use std::collections::HashMap;

use crate::tileset::SpriteCode;

#[derive(Clone)]
pub struct Render {
    pub visible: bool,
    pub sprite_code: SpriteCode,
}

pub struct Terrain {
    pub index: usize,
    pub visible: bool,
    pub sprite_code: SpriteCode,
}

pub struct Position {
    pub index: usize,
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
    pub position: HashMap<usize, Position>,
    pub terrain: HashMap<usize, Terrain>,
    pub name: HashMap<usize, Name>,
    pub ai: HashMap<usize, Ai>,
}

impl Default for ComponentStore {
    fn default() -> Self {
        Self {
            health: HashMap::new(),
            render: HashMap::new(),
            selected: HashMap::new(),
            position: HashMap::new(),
            terrain: HashMap::new(),
            name: HashMap::new(),
            ai: HashMap::new(),
        }
    }
}
