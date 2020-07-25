use sdl2::{event::Event, keyboard::Keycode};

use crate::{scenes::Scene, tileset::SpriteCode, LoopState, WorldState};

fn move_by_system(scene: &mut Scene, entity_id: usize, adjust: (isize, isize)) {
    // TODO: we cant borrow mut and iter over all of the positions as immutable
    // but there should be a way
    let old_i = if let Some(pos) = scene.components.position.get(&entity_id) {
        pos.index
    } else {
        return;
    };

    let column_count = scene.game_map.map_info.column_count;
    let row_count = scene.game_map.map_info.row_count;

    let old_x = old_i % column_count;
    let old_y = old_i / column_count;

    let n_x = old_x as isize + adjust.0;
    let n_y = old_y as isize + adjust.1;

    if n_x < 0 || n_y < 0 {
        println!("out of board");
        return;
    }

    let n_x = n_x as usize;
    let n_y = n_y as usize;

    if n_x >= column_count || n_y >= row_count {
        println!("out of board");
        return;
    }

    let new_i = n_x + (column_count * n_y);

    if scene.game_map.render_map[new_i].visible {
        return;
    }

    for ents in scene.components.position.values() {
        if ents.index == new_i {
            return;
        }
    }

    // we know it exists so unwrap should be fine
    scene.components.position.get_mut(&entity_id).unwrap().index = new_i;

    // reset terrain in render map
    let terrain_id = scene.game_map.terrain_map[old_i];
    let render_cell = &mut scene.game_map.render_map[old_i];

    if let Some(terrain) = scene.components.terrain.get(&terrain_id) {
        render_cell.visible = terrain.visible;
        render_cell.sprite_code = terrain.sprite_code;
    } else {
        render_cell.visible = false;
        render_cell.sprite_code = SpriteCode::NoSprite;
    }
}

pub fn handle_events(world: &mut WorldState, evt: &Event) -> LoopState {
    match evt {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => LoopState::Quit,
        Event::KeyDown {
            keycode: Some(Keycode::Left),
            ..
        } => {
            let scene = world.scenes.get_current_scene_mut();
            move_by_system(scene, scene.player, (-1, 0));

            LoopState::Run
        }
        Event::KeyDown {
            keycode: Some(Keycode::Right),
            ..
        } => {
            let scene = world.scenes.get_current_scene_mut();
            move_by_system(scene, scene.player, (1, 0));

            LoopState::Run
        }

        Event::KeyDown {
            keycode: Some(Keycode::Down),
            ..
        } => {
            let scene = world.scenes.get_current_scene_mut();
            move_by_system(scene, scene.player, (0, 1));

            LoopState::Run
        }
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => {
            let scene = world.scenes.get_current_scene_mut();
            move_by_system(scene, scene.player, (0, -1));

            LoopState::Run
        }

        _ => LoopState::Run,
    }
}
