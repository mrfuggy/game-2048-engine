/* game.rs -- manage the game process.
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

use super::random::{Rnd, RndMove};

#[derive(Debug)]
pub struct Game {
    board: [[u8; 4]; 4],
    rnd: Rnd,
}

impl Game {
    pub fn start_new() -> Game {
        let rnd = Rnd::new();
        let b = [[0u8; 4]; 4];
        let mut start_position = Game { board: b, rnd: rnd };

        //16 empty cell at the beginning
        let next_move = start_position.rnd.next_move(16);
        start_position.set_move(next_move);

        let next_move = start_position.rnd.next_move(15);
        start_position.set_move(next_move);
        return start_position;
    }

    fn set_move(&mut self, gmove: (u8, u8)) {
        let (value, pos) = gmove;
        let mut c = 0u8;
        'outer: for i in 0..4 {
            for j in 0..4 {
                if self.board[j][i] == 0 {
                    if c == pos {
                        self.board[j][i] = value;
                        break 'outer;
                    }
                    c += 1;
                }
            }
        }
    }

    fn empty_count(&self) -> u8 {
        let mut c = 0u8;
        for i in 0..4 {
            for j in 0..4 {
                c += (self.board[j][i] == 0) as u8;
            }
        }
        return c;
    }
}
