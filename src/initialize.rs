use std::error::Error;

use sdl2::{
    pixels::Color,
    ttf::{self, Font, Sdl2TtfContext},
};

use crate::{
    components::{EntitySize, Health, Name, Render},
    entitys::Entitys,
    scenes::Scene,
    ContextManager, WindowInfo,
};

pub fn init_screen(
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

pub fn init_font<'ttf, 'r>(
    ttf: &'ttf Sdl2TtfContext,
    font_path: &str,
    font_point: u16,
) -> Result<Font<'ttf, 'r>, Box<dyn Error>> {
    let font = ttf.load_font(font_path, font_point)?;

    Ok(font)
}

pub fn init_player(
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
