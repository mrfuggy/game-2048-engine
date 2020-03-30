/* engine_config.rs -- engine configuration parameters.
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

use crate::engine::evaluation::Weights;

pub struct EngineConfig {
    pub depth: u16,
    pub eval_fn: Weights,
    pub algorithm: Algorithm,
    pub random_mode: RandomCompleteness,
    pub order_moves: bool,
}

pub enum RandomCompleteness {
    /// All posible moves
    Full,
    /// n First posible moves
    Ordered(u8),
    /// n Random posible moves
    MonteCarlo(u8),
}

pub enum Algorithm {
    Minimax,
    MinimaxAlphaBeta,
    Negamax,
    NegamaxAlphaBeta,
    NegaScout,
    ExpectiMinimax,
}
