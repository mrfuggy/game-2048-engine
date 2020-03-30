/* evaluation.rs -- evaluation functions.
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

use crate::engine::node::Node;
use crate::matrix;

/// Normalized weights for evaluation functions (fraction)
/// 0 - disable component of evaluation function
/// Optimal overflow prevention range 0..397 (int32 / 6 / max score per compenent)
#[derive(Clone, Copy)]
pub struct Weights {
    pub max_cell: i32,
    pub max_score: i32,
    pub monotonicity: i32,
    pub smoothness: i32,
    pub std_dev: i32,
    pub free_space: i32,
    pub snakeiness: i32,
}

impl Weights {
    pub fn normalize(mut self) -> Weights {
        self.max_cell *= 14;
        //baseline 900_000 score
        //self.max_score *= 1;
        self.monotonicity *= 112_500;
        self.smoothness *= 2344;
        //almost equals
        //self.std_dev *= 1;
        self.free_space *= 60000;
        self.snakeiness *= 23;
        self
    }
}

pub(super) fn evaluate(weights: Weights, node: &Node) -> i32 {
    // TODO in ordered moves, after replace with cache
    if node.value != 0 {
        return node.value;
    }

    let mut score = 0;
    if weights.max_cell != 0 {
        score += weights.max_cell * evaluation_max_cell(node);
    }
    if weights.max_score != 0 {
        score += weights.max_score * evaluation_max_score(node);
    }
    if weights.monotonicity != 0 {
        score += weights.monotonicity * evaluation_monotonicity(node);
    }
    if weights.smoothness != 0 {
        score += weights.smoothness * evaluation_smoothness(node);
    }
    if weights.std_dev != 0 {
        score += weights.std_dev * evaluation_std_dev(node);
    }
    if weights.free_space != 0 {
        score += weights.free_space * evaluation_free_space(node);
    }
    if weights.snakeiness != 0 {
        score += weights.snakeiness * evaluation_snakeiness(node);
    }
    score
}

/// range 0..65536 theory max 131072
fn evaluation_max_cell(node: &Node) -> i32 {
    node.board.max_cell() as i32
}

/// range 0..~900_000 effective to win 60_000
fn evaluation_max_score(node: &Node) -> i32 {
    node.board.score as i32
}

/// range 0..8
fn evaluation_monotonicity(node: &Node) -> i32 {
    matrix::monotonicity(&node.board.board)
}

// range 0..384
fn evaluation_smoothness(node: &Node) -> i32 {
    //negate this - less is better
    -matrix::smoothness(&node.board.board) + 384
}

// range 0..~912_000
fn evaluation_std_dev(node: &Node) -> i32 {
    //negate this - less is better
    -matrix::std_dev(&node.board.board) + 1000
}

// range 0..15
fn evaluation_free_space(node: &Node) -> i32 {
    matrix::empty_count(&node.board.board) as i32
}

// range 0..39312
fn evaluation_snakeiness(node: &Node) -> i32 {
    matrix::snakeiness(&node.board.board) as i32
}
