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

use super::direction::Direction;
use super::matrix;
use super::random;
use super::random::{Rnd, RndMove};
use std::fmt;

#[derive(Debug)]
pub struct Game {
    board: [[u8; BOARD_SIZE]; BOARD_SIZE],
    rnd: Rnd,
    pub state: State,
    pub score: u32,
    pub move_count: u16,
}

pub const BOARD_SIZE: usize = 4;
const CELL_COUNT: u8 = (BOARD_SIZE as u8) * (BOARD_SIZE as u8);

#[derive(Debug, PartialEq)]
pub enum State {
    InGame,
    Lose,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "score: {}", self.score)?;

        for j in 0..BOARD_SIZE {
            for i in 0..BOARD_SIZE {
                let value = if self.board[j][i] != 0 {
                    2 << self.board[j][i] - 1
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

impl Game {
    /// Create new start position
    pub fn start_new() -> Game {
        let rnd = random::get_rnd();
        let b = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        let mut start_position = Game {
            board: b,
            rnd: rnd,
            state: State::InGame,
            score: 0,
            move_count: 0,
        };

        // 'CELL_COUNT' empty cell at the beginning
        let next_move = start_position.rnd.next_move(CELL_COUNT);
        start_position.set_move(next_move);

        let next_move = start_position.rnd.next_move(CELL_COUNT - 1);
        start_position.set_move(next_move);
        start_position
    }

    /// Make human move then random move
    pub fn make_move(&mut self, dir: Direction) {
        let move_made = self.human_move(dir);
        if move_made {
            self.random_move();
        }

        if !self.can_move() {
            self.state = State::Lose;
        }
    }

    /// Make human move
    /// returns: change has been made
    pub fn human_move(&mut self, dir: Direction) -> bool {
        if self.state == State::Lose {
            return false;
        }

        let can_move = self.empty_count() > 0u8 || self.can_move_dir(dir);
        if !can_move {
            return false;
        }

        let moved = self.slide_to(dir);
        // extra check when closing zeros
        if moved {
            self.move_count += 1;
            return moved;
        }

        //TODO
        return moved;
        //return true;
    }

    /// Slide board to specific side
    fn slide_to(&mut self, dir: Direction) -> bool {
        let mut moved = false;
        for j in dir.get_range_j() {
            for i in dir.get_range_i() {
                if dir.is_horizontal() {
                    moved |= self.slide_h(dir, j, i);
                } else {
                    moved |= self.slide_v(dir, j, i);
                }
            }
        }

        moved
    }

    /// Slide line horizontally in current point
    fn slide_h(&mut self, dir: Direction, j: usize, i: usize) -> bool {
        let mut moved = false;
        // move next non zero to current
        if self.board[j][i] == 0 {
            // from current to end of line
            for k in dir.get_range_k(i) {
                if self.board[j][k] != 0 {
                    self.board[j][i] = self.board[j][k];
                    self.board[j][k] = 0;
                    moved = true;
                    break;
                }
            }
        }

        // exit if rest are zeros
        if self.board[j][i] == 0 {
            return moved;
        }

        for k in dir.get_range_k(i) {
            if self.board[j][i] == self.board[j][k] {
                self.board[j][i] += 1;
                self.board[j][k] = 0;
                moved = true;
                // add score
                self.score += 2 << self.board[j][i] - 1;
                // one merge per cell
                break;
            } else if self.board[j][k] != 0 {
                break;
            }
        }

        moved
    }

    /// Slide line vertically in current point
    fn slide_v(&mut self, dir: Direction, j: usize, i: usize) -> bool {
        let mut moved = false;
        // move next non zero to current
        if self.board[j][i] == 0 {
            // from current to end of line
            for k in dir.get_range_k(j) {
                if self.board[k][i] != 0 {
                    self.board[j][i] = self.board[k][i];
                    self.board[k][i] = 0;
                    moved = true;
                    break;
                }
            }
        }

        // exit if rest are zeros
        if self.board[j][i] == 0 {
            return moved;
        }

        for k in dir.get_range_k(j) {
            if self.board[j][i] == self.board[k][i] {
                self.board[j][i] += 1;
                self.board[k][i] = 0;
                moved = true;
                // add score
                self.score += 2 << self.board[j][i] - 1;
                // one merge per cell
                break;
            } else if self.board[k][i] != 0 {
                break;
            }
        }

        moved
    }

    /// Is any move possible
    fn can_move(&self) -> bool {
        self.empty_count() > 0u8
            // two perpendicular enough
            || self.can_move_dir(Direction::Left)
            || self.can_move_dir(Direction::Up)
    }

    /// Is it possible to merge cells in a specific plane
    fn can_move_dir(&self, dir: Direction) -> bool {
        let (x, y) = dir.get_mask();
        for j in 0..BOARD_SIZE - y {
            for i in 0..BOARD_SIZE - x {
                if self.board[j][i] == self.board[j + y][i + x] {
                    return true;
                }
            }
        }
        false
    }

    /// Put random value in an empty spot
    pub fn random_move(&mut self) -> Option<(u8, u8)> {
        if self.state == State::Lose {
            return None;
        }

        let empty_count = self.empty_count();
        let next_move = self.rnd.next_move(empty_count);
        self.set_move(next_move);
        Some(next_move)
    }

    /// Put value in specific empty cell
    fn set_move(&mut self, game_move: (u8, u8)) {
        let (value, pos) = game_move;
        let mut c = 0u8;
        'outer: for j in 0..BOARD_SIZE {
            for i in 0..BOARD_SIZE {
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

    /// Count the number of empty cells
    fn empty_count(&self) -> u8 {
        matrix::empty_count(&self.board)
    }
}
