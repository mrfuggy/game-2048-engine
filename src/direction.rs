/* direction.rs -- direction of move.
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

use super::game::BOARD_SIZE;
use std::iter::Rev;
use std::ops::Range;

#[derive(Copy, Clone)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub(super) enum RangeOrRev {
    RORRange(Range<usize>),
    RORRev(Rev<Range<usize>>),
}

impl Iterator for RangeOrRev {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        match self {
            RangeOrRev::RORRange(range) => range.next(),
            RangeOrRev::RORRev(rev) => rev.next(),
        }
    }
}

impl Direction {
    pub(super) fn is_horizontal(&self) -> bool {
        match self {
            Direction::Right => true,
            Direction::Left => true,
            Direction::Up => false,
            Direction::Down => false,
        }
    }

    pub(super) fn get_mask(&self) -> (usize, usize) {
        match self {
            Direction::Right => (1usize, 0usize),
            Direction::Left => (1usize, 0usize),
            Direction::Up => (0usize, 1usize),
            Direction::Down => (0usize, 1usize),
        }
    }

    // ← ↑ → ↓ - loop order
    // ↤ ↦ ↥ ↧ - find

    // Direction = j i k
    // Left      = ↓ → ↦
    // Right     = ↓ ← ↤
    // Up        = ↓ → ↧
    // Down      = ↑ → ↥

    /// Vertical order
    pub(super) fn get_range_j(&self) -> RangeOrRev {
        match self {
            Direction::Left => RangeOrRev::RORRange(0..BOARD_SIZE),
            Direction::Right => RangeOrRev::RORRange(0..BOARD_SIZE),
            Direction::Up => RangeOrRev::RORRange(0..BOARD_SIZE - 1),
            Direction::Down => RangeOrRev::RORRev((1..BOARD_SIZE).rev()),
        }
    }

    /// Horizontal order
    pub(super) fn get_range_i(&self) -> RangeOrRev {
        match self {
            Direction::Left => RangeOrRev::RORRange(0..BOARD_SIZE - 1),
            Direction::Right => RangeOrRev::RORRev((1..BOARD_SIZE).rev()),
            Direction::Up => RangeOrRev::RORRange(0..BOARD_SIZE),
            Direction::Down => RangeOrRev::RORRange(0..BOARD_SIZE),
        }
    }

    /// Search order next value in the current line
    pub(super) fn get_range_k(&self, current: usize) -> RangeOrRev {
        match self {
            Direction::Left => RangeOrRev::RORRange(current + 1..BOARD_SIZE),
            Direction::Right => RangeOrRev::RORRev((0..current).rev()),
            Direction::Up => RangeOrRev::RORRange(current + 1..BOARD_SIZE),
            Direction::Down => RangeOrRev::RORRev((0..current).rev()),
        }
    }
}
