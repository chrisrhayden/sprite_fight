use crate::{astar::astar, scenes::Scene};

use super::move_system::{move_by_system, move_to_system};

#[allow(dead_code)]
fn basic_movement(scene: &mut Scene, ai_id: usize) {
    let index = scene.components.render.get(&scene.player).unwrap().index;

    let ai_pos_index = if let Some(ai_pos) = scene.components.render.get(&ai_id)
    {
        ai_pos.index
    } else {
        println!("ai has no position");
        return;
    };

    if scene.game_map.render_map[ai_pos_index].lit {
        let cols = scene.game_map.map_info.column_count;
        let player_x = (index % cols) as isize;
        let player_y = (index / cols) as isize;

        let ai_x = (ai_pos_index % cols) as isize;
        let ai_y = (ai_pos_index / cols) as isize;

        let distance_x = (player_x - ai_x) as isize;
        let distance_y = (player_y - ai_y) as isize;

        let distance = ((distance_x.pow(2) + distance_y.pow(2)) as f64).sqrt();

        let dx = (distance_x as f64 / distance).round() as isize;
        let dy = (distance_y as f64 / distance).round() as isize;

        move_by_system(
            &mut scene.game_map,
            &mut scene.components.render,
            ai_id.to_owned(),
            (dx, dy),
        );
    }
}

pub fn ai_system(scene: &mut Scene) {
    let column_count = scene.game_map.map_info.column_count;

    let player_index =
        scene.components.render.get(&scene.player).unwrap().index;

    let player_x = (player_index % column_count) as isize;
    let player_y = (player_index / column_count) as isize;

    for ai_id in scene.components.ai.keys() {
        if let Some(ai_ent_render) = scene.components.render.get(ai_id) {
            if scene.game_map.render_map[ai_ent_render.index].lit {
                let ai_x = (ai_ent_render.index % column_count) as isize;
                let ai_y = (ai_ent_render.index / column_count) as isize;

                let path = match astar(
                    &mut scene.game_map,
                    (ai_x, ai_y),
                    (player_x, player_y),
                ) {
                    Ok(path) => path,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };

                if let Some(path) = path {
                    let first = path.first().unwrap().clone();

                    if first != (player_x, player_y) {
                        continue;
                    }

                    let last =
                        path.iter().rev().skip(1).take(1).next().unwrap();

                    move_to_system(
                        &mut scene.game_map,
                        &mut scene.components.render,
                        *ai_id,
                        *last,
                    );
                } else {
                    // no path to take
                }
            }
        }
    }
}
