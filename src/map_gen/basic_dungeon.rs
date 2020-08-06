use std::cmp::{max, min};

use rand::prelude::*;

use crate::map_gen::generator::MapRect;

fn new_room(
    rng: &mut ThreadRng,
    min_room: usize,
    max_room: usize,
    column_count: usize,
    row_count: usize,
) -> MapRect {
    let w = rng.gen_range(min_room, max_room);
    let h = rng.gen_range(min_room, max_room);
    let x = rng.gen_range(0, column_count - w - 1);
    let y = rng.gen_range(0, row_count - h - 1);

    MapRect::new(x, y, w, h)
}

fn carve_room(room: &MapRect, char_map: &mut Vec<char>, column_count: usize) {
    for x in room.x1..=room.x2 {
        for y in room.y1..=room.y2 {
            let i = x + (column_count * y);

            if char_map[i] != 'Z' {
                char_map[i] = 'Z';
            }
        }
    }
}

fn carve_hallways(
    rng: &mut ThreadRng,
    past_room: &MapRect,
    cur_room: &MapRect,
    char_map: &mut Vec<char>,
    column_count: usize,
) {
    let column_count = column_count;
    let p_center = past_room.center();

    let c_center = cur_room.center();

    let (sx, sy) = if rng.gen() {
        (c_center.0, p_center.1)
    } else {
        (p_center.0, c_center.1)
    };

    let min_x = min(p_center.0, c_center.0);
    let max_x = max(p_center.0, c_center.0);

    for x in min_x..=max_x {
        let i = x + (column_count * sy);

        if char_map[i] != 'Z' {
            char_map[i] = 'Z';
        }
    }

    let min_y = min(p_center.1, c_center.1);
    let max_y = max(p_center.1, c_center.1);

    for y in min_y..=max_y {
        let i = sx + (column_count * y);

        if char_map[i] != 'Z' {
            char_map[i] = 'Z';
        }
    }
}

pub fn basic_gen(
    rng: &mut ThreadRng,
    column_count: usize,
    row_count: usize,
    total_tiles: usize,
) -> (Vec<char>, (usize, usize)) {
    let mut char_map: Vec<char> = vec![];

    for _ in 0..total_tiles {
        char_map.push('#');
    }

    let mut rooms = vec![];
    let max_trys = 50;

    let min_room = 3;
    let max_room = 5;

    let first = new_room(rng, min_room, max_room, column_count, row_count);

    carve_room(&first, &mut char_map, column_count);

    rooms.push(first);

    'rooms: for _ in 0..max_trys {
        let new_room =
            new_room(rng, min_room, max_room, column_count, row_count);

        for r in &rooms {
            if new_room.intersects(r) {
                continue 'rooms;
            }
        }

        carve_room(&new_room, &mut char_map, column_count);

        let past_room = rooms.last().unwrap();

        carve_hallways(rng, past_room, &new_room, &mut char_map, column_count);

        if rng.gen_ratio(2, 3) {
            let center = new_room.center();

            let i = center.0 + (column_count * center.1);

            char_map[i] = 'Z';
        }

        rooms.push(new_room);
    }

    (char_map, rooms.first().unwrap().center())
}
