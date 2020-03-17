/* node.rs -- minimax tree node.
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

use crate::board::Board;
use crate::board::State;
use crate::direction::Direction;
use crate::engine::engine_config::EngineConfig;
use crate::engine::evaluation;
use crate::engine::moves::BestMove;
use crate::engine::moves::Move;

const BOARD_SIZE: usize = 4;

#[derive(Debug, Default)]
pub(crate) struct Node {
    pub(crate) board: Board,
    turn: Move,
    value: i32,
    pub(super) children: Option<Vec<Node>>,
}

const DIRICTION_CYCLE: [Direction; 4] = [
    Direction::Left,
    Direction::Right,
    Direction::Down,
    Direction::Up,
];

impl Node {
    pub(super) fn with_board(new_board: Board, turn: Move) -> Node {
        Node {
            board: new_board,
            turn: turn,
            value: 0,
            children: None,
        }
    }

    fn gen_next_nodes(&mut self, config: &EngineConfig) -> &mut Vec<Node> {
        if self.children.is_none() {
            let nodes = if self.turn.is_human() {
                self.next_random_moves(config)
            } else {
                self.next_human_moves()
            };
            self.children = Some(nodes);
        }

        match self.children {
            Some(ref mut vec) => vec,
            _ => unreachable!(),
        }
    }

    fn next_human_moves(&self) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::with_capacity(4);
        for dir in &DIRICTION_CYCLE {
            let mut new_board = self.board;
            let moved = new_board.slide_to(*dir);
            if moved {
                let node = Node::with_board(new_board, Move::Human(*dir));
                nodes.push(node);
            }
        }
        nodes.shrink_to_fit();
        nodes
    }

    fn next_random_moves(&self, config: &EngineConfig) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::with_capacity(15);
        self.next_random_moves_value(&mut nodes, 1);
        self.next_random_moves_value(&mut nodes, 2);
        nodes.shrink_to_fit();
        nodes
    }

    fn next_random_moves_value(&self, nodes: &mut Vec<Node>, value: u8) {
        let mut c = 0u8;
        for j in 0..BOARD_SIZE {
            for i in 0..BOARD_SIZE {
                if self.board.board[j][i] == 0 {
                    let mut new_board = self.board;
                    new_board.board[j][i] = value;
                    let node = Node::with_board(new_board, Move::Random(c, value));
                    nodes.push(node);
                    c += 1;
                }
            }
        }
    }
}

//algorithms
impl Node {
    pub(super) fn minimax(
        &mut self,
        config: &EngineConfig,
        depth: u16,
        max_player: bool,
    ) -> BestMove {
        if depth == 0 || self.board.state == State::Lose {
            let score = evaluation::evaluate(config.eval_fn, self);
            self.value = score;
            return BestMove {
                turn: self.turn,
                local_id: 0,
                score,
            };
        }

        if max_player {
            let mut value = BestMove::new(-1000);
            let nodes = self.gen_next_nodes(config);
            for (index, node) in nodes.iter_mut().enumerate() {
                let best_move = node.minimax(config, depth - 1, false);
                if best_move.score > value.score {
                    value = BestMove {
                        turn: node.turn,
                        local_id: index as u8,
                        score: best_move.score,
                    };
                }
            }
            self.value = value.score;
            value
        } else {
            let mut value = BestMove::new(1000);
            let nodes = self.gen_next_nodes(config);

            for (index, node) in nodes.iter_mut().enumerate() {
                let best_move = node.minimax(config, depth - 1, true);
                if best_move.score < value.score {
                    value = BestMove {
                        turn: node.turn,
                        local_id: index as u8,
                        score: best_move.score,
                    };
                }
            }
            self.value = value.score;
            value
        }
    }

    pub(super) fn negamax(&mut self, config: &EngineConfig, depth: u16, color: i8) -> BestMove {
        if depth == 0 || self.board.state == State::Lose {
            let score = color as i32 * evaluation::evaluate(config.eval_fn, self);
            self.value = score;
            return BestMove {
                turn: self.turn,
                local_id: 0,
                score,
            };
        }

        let mut value = BestMove::new(-1000);
        let nodes = self.gen_next_nodes(config);
        for (index, node) in nodes.iter_mut().enumerate() {
            let best_move = -node.negamax(config, depth - 1, -color);
            if best_move.score > value.score {
                value = BestMove {
                    turn: node.turn,
                    local_id: index as u8,
                    score: best_move.score,
                };
            }
        }
        self.value = value.score;
        value
    }
}
