mod components;
mod entitys;
mod fov;
mod game_map;
mod map_gen;
mod render;
mod scenes;
mod systems;
mod tileset;

use std::error::Error;

use sdl2::{
    image::LoadSurface,
    image::Sdl2ImageContext,
    pixels::Color,
    render::{Canvas, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
    EventPump,
};

use components::ComponentStore;
use entitys::Entitys;
use game_map::MapInfo;
use map_gen::generator::{MapGen, MapType};
use scenes::{SceneBuilder, SceneManager};
use systems::input_system::handle_events;
use tileset::{TileInfo, Tileset};

pub struct WindowInfo {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

// sdl context, mostly to keep parts of sdl alive
pub struct ContextManager {
    _img: Sdl2ImageContext,
    pub canvas: Canvas<Window>,
    pub events: EventPump,
}

// contain all the game data in one place
pub struct WorldState<'t> {
    pub window_info: WindowInfo,
    pub entitys: Entitys,
    pub tileset: Tileset<'t>,
    pub scenes: SceneManager,
}

pub enum LoopState {
    Run,
    Quit,
}

fn init_screen(
    window_info: &WindowInfo,
) -> Result<ContextManager, Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(&window_info.name, window_info.width, window_info.height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build()?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let events = sdl_context.event_pump()?;

    let img_int_flags = sdl2::image::InitFlag::PNG;

    let _img = sdl2::image::init(img_int_flags)?;

    let ctx = ContextManager {
        canvas,
        events,
        _img,
    };

    return Ok(ctx);
}

fn init_texture<'t>(
    texture_creator: &'t TextureCreator<WindowContext>,
    texture_path: &str,
) -> Result<Tileset<'t>, Box<dyn Error>> {
    let mut surface = Surface::from_file(texture_path)?;

    surface.set_color_key(true, Color::RGB(0, 0, 0))?;

    let texture = surface.as_texture(&texture_creator)?;

    let tile_map = TileInfo {
        total_count: 236,
        row_count: 12,
        width: 16,
        height: 16,
    };

    let tileset = Tileset::new(texture, tile_map)?;

    Ok(tileset)
}

pub fn run_game() -> Result<(), Box<dyn Error>> {
    let window_info = WindowInfo {
        width: 320,
        height: 320,
        name: String::from("rend"),
    };

    let texture_path = "assets/png/sprites.png";

    let mut ctx = init_screen(&window_info)?;

    let texture_creator = ctx.canvas.texture_creator();

    let mut world = WorldState {
        entitys: Entitys::new(),
        scenes: SceneManager::new(),
        tileset: init_texture(&texture_creator, texture_path)?,
        window_info,
    };

    let map_info = MapInfo {
        column_count: 20,
        row_count: 20,
        total_count: 20 * 20,
    };

    let mut components = ComponentStore::default();

    let (game_map, center) = MapGen::new(MapType::Basic, map_info)
        .make_map(&mut components, &mut world.entitys);

    let scene_builder = SceneBuilder::new()
        .set_game_map(game_map)
        .set_components(components);

    let scene_id = world.scenes.register_scene(scene_builder);

    world.scenes.set_current_scene(scene_id);

    {
        let scene = world.scenes.get_current_scene_mut();

        let player_id = world.entitys.new_id();

        let index =
            center.0 + (scene.game_map.map_info.column_count * center.1);

        scene.player = player_id;

        scene
            .components
            .position
            .insert(player_id, components::Position { index });

        scene.components.render.insert(
            player_id,
            components::Render {
                visible: true,
                sprite_code: tileset::SpriteCode::Charf1,
            },
        );

        let r_cell = &mut scene.game_map.render_map[index];
        r_cell.visible = true;
        r_cell.sprite_code = tileset::SpriteCode::Charf1;
    }

    'main_game: loop {
        ctx.canvas.set_draw_color(Color::RGB(0, 0, 0));
        ctx.canvas.clear();

        for evt in ctx.events.poll_iter() {
            match handle_events(&mut world, &evt) {
                LoopState::Quit => break 'main_game,
                _ => {}
            }
        }

        world.scenes.render_scene(
            &mut ctx.canvas,
            &world.tileset,
            &world.window_info,
        )?;

        ctx.canvas.present();
    }

    Ok(())
}
