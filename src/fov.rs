use crate::{components::EntitySize, game_map::GameMap};

const MULT: [[isize; 8]; 4] = [
    [1, 0, 0, -1, -1, 0, 0, 1],
    [0, 1, -1, 0, 0, -1, 1, 0],
    [0, 1, 1, 0, 0, -1, -1, 0],
    [1, 0, 0, 1, -1, 0, 0, -1],
];

struct ShadowData {
    radius: isize,
    xx: isize,
    xy: isize,
    yx: isize,
    yy: isize,
    view_x: isize,
    view_y: isize,
    column_count: isize,
    row_count: isize,
}

pub fn fov(game_map: &mut GameMap, view_point: (usize, usize)) {
    let ind =
        view_point.0 + (game_map.map_info.column_count as usize * view_point.1);

    game_map.render_map[ind].lit = true;

    for region in 0..8 {
        let mut shadow_data = ShadowData {
            column_count: game_map.map_info.column_count as isize,
            row_count: game_map.map_info.row_count as isize,
            view_x: view_point.0 as isize,
            view_y: view_point.1 as isize,
            radius: 7,
            xx: MULT[0][region],
            xy: MULT[1][region],
            yx: MULT[2][region],
            yy: MULT[3][region],
        };

        recursive_shadowcasting(game_map, &mut shadow_data, 1, 1.0f64, 0.0f64);
    }
}

fn recursive_shadowcasting(
    game_map: &mut GameMap,
    shadow_data: &mut ShadowData,
    row: isize,
    start: f64,
    end: f64,
) {
    if start < end {
        return;
    }

    let mut left_view_slope = start;

    let view_radius_square =
        shadow_data.radius as f64 * shadow_data.radius as f64;

    let view_ceiling = shadow_data.radius;

    let mut prev_was_blocked = false;

    let mut saved_right_slope = -1.0f64;

    let map_width = shadow_data.column_count;
    let map_height = shadow_data.row_count;

    // move along the columns / x axis
    for cur_col in row..=view_ceiling {
        let yc = -(cur_col);

        // move down the rows / y axis
        for xc in yc..=0 {
            let grid_x =
                shadow_data.view_x + xc * shadow_data.xx + yc * shadow_data.xy;

            let grid_y =
                shadow_data.view_y + xc * shadow_data.yx + yc * shadow_data.yy;

            let left_block_slope = (xc as f64 - 0.5) / (yc as f64 + 0.5);
            let right_block_slope = (xc as f64 + 0.5) / (yc as f64 - 0.5);

            if grid_x < 0
                || grid_x >= map_width
                || grid_y < 0
                || grid_y >= map_height
            {
                continue;
            }

            if right_block_slope > left_view_slope {
                // block is above the left edge if our view, skip
                continue;
            } else if left_block_slope < end {
                // block is below th right edge of our view area, were done
                break;
            }

            let distance_squer = ((xc * xc) + (yc * yc)) as f64;

            let cel_ind = (grid_x + (map_width * grid_y)) as usize;

            if distance_squer <= view_radius_square {
                let cell = &mut game_map.render_map[cel_ind];

                cell.lit = true;
            }

            let cur_blocked =
                game_map.render_map[cel_ind].ent_size > EntitySize::Small;

            if prev_was_blocked {
                if cur_blocked {
                    saved_right_slope = right_block_slope;
                } else {
                    prev_was_blocked = false;
                    left_view_slope = saved_right_slope;
                }
            } else {
                if cur_blocked && cur_col < shadow_data.radius {
                    recursive_shadowcasting(
                        game_map,
                        shadow_data,
                        cur_col + 1,
                        left_view_slope,
                        left_block_slope,
                    );

                    prev_was_blocked = true;

                    saved_right_slope = right_block_slope;
                }
            }
        }

        if prev_was_blocked {
            break;
        }
    }
}
