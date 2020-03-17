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

#[derive(Clone, Copy)]
pub enum EvaluationFunction {
    MaxCell,
    MaxScore,
    Monotonicity,
    Smoothness,
    StdDev,
    FreeSpace,
}

pub(super) fn evaluate(eval_fn: EvaluationFunction, node: &Node) -> i32 {
    match eval_fn {
        EvaluationFunction::MaxCell => evaluation_max_cell(node),
        EvaluationFunction::MaxScore => evaluation_max_score(node),
        EvaluationFunction::Monotonicity => evaluation_monotonicity(node),
        EvaluationFunction::Smoothness => evaluation_smoothness(node),
        EvaluationFunction::StdDev => evaluation_std_dev(node),
        EvaluationFunction::FreeSpace => evaluation_free_space(node),
    }
}

/// range 0..65536 theory max 131072
fn evaluation_max_cell(node: &Node) -> i32 {
    node.board.max_cell() as i32
}

/// range 0..~1_000_000
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

fn evaluation_std_dev(_node: &Node) -> i32 {
    unimplemented!()
}

// range 0..15
fn evaluation_free_space(node: &Node) -> i32 {
    matrix::empty_count(&node.board.board) as i32
}
