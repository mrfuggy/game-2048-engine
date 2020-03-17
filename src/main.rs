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

mod direction;
mod game;
mod input;
mod matrix;
mod random;

use direction::Direction;
use game::{Game, State};
use std::io;

fn main() {
    let mut game = Game::start_new();
    loop {
        println!("{}", game);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.len() > 1 {
            let dir = input::parse_input(&input.chars().nth(0).unwrap());
            print!("\r");
            if dir.is_some() {
                game.make_move(dir.unwrap());
            }
        }

        if game.state == State::Lose {
            println!("You lost. Score:{}", game.score);
            break;
        }
    }
}
