use std::collections::HashMap;
use std::error::Error;

use sdl2::{
    pixels::Color,
    rect::Rect,
    render::Canvas,
    render::TextureCreator,
    ttf::Font,
    video::{Window, WindowContext},
};

use crate::{
    components::ComponentStore, game_map::GameMap, tileset::Tileset, LoopState,
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
            loop_state: LoopState::Wait,
        }
    }
}

pub struct Scene {
    pub game_map: GameMap,
    pub components: ComponentStore,
    pub player: usize,
    pub loop_state: LoopState,
}

impl Scene {
    fn render_map<'t>(
        &mut self,
        canvas: &mut Canvas<Window>,
        tileset: &mut Tileset<'t>,
        column_count: usize,
        start_x: i32,
    ) -> Result<(), Box<dyn Error>> {
        let tile_width = tileset.tile_info.width as i32;
        let tile_height = tileset.tile_info.height as i32;

        let mut x: i32 = start_x;

        let mut y: i32 = 0 - tile_height;

        let mut dest_rect = Rect::new(
            x as i32,
            y as i32,
            tileset.tile_info.width,
            tileset.tile_info.height,
        );

        tileset.texture.set_color_mod(250, 250, 250);

        for (i, render_cell) in self.game_map.render_map.iter().enumerate() {
            if i % column_count == 0 {
                x = start_x;
                y += tile_height;
            }

            dest_rect.set_x(x);
            dest_rect.set_y(y);

            if render_cell.lit {
                canvas.set_draw_color(Color::RGB(10, 10, 50));
                canvas.fill_rect(dest_rect)?;

                if render_cell.ent_char != ' ' {
                    let char_rect = tileset.get_char(render_cell.ent_char);

                    canvas.copy(&tileset.texture, *char_rect, dest_rect)?;
                }
            }

            x += tile_width;
        }

        Ok(())
    }

    fn render_ui(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        canvas: &mut Canvas<Window>,
        font: &mut Font,
        start_x: i32,
        start_y: i32,
    ) -> Result<(), Box<dyn Error>> {
        let text = String::from("health");

        let f_surface = font.render(&text).blended(Color::RGB(1, 1, 1))?;

        let text_width = f_surface.width();
        let text_height = f_surface.height();

        let percent =
            if let Some(health) = self.components.health.get(&self.player) {
                let max = health.max_value;
                let cur = health.cur_value;

                let percent = (max / cur) * 100;

                percent as u32
            } else {
                return Ok(());
            };

        let max_bar_value = 240;

        let max_bar =
            Rect::new(start_x, start_y, max_bar_value, text_height + 4);

        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.fill_rect(max_bar)?;

        let percent = (percent * max_bar_value) / 100;

        let percent_bar = Rect::new(start_x, start_y, percent, text_height + 4);

        canvas.set_draw_color(Color::RGB(200, 50, 50));
        canvas.fill_rect(percent_bar)?;

        let text = texture_creator.create_texture_from_surface(&f_surface)?;

        let text_dist_rect =
            Rect::new(start_x, start_y, text_width, text_height);

        canvas.copy(&text, None, text_dist_rect)?;

        Ok(())
    }

    pub fn render_scene<'t>(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        canvas: &mut Canvas<Window>,
        font: &mut Font,
        tileset: &mut Tileset<'t>,
        window_info: &WindowInfo,
    ) -> Result<(), Box<dyn Error>> {
        let start_x = window_info.map_start_x;

        let column_count = self.game_map.map_info.column_count;
        let tile_width = tileset.tile_info.width as i32;

        self.render_map(canvas, tileset, column_count, start_x)?;

        let ui_start_x = start_x + (column_count as i32 * tile_width) + 5;

        let ui_start_y = 5;

        self.render_ui(texture_creator, canvas, font, ui_start_x, ui_start_y)?;

        Ok(())
    }
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
}
