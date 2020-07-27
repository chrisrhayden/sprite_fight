use std::collections::HashMap;

use crate::{
    components::{Position, Terrain},
    game_map::GameMap,
    tileset::SpriteCode,
};

pub fn move_to_system(
    game_map: &mut GameMap,
    position: &mut HashMap<usize, Position>,
    terrain: &HashMap<usize, Terrain>,
    entity_id: usize,
    from: (isize, isize),
    to: (isize, isize),
) {
    let column_count = game_map.map_info.column_count as isize;
    let row_count = game_map.map_info.row_count as isize;

    if to.0 < 0 || to.1 < 0 || to.0 >= column_count || to.1 >= row_count {
        println!("out of board");
        return;
    }

    let old_i = (from.0 + (column_count as isize * from.1)) as usize;
    let new_i = (to.0 + (column_count * to.1)) as usize;

    if game_map.render_map[new_i].visible {
        return;
    }

    for ents in position.values() {
        if ents.index == new_i {
            return;
        }
    }

    // we know it exists so unwrap should be fine
    position.get_mut(&entity_id).unwrap().index = new_i;

    // reset terrain in render map
    let terrain_id = game_map.terrain_map[old_i];
    let render_cell = &mut game_map.render_map[old_i];

    if let Some(terrain) = terrain.get(&terrain_id) {
        render_cell.visible = terrain.visible;
        render_cell.sprite_code = terrain.sprite_code;
    } else {
        render_cell.visible = false;
        render_cell.sprite_code = SpriteCode::NoSprite;
    }
}

// we cant borrow the scene as mut in the way we want so we borrow the parts we
// need
pub fn move_by_system(
    game_map: &mut GameMap,
    position: &mut HashMap<usize, Position>,
    terrain: &HashMap<usize, Terrain>,
    entity_id: usize,
    adjust: (isize, isize),
) {
    let old_i = if let Some(pos) = position.get(&entity_id) {
        pos.index
    } else {
        return;
    };

    let column_count = game_map.map_info.column_count;
    let row_count = game_map.map_info.row_count;

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

    if game_map.render_map[new_i].visible {
        return;
    }

    for ents in position.values() {
        if ents.index == new_i {
            return;
        }
    }

    // we know it exists so unwrap should be fine
    position.get_mut(&entity_id).unwrap().index = new_i;

    // reset terrain in render map
    let terrain_id = game_map.terrain_map[old_i];
    let render_cell = &mut game_map.render_map[old_i];

    if let Some(terrain) = terrain.get(&terrain_id) {
        render_cell.visible = terrain.visible;
        render_cell.sprite_code = terrain.sprite_code;
    } else {
        render_cell.visible = false;
        render_cell.sprite_code = SpriteCode::NoSprite;
    }
}
