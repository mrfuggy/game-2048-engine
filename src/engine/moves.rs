/* moves.rs -- move struct.
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

use crate::direction::Direction;
use std::ops::Add;
use std::ops::Neg;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Move {
    Human(Direction),
    Random(u8, u8),
}

impl Move {
    pub(super) fn is_human(&self) -> bool {
        match self {
            Move::Human(_) => true,
            Move::Random(_, _) => false,
        }
    }
}

/*impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Move::Human(l_dir) => match other {
                Move::Human(r_dir) => l_dir == r_dir,
                _ => false,
            },
            Move::Random(l_index, l_value) => match other {
                Move::Random(r_index, r_value) => l_index == r_index & l_value == r_value,
                _ => false,
            }
        }
    }
}*/

impl Default for Move {
    fn default() -> Move {
        Move::Random(0, 0)
    }
}

#[derive(Debug)]
pub struct BestMove {
    pub turn: Move,
    pub(super) local_id: u8,
    pub(super) score: i32,
    pub(super) stat: Statistics,
}

impl BestMove {
    pub(super) fn new(score: i32) -> BestMove {
        const EMPTY: Move = Move::Random(0, 0);
        BestMove {
            turn: EMPTY,
            local_id: 0,
            score,
            stat: Statistics::default(),
        }
    }
}

impl Neg for BestMove {
    type Output = Self;
    fn neg(self) -> Self::Output {
        BestMove {
            score: -self.score,
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Statistics {
    pub(super) total_nodes: u32,
    pub(super) cut_nodes: u32,
}

impl Add for Statistics {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Statistics {
            total_nodes: self.total_nodes + other.total_nodes,
            cut_nodes: self.cut_nodes + other.cut_nodes,
        }
    }
}
