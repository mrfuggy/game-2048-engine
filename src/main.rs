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
use game_2048_engine::engine::evaluation::EvaluationFunction;
use game_2048_engine::engine::node::Move;
use game_2048_engine::game::Game;
use game_2048_engine::input;
use std::io;

fn main() {
    let mut game = Game::start_new();
    let engine_config = EngineConfig {
        depth: 7,
        eval_fn: EvaluationFunction::MaxCell,
        algorithm: Algorithm::Negamax,
        random_mode: RandomCompleteness::Full,
    };
    let mut engine = Engine::from_game(&game, engine_config);
    let best_move = engine.best_move();
    println!("{}", game);
    if let Move::Human(human_move) = best_move {
        println!("{:?}", human_move);
    }
    if let Move::Random(c, v) = best_move {
        println!("{} = {}", c, v);
    }
}

fn game() {
    let mut game = Game::start_new();
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
