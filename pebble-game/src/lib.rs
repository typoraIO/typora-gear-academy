#![no_std]
#![allow(warnings)]
use core::panic;

use auxilar::get_init_pebbles_remaining;
use gstd::*;
use pebble_game_io::*;

pub mod auxilar;

static mut PEBBLE_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {
    let config: PebblesInit = msg::load().expect("Init Error.");

    let first_player = auxilar::check_first_player();

    let init_game = auxilar::get_init_pebbles_remaining(
        config.difficulty,
        config.pebbles_count,
        config.max_pebbles_per_turn,
        first_player.clone(),
    );

    let game = GameState {
        pebbles_count: config.pebbles_count,
        max_pebbles_per_turn: config.max_pebbles_per_turn,
        pebbles_remaining: init_game,
        difficulty: config.difficulty,
        first_player,
        winner: None,
    };
    unsafe { PEBBLE_GAME = Some(game) };
}

#[no_mangle]
extern "C" fn handle() {
    let action: PebblesAction = msg::load().expect("Get PebbleAction Error.");

    let mut game_state = unsafe { PEBBLE_GAME.clone().expect("Get state failed.") };
    match action {
        PebblesAction::Turn(pebbles_got) => {
            if pebbles_got > game_state.max_pebbles_per_turn || pebbles_got == 0 {
                panic!("Invalid input, please try again.");
            }

            if pebbles_got > game_state.pebbles_remaining {
                panic!("Remainig is not enough, please try again.");
            }

            game_state.pebbles_remaining -= pebbles_got;

            if game_state.pebbles_remaining == 0 {
                game_state.winner = Some(Player::User);
                msg::reply(PebblesEvent::Won(Player::User), 0).expect("Failed to turn to Won.");
            } else {
                let counter_pebbles = auxilar::get_contract_pebbles(
                    game_state.difficulty,
                    game_state.pebbles_count,
                    game_state.max_pebbles_per_turn,
                );
                game_state.pebbles_remaining -= counter_pebbles;
                if game_state.pebbles_remaining == 0 {
                    game_state.winner = Some(Player::Program);
                    msg::reply(PebblesEvent::Won(Player::Program), 0)
                        .expect("Failed to turn to Won.");
                } else {
                    msg::reply(PebblesEvent::CounterTurn(counter_pebbles), 0)
                        .expect("Failed to turn to CounterTurn.");
                }
            }
        }
        PebblesAction::GiveUp => {
            game_state.winner = Some(Player::Program);
            msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Failed to turn to Won.");
        }
        PebblesAction::Restart {
            difficulty,
            pebbles_count,
            max_pebbles_per_turn,
        } => {
            let first_player = auxilar::check_first_player();
            let pebbles_remaining = auxilar::get_init_pebbles_remaining(
                difficulty,
                pebbles_count,
                max_pebbles_per_turn,
                first_player.clone(),
            );
            game_state = GameState {
                pebbles_count,
                max_pebbles_per_turn,
                pebbles_remaining,
                difficulty,
                first_player,
                winner: None,
            };
        }
    }

    unsafe {
        PEBBLE_GAME = Some(game_state);
    }
}

#[no_mangle]
extern "C" fn state() {
    let game_state = unsafe { PEBBLE_GAME.clone().expect("Get state failed.") };

    msg::reply(game_state, 0).expect("State reply failed");
}
