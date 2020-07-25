use std::collections::HashMap;
use std::error::Error;

use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::components::Position;
use crate::{
    components::ComponentStore, fov::fov, game_map::GameMap, tileset::Tileset,
    WindowInfo,
};

pub struct SceneBuilder {
    game_map: Option<GameMap>,
    components: Option<ComponentStore>,
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        Self {
            game_map: None,
            components: None,
        }
    }

    pub fn set_game_map(mut self, game_map: GameMap) -> Self {
        self.game_map = Some(game_map);

        self
    }

    pub fn set_components(mut self, components: ComponentStore) -> Self {
        self.components = Some(components);

        self
    }

    pub fn build(self) -> Scene {
        let game_map = if let Some(game_map) = self.game_map {
            game_map
        } else {
            panic!("need a game map");
        };

        let components = if let Some(components) = self.components {
            components
        } else {
            panic!("needs components");
        };

        Scene {
            components,
            game_map,
            player: 0,
        }
    }
}

pub struct Scene {
    pub game_map: GameMap,
    pub components: ComponentStore,
    pub player: usize,
}

pub struct SceneManager {
    pub next_id: usize,
    pub scenes: HashMap<usize, Scene>,
    pub current_scene: usize,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            next_id: 0,
            current_scene: 0,
        }
    }

    fn new_id(&mut self) -> usize {
        let id = self.next_id;

        self.next_id += 1;

        return id;
    }

    pub fn set_current_scene(&mut self, scene_id: usize) {
        if self.scenes.contains_key(&self.current_scene) {
            self.current_scene = scene_id;
        } else {
            // TODO: this is probably not what i want
            panic!("scene dose not exist");
        }
    }

    pub fn register_scene(&mut self, new_scene: SceneBuilder) -> usize {
        let scene = new_scene.build();

        let new_id = self.new_id();

        self.scenes.insert(new_id, scene);

        return new_id;
    }

    pub fn get_current_scene(&self) -> &Scene {
        if let Some(scene) = self.scenes.get(&self.current_scene) {
            scene
        } else {
            // TODO: this is probably not what i want
            panic!("scene dose not exist");
        }
    }

    pub fn get_current_scene_mut(&mut self) -> &mut Scene {
        if let Some(scene) = self.scenes.get_mut(&self.current_scene) {
            scene
        } else {
            panic!("scene dose not exist");
        }
    }

    pub fn render_scene(
        &mut self,
        canvas: &mut Canvas<Window>,
        tileset: &Tileset,
        _window_info: &WindowInfo,
    ) -> Result<(), Box<dyn Error>> {
        let scene = self.get_current_scene_mut();

        let tile_width = tileset.tile_info.width as i32;
        let tile_height = tileset.tile_info.height as i32;

        let column_count = scene.game_map.map_info.column_count;

        let start_x = 0;

        let mut x = start_x;
        let mut y = 0 - tile_height as i32;

        let mut dest_rect =
            Rect::new(x, y, tileset.tile_info.width, tileset.tile_info.height);

        let render_map = &mut scene.game_map.render_map;

        let mut player_pos = &Position { index: 0 };

        // add the entitys to the render_map
        for (key, pos) in scene.components.position.iter() {
            if let Some(ent) = scene.components.render.get(&key) {
                let cell = &mut render_map[pos.index];

                cell.visible = ent.visible;
                cell.sprite_code = ent.sprite_code;
            }

            if *key == scene.player {
                player_pos = pos;
            }
        }

        let cx = player_pos.index % 20;
        let cy = player_pos.index / 20;

        for c in render_map.iter_mut() {
            c.lit = false;
        }

        fov(render_map, (cx, cy));

        for (i, render_cell) in scene.game_map.render_map.iter().enumerate() {
            if i % column_count == 0 {
                x = start_x;
                y += tile_height;
            }

            if render_cell.lit || render_cell.visited {
                dest_rect.set_x(x);
                dest_rect.set_y(y);

                canvas.copy(
                    &tileset.texture,
                    tileset.tile_codes[render_cell.sprite_code as usize]
                        .to_owned(),
                    dest_rect,
                )?;
            }

            x += tile_width;
        }

        Ok(())
    }
}
