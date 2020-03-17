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
use std::cmp::Ordering;
use std::ops::Neg;

#[derive(Debug, Clone, Copy)]
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
    pub(super) fn is_random(&self) -> bool {
        !self.is_human()
    }
}

impl Default for Move {
    fn default() -> Move {
        Move::Random(0, 0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BestMove {
    pub turn: Move,
    pub(super) local_id: u8,
    pub(super) score: i32,
}

impl BestMove {
    pub(super) fn new(score: i32) -> BestMove {
        const EMPTY: Move = Move::Random(0, 0);
        BestMove {
            turn: EMPTY,
            local_id: 0,
            score,
        }
    }

    fn with_local_id(self, local_id: u8) -> BestMove {
        BestMove {
            local_id: local_id,
            ..self
        }
    }
}

impl Ord for BestMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for BestMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for BestMove {}

impl PartialEq for BestMove {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Neg for BestMove {
    type Output = BestMove;
    fn neg(self) -> Self::Output {
        BestMove {
            score: -self.score,
            ..self
        }
    }
}
