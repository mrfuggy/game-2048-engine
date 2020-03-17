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
}

pub(super) fn evaluate(eval_fn: EvaluationFunction, node: &Node) -> i32 {
    match eval_fn {
        EvaluationFunction::MaxCell => evaluation_max_cell(node),
        EvaluationFunction::MaxScore => evaluation_max_score(node),
        EvaluationFunction::Monotonicity => evaluation_monotonicity(node),
        EvaluationFunction::Smoothness => evaluation_smoothness(node),
        EvaluationFunction::StdDev => evaluation_std_dev(node),
    }
}

fn evaluation_max_cell(node: &Node) -> i32 {
    matrix::max_cell(&node.board) as i32
}

fn evaluation_max_score(node: &Node) -> i32 {
    node.score as i32
}

fn evaluation_monotonicity(_node: &Node) -> i32 {
    unimplemented!()
}

fn evaluation_smoothness(_node: &Node) -> i32 {
    unimplemented!()
}

fn evaluation_std_dev(_node: &Node) -> i32 {
    unimplemented!()
}
