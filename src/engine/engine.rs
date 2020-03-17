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

use crate::direction::Direction;
use crate::engine::engine_config::Algorithm;
use crate::engine::engine_config::EngineConfig;
use crate::engine::node::Move;
use crate::engine::node::Node;
use crate::game::Game;

pub(super) struct Engine {
    pub(super) root: Node,
    pub(super) config: EngineConfig,
}

impl Engine {
    pub fn new_from(game: &Game, config: EngineConfig) -> Engine {
        Engine {
            root: Node::from_game(game, Move::Random(0, 0)),
            config,
        }
    }

    pub fn best_move(&mut self) -> Move {
        let score/*(best_turn, score)*/ = match self.config.algorithm {
            Algorithm::Minimax => self.root.minimax(&self.config, self.config.depth, true),
            _ => unimplemented!(),
        };

        //best_turn
        Move::Human(Direction::Left)
    }
}
