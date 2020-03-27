/* main.rs -- run game or engine lib.
Copyright (C) 2020 fuggy

This file is part of game-2048-engine.

game-2048-engine is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

game-2048-engine is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with game-2048-engine.  If not, see <https://www.gnu.org/licenses/>.
*/

use game_2048_engine::board::State;
use game_2048_engine::direction::Direction;
use game_2048_engine::engine::engine::Engine;
use game_2048_engine::engine::engine_config::{Algorithm, EngineConfig, RandomCompleteness};
use game_2048_engine::engine::evaluation::Weights;
use game_2048_engine::engine::moves::Move;
use game_2048_engine::game::Game;
use game_2048_engine::input;
use std::io;

fn main() {
    //load caches
    game_2048_engine::board::load_cache();

    let mut game = Game::with_seed(3);
    //let mut game = Game::start_new();
    let weights = Weights {
        max_cell: 30,
        max_score: 10,
        monotonicity: 100,
        smoothness: 50,
        std_dev: 0,
        free_space: 300,
        snakeiness: 0,
    };
    let engine_config = EngineConfig {
        depth: 7,
        eval_fn: weights.normalize(),
        algorithm: Algorithm::NegamaxAlphaBeta,
        //algorithm: Algorithm::MinimaxAlphaBeta,
        random_mode: RandomCompleteness::Full,
        //random_mode: RandomCompleteness::MonteCarlo(10),
    };
    let mut engine = Engine::from_game(&game, engine_config);
    loop {
        println!("start move {} {} ", game.board.move_count, game);
        let best_move = engine.best_move();
        let move_made = game.human_move(best_move);
        println!("move {:?}", best_move);
        /*if game.board.move_count > 1 {
            break;
        }*/
        if move_made {
            if let Some((i, v)) = game.random_move() {
                engine.make_random_move(Move::Random(i, v));
                if game.board.state == State::Lose {
                    println!("start {}", game);
                    println!("You lost. Score: {}", game.board.score);
                    break;
                }
            }
        } else {
            //println!("{:?}", engine.root.board);
            panic!("wrong move");
        }
    }
}

#[allow(dead_code)]
fn create_cache() {
    game_2048_engine::board::create_cache();
}

#[allow(dead_code)]
fn game() {
    //load caches
    game_2048_engine::board::load_cache();

    let mut game = Game::with_seed(3);
    //let mut game = Game::start_new();
    loop {
        println!("{}", game);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.len() > 1 {
            let dir = input::parse_input(input.chars().nth(0).unwrap());
            print!("\r");
            if let Some(value) = dir {
                game.make_move(value);
            }
        }

        if game.board.state == State::Lose {
            println!("You lost. Score: {}", game.board.score);
            break;
        }
    }
}

#[allow(dead_code)]
fn simple_strategy() {
    let mut game = Game::start_new();

    loop {
        let mut all = false;
        all |= game.make_move(Direction::Left);
        if game.board.state == State::Lose {
            break;
        }
        all |= game.make_move(Direction::Down);
        if game.board.state == State::Lose {
            break;
        }
        all |= game.make_move(Direction::Right);
        if game.board.state == State::Lose {
            break;
        }
        all |= game.make_move(Direction::Down);
        if game.board.state == State::Lose {
            break;
        }
        if !all {
            game.make_move(Direction::Up);
            if game.board.state == State::Lose {
                break;
            }
        }
    }
    println!(
        "You lost. Score: {} Max: {} Moves: {}",
        game.board.score,
        game.board.max_cell(),
        game.board.move_count
    );
    //println!("{}", game);
}
