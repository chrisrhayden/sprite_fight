use crate::{
    components::ComponentStore,
    entitys::Entitys,
    game_map::{GameMap, MapInfo},
    map_gen::basic_dungeon::basic_gen,
};

pub struct MapRect {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

impl MapRect {
    pub fn new(x1: usize, y1: usize, w: usize, h: usize) -> Self {
        Self {
            x1,
            y1,
            x2: x1 + w,
            y2: y1 + h,
        }
    }

    pub fn center(&self) -> (usize, usize) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn intersects(&self, other: &MapRect) -> bool {
        self.x1 <= other.x2
            && self.x2 >= other.x1
            && self.y1 <= other.y2
            && self.y2 >= other.y1
    }
}

pub enum MapType {
    Basic,
}

pub struct MapGen {
    pub map_type: MapType,
    pub map_info: MapInfo,
}

impl MapGen {
    pub fn new(map_type: MapType, map_info: MapInfo) -> Self {
        Self { map_type, map_info }
    }

    pub fn make_map(
        &self,
        components: &mut ComponentStore,
        entitys: &mut Entitys,
    ) -> (GameMap, (usize, usize)) {
        let mut rng = rand::thread_rng();

        let mut game_map = GameMap::new(self.map_info.clone());

        let (sprite_map, center) = match self.map_type {
            MapType::Basic => basic_gen(
                &mut rng,
                game_map.map_info.column_count,
                game_map.map_info.row_count,
                game_map.map_info.total_count,
            ),
        };

        game_map.init_map(components, entitys, sprite_map);

        (game_map, center)
    }
}
