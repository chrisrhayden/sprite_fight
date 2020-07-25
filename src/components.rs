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

pub struct Health {
    pub value: isize,
}

pub struct ComponentStore {
    pub health: HashMap<usize, Health>,
    pub render: HashMap<usize, Render>,
    pub selected: HashMap<usize, Selected>,
    pub position: HashMap<usize, Position>,
    pub terrain: HashMap<usize, Terrain>,
}

impl Default for ComponentStore {
    fn default() -> Self {
        Self {
            health: HashMap::new(),
            render: HashMap::new(),
            selected: HashMap::new(),
            position: HashMap::new(),
            terrain: HashMap::new(),
        }
    }
}
