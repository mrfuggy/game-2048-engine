/* direction.rs -- direction of move.
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

use core::convert::TryFrom;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub(super) fn get_mask(self) -> (usize, usize) {
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
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'w' => Ok(Direction::Up),
            'a' => Ok(Direction::Left),
            's' => Ok(Direction::Down),
            'd' => Ok(Direction::Right),

            'k' => Ok(Direction::Up),
            'j' => Ok(Direction::Left),
            'h' => Ok(Direction::Down),
            'l' => Ok(Direction::Right),

            '↑' => Ok(Direction::Up),
            '←' => Ok(Direction::Left),
            '↓' => Ok(Direction::Down),
            '→' => Ok(Direction::Right),

            _ => Err(()),
        }
    }
}
