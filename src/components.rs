use std::collections::HashMap;

use crate::tileset::SpriteCode;

pub trait Component {
    fn get_id(&self) -> usize;
}

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
        }
    }
}
