use sdl2::{event::Event, keyboard::Keycode};

use crate::{scenes::Scene, LoopState};

use super::move_system::move_by_system;

fn move_player(scene: &mut Scene, to_move: (isize, isize)) -> LoopState {
    move_by_system(
        &mut scene.game_map,
        &mut scene.components.position,
        &scene.components.terrain,
        scene.player,
        to_move,
    );

    LoopState::Run
}

fn handle_wait(scene: &mut Scene, evt: &Event) -> LoopState {
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
            move_player(scene, (-1, 0));

            LoopState::Run
        }
        Event::KeyDown {
            keycode: Some(Keycode::Right),
            ..
        } => {
            move_player(scene, (1, 0));

            LoopState::Run
        }

        Event::KeyDown {
            keycode: Some(Keycode::Down),
            ..
        } => {
            move_player(scene, (0, 1));

            LoopState::Run
        }
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => {
            move_player(scene, (0, -1));

            LoopState::Run
        }

        _ => LoopState::Wait,
    }
}

pub fn handle_events(scene: &mut Scene, evt: &Event) -> LoopState {
    match scene.loop_state {
        LoopState::Wait => handle_wait(scene, evt),
        _ => LoopState::Wait,
    }
}
