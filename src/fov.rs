use crate::game_map::{GameMap, RenderCell};

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
    let index = view_point.0 + (game_map.map_info.column_count * view_point.1);

    game_map.render_map[index].lit = true;

    for region in 0..8 {
        let shadow_data = ShadowData {
            column_count: game_map.map_info.column_count as isize,
            row_count: game_map.map_info.row_count as isize,
            view_x: view_point.0 as isize,
            view_y: view_point.1 as isize,
            radius: 10,
            xx: MULT[0][region],
            xy: MULT[1][region],
            yx: MULT[2][region],
            yy: MULT[3][region],
        };

        recursive_shadowcasting(
            &mut game_map.render_map,
            &shadow_data,
            1,
            1.0f32,
            0.0f32,
        );
    }
}

fn recursive_shadowcasting(
    render_map: &mut Vec<RenderCell>,
    shadow_data: &ShadowData,
    row: isize,
    start: f32,
    end: f32,
) {
    let mut start = start;
    let radius_squer = shadow_data.radius * shadow_data.radius;

    let mut new_start = start;

    if start < end {
        return;
    }

    let mut blocked = false;

    for distance in row..=shadow_data.radius {
        let dx = -(distance as isize);
        let dy = -(distance as isize);

        for delta_x in dx..=0 {
            // make map cords
            // translate the dx, dy coordinates into map coordinates
            let x = shadow_data.view_x
                + delta_x * shadow_data.xx
                + dy * shadow_data.xy;

            let y = shadow_data.view_y
                + delta_x * shadow_data.yx
                + dy * shadow_data.yy;

            if x < 0
                || x >= shadow_data.column_count
                || y < 0
                || y >= shadow_data.row_count
            {
                continue;
            }

            let i = (x + (shadow_data.column_count * y)) as usize;

            let cell = &mut render_map[i];

            let l_slope = (dx as f32 - 0.5) / (dy as f32 + 0.5);
            let r_slope = (dx as f32 + 0.5) / (dy as f32 - 0.5);

            if start < r_slope {
                continue;
            }
            if end > l_slope {
                break;
            }

            if (delta_x * delta_x) + (dy * dy) < radius_squer {
                cell.lit = true;
                cell.visited = true;
            }

            if blocked {
                if cell.visible {
                    new_start = r_slope;
                    continue;
                } else {
                    blocked = false;
                    start = new_start;
                }
            } else {
                if cell.visible && distance < shadow_data.radius {
                    blocked = true;

                    // recurse here for more
                    recursive_shadowcasting(
                        render_map,
                        shadow_data,
                        distance + 1,
                        start,
                        l_slope,
                    );

                    new_start = r_slope;
                }
            }
        }

        if blocked {
            break;
        }
    }
}
