/* board.rs -- common board actions.
Copyright (C) 2020-2021 fuggy

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

use crate::cache::lazy::Lazy;
use crate::cache::slide_cache::SlideCache;
use crate::direction::Direction;
use crate::matrix;

#[derive(Debug, Clone, Copy, Default)]
pub struct Board {
    pub board: [[u8; BOARD_SIZE]; BOARD_SIZE],
    pub state: State,
    pub score: u32,
    pub move_count: u16,
}

pub const BOARD_SIZE: usize = 4;
static SLIDE_CACHE: Lazy<SlideCache> = Lazy::new(SlideCache::new());

pub fn load_cache() {
    SLIDE_CACHE.set(SlideCache::load_cache());
}

pub fn create_cache() {
    SlideCache::create_cache();
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    InGame,
    Lose,
}

impl Default for State {
    fn default() -> Self {
        State::InGame
    }
}

impl Board {
    /// Create Empty Board
    pub fn new() -> Self {
        Board {
            board: [[0u8; BOARD_SIZE]; BOARD_SIZE],
            state: State::InGame,
            score: 0,
            move_count: 0,
        }
    }

    /// Slide board to specific side
    pub fn slide_to(&mut self, dir: Direction) -> bool {
        let moved = match dir {
            Direction::Left => self.slide_board(matrix::to_u16, matrix::from_u16),
            Direction::Right => self.slide_board(matrix::to_u16_rev, matrix::from_u16_rev),
            Direction::Up => {
                matrix::transpose(&mut self.board);
                let moved = self.slide_board(matrix::to_u16, matrix::from_u16);
                matrix::transpose(&mut self.board);
                moved
            }
            Direction::Down => {
                matrix::transpose(&mut self.board);
                let moved = self.slide_board(matrix::to_u16_rev, matrix::from_u16_rev);
                matrix::transpose(&mut self.board);
                moved
            }
        };

        if moved {
            self.move_count += 1;
        }
        moved
    }

    /// Slide board with cache get functions
    fn slide_board(
        &mut self,
        encode_fn: fn([u8; BOARD_SIZE]) -> u16,
        decode_fn: fn(&mut [u8; BOARD_SIZE], u16),
    ) -> bool {
        let mut moved = false;
        for j in 0..BOARD_SIZE {
            let id = encode_fn(self.board[j]);
            let item = SLIDE_CACHE.get().table[id as usize];
            if item.line != 0 {
                decode_fn(&mut self.board[j], item.line);
                self.score += if item.score == 0 {
                    0
                } else {
                    (item.score + 1) as u32
                };
                moved = true;
            }
        }

        moved
    }

    /// Is any move possible
    pub fn can_move(&self) -> bool {
        self.empty_count() > 0u8
            // two perpendicular enough
            || self.can_move_dir(Direction::Left)
            || self.can_move_dir(Direction::Up)
    }

    /// Is it possible to merge cells in a specific plane
    pub fn can_move_dir(&self, dir: Direction) -> bool {
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

    /// Put value in specific empty cell
    pub fn set_move(&mut self, game_move: (u8, u8)) {
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
        self.move_count += 1;
    }

    /// Count the number of empty cells
    pub fn empty_count(&self) -> u8 {
        matrix::empty_count(&self.board)
    }

    /// Count the number of empty cells
    pub fn max_cell(&self) -> u16 {
        1 << matrix::max_cell(&self.board)
    }

    // Get board as u64 value
    pub fn get_board_id(&self) -> u64 {
        matrix::to_u64(&self.board)
    }
}
