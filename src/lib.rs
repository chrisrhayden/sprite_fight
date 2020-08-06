mod astar;
mod components;
mod entitys;
mod fov;
mod game_map;
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
    ttf::{self, Font, Sdl2TtfContext},
    video::Window,
    EventPump,
};

use components::{ComponentStore, EntitySize, Health, Name, Render};
use entitys::Entitys;
use fov::fov;
use game_map::MapInfo;
use map_gen::generator::{MapGen, MapType};
use scenes::{Scene, SceneBuilder, SceneManager};
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

fn init_screen(
    window_info: &WindowInfo,
) -> Result<ContextManager, Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(&window_info.name, window_info.width, window_info.height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build()?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let events = sdl_context.event_pump()?;

    let img_int_flags = sdl2::image::InitFlag::PNG;

    let _img = sdl2::image::init(img_int_flags)?;

    let _ttf = ttf::init()?;

    let ctx = ContextManager {
        _ttf,
        canvas,
        events,
        _img,
    };

    return Ok(ctx);
}

fn init_font<'ttf, 'r>(
    ttf: &'ttf Sdl2TtfContext,
    font_path: &str,
    font_point: u16,
) -> Result<Font<'ttf, 'r>, Box<dyn Error>> {
    let font = ttf.load_font(font_path, font_point)?;

    Ok(font)
}

fn init_player(
    scene: &mut Scene,
    entitys: &mut Entitys,
    center: (usize, usize),
) {
    let player_id = entitys.new_id();

    let index = center.0 + (scene.game_map.map_info.column_count * center.1);

    scene.player = player_id;

    scene.components.name.insert(
        player_id,
        Name {
            value: "test player".to_string(),
        },
    );

    scene.components.health.insert(
        player_id,
        Health {
            max_value: 10,
            cur_value: 10,
        },
    );

    scene.components.render.insert(
        player_id,
        Render {
            index,
            reper_char: '@',
            size: EntitySize::Medium,
            visible: true,
        },
    );
}

pub fn run_game<'tex, 'font>(
    window_info: WindowInfo,
    tile_info: TileInfo,
    map_info: MapInfo,
) -> Result<(), Box<dyn Error>> {
    let font_path = "assets/ttf/unscii-16-full.ttf";
    let texture_path = "assets/png/Potash_10x10.png";

    let mut ctx = init_screen(&window_info)?;

    let texture_creator = ctx.canvas.texture_creator();

    let sprites = Surface::from_file(texture_path)?;

    let sprite_texture =
        texture_creator.create_texture_from_surface(sprites)?;

    let tileset = Tileset::new(sprite_texture, tile_info);

    let mut font = init_font(&ctx._ttf, font_path, 18)?;

    // let (mut world, center) = init_world(tileset, window_info, map_info);

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

    let scene_id = world.scenes.register_scene(scene_builder);

    world.scenes.set_current_scene(scene_id);

    let entitys = &mut world.entitys;
    let scene = world.scenes.get_current_scene_mut();

    init_player(scene, entitys, center);

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

pub fn make_game_info() -> (WindowInfo, TileInfo, MapInfo) {
    let map_cols = 30;
    let map_rows = 30;

    let total_tiles = 256;
    let tile_col_count = 16;
    let orig_w = 10;
    let orig_h = 10;

    let tile_width = orig_w * 2;
    let tile_heigh = orig_h * 2;

    let window_info = WindowInfo {
        width: (tile_width * map_cols) + 250,
        height: tile_heigh * map_rows,
        name: String::from("rend"),
        map_start_x: 0,
    };

    let tile_info = TileInfo {
        orig_w,
        orig_h,
        width: tile_width,
        height: tile_heigh,
        col_count: tile_col_count,
        total_count: total_tiles,
    };

    let map_cols = map_cols as usize;
    let map_rows = map_rows as usize;

    let map_info = MapInfo {
        column_count: map_cols,
        row_count: map_rows,
        total_count: map_cols * map_rows,
    };

    (window_info, tile_info, map_info)
}
