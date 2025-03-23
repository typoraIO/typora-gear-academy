#![allow(warnings)]
use gtest::{Program, System};
use pebble_game_io::*;

const ID: u64 = 50;
const VALUE: u128 = 100000000000000000;

#[test]
fn test_with_positive_input() {
    let system = System::new();
    let program = Program::current(&system);
    let pid = program.id();

    system.mint_to(ID, VALUE);
    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 4,
    };

    program.send(ID, init_message);
    system.run_next_block();
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the initial state of the game");

    assert!(state.pebbles_remaining <= 10);
    assert_eq!(state.pebbles_count, 10);
    assert_eq!(state.max_pebbles_per_turn, 4);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);
    assert_eq!(state.winner, None::<Player>);

    program.send(ID, PebblesAction::Turn(3));
    system.run_next_block();
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after user's turn");

    assert!(state.pebbles_remaining < 7);

    program.send(ID, PebblesAction::GiveUp);
    system.run_next_block();

    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after giving up");

    assert_eq!(state.winner, Some(Player::Program));

    let restart_message = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 15,
        max_pebbles_per_turn: 10,
    };

    program.send(ID, restart_message);
    system.run_next_block();

    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after restart");

    assert!(state.pebbles_remaining <= 15);
    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn, 10);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
    assert_eq!(state.winner, None);
}

#[test]
fn test_with_negative_input() {
    let system = System::new();
    let program = Program::current(&system);
    let pid = program.id();
    system.mint_to(ID, VALUE);
    // Setting of the pebbles game
    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 3,
        max_pebbles_per_turn: 4,
    };
    program.send(ID, init_message);
    system.run_next_block();
    // Check the initial state of the GameState
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the initial state of the game");

    assert!(state.pebbles_remaining <= 10);
    assert_eq!(state.pebbles_count, 3);
    assert_eq!(state.max_pebbles_per_turn, 4);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);
    assert_eq!(state.winner, None::<Player>);

    program.send(ID, PebblesAction::Turn(1));
    system.run_next_block();

    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after user's turn");

    if state.pebbles_remaining <= state.max_pebbles_per_turn {
        // Restart the game
        let restart_message = PebblesAction::Restart {
            difficulty: DifficultyLevel::Hard,
            pebbles_count: 15,
            max_pebbles_per_turn: 10,
        };

        program.send(ID, restart_message);
        system.run_next_block();

        let state: GameState = program
            .read_state(pid)
            .expect("Failed to get the state of the game after restart");

        assert!(state.pebbles_remaining <= 15);
        assert_eq!(state.pebbles_count, 15);
        assert_eq!(state.max_pebbles_per_turn, 10);
        assert_eq!(state.difficulty, DifficultyLevel::Hard);
        assert_eq!(state.winner, None);
    } else {
        program.send(ID, PebblesAction::GiveUp);
        system.run_next_block();

        let state: GameState = program
            .read_state(pid)
            .expect("Failed to get the state of the game after giving up");
        assert_eq!(state.winner, Some(Player::Program));
    }
}

#[test]
fn test_with_illegal_input() {
    let system = System::new();
    let program = Program::current(&system);
    let pid = program.id();
    let pebbles_count = 10;
    system.mint_to(ID, VALUE);
    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count,
        max_pebbles_per_turn: 4,
    };

    program.send(ID, init_message);
    system.run_next_block();

    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the initial state of the game");

    assert!(state.pebbles_remaining <= 10);
    assert_eq!(state.pebbles_count, 10);
    assert_eq!(state.max_pebbles_per_turn, 4);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);
    assert_eq!(state.winner, None::<Player>);

    program.send(ID, PebblesAction::Turn(12));
    system.run_next_block();

    assert!(state.max_pebbles_per_turn <= 12);

    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after user's turn");

    if state.pebbles_remaining >= 1 {
        program.send(ID, PebblesAction::GiveUp);
        system.run_next_block();

        let state: GameState = program
            .read_state(pid)
            .expect("Failed to get the state of the game after giving up");

        assert_eq!(state.winner, Some(Player::Program));
    };

    let restart_message = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 15,
        max_pebbles_per_turn: 10,
    };

    program.send(ID, restart_message);
    system.run_next_block();

    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after restart");

    assert!(state.pebbles_remaining <= 15);
    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn, 10);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
    assert_eq!(state.winner, None);
}
