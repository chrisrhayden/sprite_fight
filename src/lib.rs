mod astar;
mod components;
pub mod config;
mod entitys;
mod fov;
mod game_map;
mod initialize;
mod map_gen;
mod scenes;
mod systems;
mod tileset;

// use std::cell::RefCell;
use std::error::Error;

use sdl2::{
    image::{LoadSurface, Sdl2ImageContext},
    pixels::Color,
    render::Canvas,
    surface::Surface,
    ttf::Sdl2TtfContext,
    video::Window,
    EventPump,
};

use components::ComponentStore;
use entitys::Entitys;
use fov::fov;
use game_map::MapInfo;
use map_gen::generator::{MapGen, MapType};
use scenes::{SceneBuilder, SceneManager};
use systems::{ai_system::ai_system, input_system::handle_events};
use tileset::{TileInfo, Tileset};

pub struct WindowInfo {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub map_start_x: i32,
}

// sdl context, mostly to keep parts of sdl alive
pub struct ContextManager {
    _ttf: Sdl2TtfContext,
    _img: Sdl2ImageContext,
    pub canvas: Canvas<Window>,
    pub events: EventPump,
}

// contain all the game data in one place
pub struct WorldState<'tex> {
    pub window_info: WindowInfo,
    pub entitys: Entitys,
    pub scenes: SceneManager,
    pub tileset: Tileset<'tex>,
}

#[derive(PartialEq, Debug)]
pub enum LoopState {
    Run,
    Quit,
    Wait,
}

pub fn run_game<'tex, 'font>(
    window_info: WindowInfo,
    tile_info: TileInfo,
    map_info: MapInfo,
) -> Result<(), Box<dyn Error>> {
    let font_path = "assets/ttf/unscii-16-full.ttf";

    let mut ctx = initialize::init_screen(&window_info)?;

    let texture_creator = ctx.canvas.texture_creator();

    let mut sprites = Surface::from_file(&tile_info.tile_path)?;

    sprites.set_color_key(true, Color::RGB(255, 0, 255))?;

    let sprite_texture =
        texture_creator.create_texture_from_surface(sprites)?;

    let tileset = Tileset::new(sprite_texture, tile_info);

    let mut font = initialize::init_font(&ctx._ttf, font_path, 18)?;

    let mut world = WorldState {
        entitys: Entitys::new(),
        scenes: SceneManager::new(),
        window_info,
        tileset,
    };

    let mut components = ComponentStore::default();

    let (game_map, center) = MapGen::new(MapType::Basic, map_info)
        .make_map(&mut components, &mut world.entitys);

    let scene_builder = SceneBuilder::new()
        .set_game_map(game_map)
        .set_components(components);

    let new_scene = world.scenes.register_scene(scene_builder);

    world.scenes.set_current_scene(new_scene);

    let entitys = &mut world.entitys;
    let scene = world.scenes.get_current_scene_mut();

    initialize::init_player(scene, entitys, center);

    for ent in scene.components.render.values() {
        let ent_ind = ent.index;
        let cell = &mut scene.game_map.render_map[ent_ind];

        cell.ent_size = ent.size;
        cell.ent_char = ent.reper_char;
        cell.visible = ent.visible;
    }

    fov(&mut scene.game_map, center);

    'main_game: loop {
        ctx.canvas.set_draw_color(Color::RGB(0, 0, 0));
        ctx.canvas.clear();

        let scene = world.scenes.get_current_scene_mut();

        for evt in ctx.events.poll_iter() {
            let loop_state = handle_events(scene, &evt);

            match loop_state {
                LoopState::Quit => break 'main_game,
                _ => {
                    scene.loop_state = loop_state;
                }
            }
        }

        if scene.loop_state == LoopState::Run {
            for cell in scene.game_map.render_map.iter_mut() {
                cell.lit = false;

                cell.ent_size = cell.terrain_size;
                cell.ent_char = cell.terrain_char;
                if cell.terrain_char != ' ' {
                    cell.visible = true;
                }
            }

            for ent in scene.components.render.values() {
                let cell = &mut scene.game_map.render_map[ent.index];

                cell.ent_char = ent.reper_char;
                cell.ent_size = ent.size;
                cell.visible = ent.visible;
            }

            let player_id = scene.player;

            let render_ent = scene.components.render.get(&player_id).unwrap();

            let cx = render_ent.index % scene.game_map.map_info.column_count;
            let cy = render_ent.index / scene.game_map.map_info.column_count;

            fov(&mut scene.game_map, (cx, cy));

            ai_system(scene);

            scene.loop_state = LoopState::Wait;
        }

        scene.render_scene(
            &texture_creator,
            &mut ctx.canvas,
            &mut font,
            &mut world.tileset,
            &world.window_info,
        )?;

        ctx.canvas.present();
    }

    Ok(())
}
