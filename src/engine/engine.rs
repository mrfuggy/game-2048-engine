/* engine.rs -- engine manager.
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

use crate::engine::engine_config::Algorithm;
use crate::engine::engine_config::EngineConfig;
use crate::engine::moves::Move;
use crate::engine::node::Node;
use crate::game::Game;

pub struct Engine {
    pub(super) root: Node,
    pub(super) config: EngineConfig,
}

#[derive(Default)]
pub(super) struct Statistics {
    pub(super) total_nodes: u32,
    pub(super) cut_nodes: u32,
}

impl Engine {
    pub fn from_game(game: &Game, config: EngineConfig) -> Engine {
        Engine {
            root: Node::with_board(game.board, Move::Random(0, 0)),
            config,
        }
    }

    pub fn best_move(&mut self) -> Move {
        let stat = Statistics::default();
        let best_move = match self.config.algorithm {
            Algorithm::Minimax => self.root.minimax(&self.config, self.config.depth, true),
            Algorithm::Negamax => self.root.negamax(&self.config, self.config.depth, 1),
            _ => unimplemented!(),
        };

        println!("{:?}", std::mem::size_of::<[[u8; 4]; 4]>());

        //best_turn
        if let Some(ref mut vec) = self.root.children {
            let next_move = std::mem::take(&mut vec[best_move.local_id as usize]);
            std::mem::replace(&mut self.root, next_move);
        }
        best_move.turn
    }
}
