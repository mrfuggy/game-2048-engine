/* output.rs -- output utils.
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

use super::board::BOARD_SIZE;
use super::game::Game;
use std::fmt;

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "score: {}", self.board.score)?;

        for j in 0..BOARD_SIZE {
            for i in 0..BOARD_SIZE {
                let value = if self.board.board[j][i] != 0 {
                    1 << self.board.board[j][i]
                } else {
                    0
                };
                write!(f, "{:>6}", value)?
            }
            writeln!(f)?
        }
        write!(f, "")
    }
}
