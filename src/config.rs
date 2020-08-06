use std::path::PathBuf;

use crate::{game_map::MapInfo, tileset::TileInfo, WindowInfo};

pub fn make_game_info() -> (WindowInfo, TileInfo, MapInfo) {
    let map_cols = 30;
    let map_rows = 30;

    let total_tiles = 256;
    let tile_col_count = 16;
    let orig_w = 10;
    let orig_h = 10;
    let tile_path = PathBuf::from("assets/png/Potash_10x10.png");

    let tile_width = orig_w * 2;
    let tile_heigh = orig_h * 2;

    let window_info = WindowInfo {
        width: (tile_width * map_cols) + 250,
        height: tile_heigh * map_rows,
        name: String::from("rend"),
        map_start_x: 0,
    };

    let tile_info = TileInfo {
        tile_path,
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
