// The following codes are used for convenience in creating game.
// We difine a structure to describe the game.
// All of the operations for game are implemented for the structure.

use gstd::*;
use pebble_game_io::*;

// This is the helper function offered from task instruction.
// Here we use it to create random selection for players.
pub fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

pub fn check_first_player() -> Player {
    if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    }
}

// Create pebbles by operating contract.
pub fn get_contract_pebbles(
    difficulty: DifficultyLevel,
    pebbles_count: u32,
    max_pebbles_per_turn: u32,
) -> u32 {
    match difficulty {
        DifficultyLevel::Easy => {
            let random_number = get_random_u32();
            (random_number % max_pebbles_per_turn + 1).min(pebbles_count)
        }
        DifficultyLevel::Hard => {
            let optimal_pebbles = pebbles_count % (max_pebbles_per_turn + 1);
            if optimal_pebbles == 0 {
                1
            } else {
                optimal_pebbles
            }
        }
    }
}

// Get the remainig of the game.
pub fn get_init_pebbles_remaining(
    difficulty: DifficultyLevel,
    pebbles_count: u32,
    max_pebbles_per_turn: u32,
    first_player: Player,
) -> u32 {
    let mut pebbles_remaining = pebbles_count;

    if first_player == Player::Program {
        let counter_pebbles = get_contract_pebbles(difficulty, pebbles_count, max_pebbles_per_turn);
        pebbles_remaining -= counter_pebbles;
        msg::reply(PebblesEvent::CounterTurn(counter_pebbles), 0).expect("Counter turn failed.");
    }

    pebbles_remaining
}
