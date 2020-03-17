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
use crate::engine::engine_config::{EngineConfig, RandomCompleteness};
use crate::engine::evaluation;
use crate::engine::moves::BestMove;
use crate::engine::moves::Move;
use crate::engine::moves::Statistics;
use crate::random;
use crate::random::RndMove;
use std::mem::take;

const BOARD_SIZE: usize = 4;

#[derive(Debug, Default)]
pub(super) struct Node {
    pub(super) board: Board,
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
            turn,
            value: 0,
            children: None,
        }
    }

    fn as_terminal_leaf(&self) -> BestMove {
        BestMove {
            turn: self.turn,
            local_id: 0,
            score: self.value,
            stat: Statistics::default(),
        }
    }

    /// Use existing tree
    pub(super) fn find_next_random_move(&mut self, random_move: Move) -> Node {
        if let Some(ref mut vec) = self.children {
            //TODO use map
            for node in vec {
                if node.turn == random_move {
                    let next_move = take(node);
                    return next_move;
                }
            }
        }

        //otherwise if not found, create new tree
        let rmove = random_move.unwrap_random();
        self.board.set_move(rmove);
        Node::with_board(self.board, random_move)
    }

    fn gen_next_nodes(&mut self, config: &EngineConfig) -> &mut Option<Vec<Node>> {
        if self.children.is_none() {
            let nodes = if self.turn.is_human() {
                self.next_random_moves(config)
            } else {
                self.next_human_moves()
            };
            if !nodes.is_empty() {
                self.children = Some(nodes);
            } else {
                self.children = None;
            }
        }

        &mut self.children
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
        match config.random_mode {
            RandomCompleteness::Full => self.next_random_moves_full(),
            RandomCompleteness::Ordered(count) => self.next_random_moves_limit(count),
            RandomCompleteness::MonteCarlo(count) => self.next_random_moves_montecarlo(count),
        }
    }

    fn next_random_moves_limit(&self, limit: u8) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::with_capacity(limit as usize);
        self.next_random_moves_value_limit(&mut nodes, 1, limit);
        if (nodes.len() as u8) < limit {
            let limit = limit - nodes.len() as u8;
            self.next_random_moves_value_limit(&mut nodes, 2, limit);
        }
        nodes
    }

    fn next_random_moves_montecarlo(&self, mut limit: u8) -> Vec<Node> {
        let mut rnd = random::get_rnd();
        let mut empty_count = self.board.empty_count();

        if empty_count < limit {
            limit = empty_count;
        }
        let mut nodes: Vec<Node> = Vec::with_capacity(limit as usize);

        for _i in 0..limit {
            let next_move = rnd.next_move(empty_count);
            let mut new_board = self.board;
            new_board.set_move(next_move);
            let node = Node::with_board(new_board, Move::from_tuple(next_move));
            nodes.push(node);

            empty_count -= 1;
        }

        nodes
    }

    fn next_random_moves_full(&self) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::with_capacity(30);
        self.next_random_moves_value(&mut nodes, 1);
        self.next_random_moves_value(&mut nodes, 2);
        nodes.shrink_to_fit();
        nodes
    }

    fn next_random_moves_value(&self, nodes: &mut Vec<Node>, value: u8) {
        //TODO move to board
        let mut c = 0u8;
        for j in 0..BOARD_SIZE {
            for i in 0..BOARD_SIZE {
                if self.board.board[j][i] == 0 {
                    let mut new_board = self.board;
                    new_board.board[j][i] = value;
                    new_board.move_count += 1;
                    let node = Node::with_board(new_board, Move::Random(value, c));
                    nodes.push(node);
                    c += 1;
                }
            }
        }
    }

    fn next_random_moves_value_limit(&self, nodes: &mut Vec<Node>, value: u8, limit: u8) {
        //TODO move to board
        let mut c = 0u8;
        'outer: for j in 0..BOARD_SIZE {
            for i in 0..BOARD_SIZE {
                if self.board.board[j][i] == 0 {
                    let mut new_board = self.board;
                    new_board.board[j][i] = value;
                    new_board.move_count += 1;
                    let node = Node::with_board(new_board, Move::Random(value, c));
                    nodes.push(node);
                    c += 1;
                    if c >= limit {
                        break 'outer;
                    }
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
            self.value = evaluation::evaluate(config.eval_fn, self);
            return self.as_terminal_leaf();
        }

        if max_player {
            let nodes = self.gen_next_nodes(config);
            let mut value = BestMove::new(-1_000_000_000);

            if let Some(ref mut vec) = nodes {
                for (index, node) in vec.iter_mut().enumerate() {
                    let best_move = node.minimax(config, depth - 1, false);
                    max_score_move(best_move, &mut value, &node, index);
                }
                self.value = value.score;
                value
            } else {
                //penalty for losing
                self.value = evaluation::evaluate(config.eval_fn, self) - 1_000_000;
                self.as_terminal_leaf()
            }
        } else {
            let nodes = self.gen_next_nodes(config);
            let mut value = BestMove::new(1_000_000_000);

            if let Some(ref mut vec) = nodes {
                for (index, node) in vec.iter_mut().enumerate() {
                    let best_move = node.minimax(config, depth - 1, true);
                    min_score_move(best_move, &mut value, &node, index);
                }
                self.value = value.score;
                value
            } else {
                //TODO estimate the possibility of cutting a node with non full filling or alpha-beta
                //penalty for losing
                self.value = evaluation::evaluate(config.eval_fn, self) + 1_000_000;
                self.as_terminal_leaf()
            }
        }
    }

    pub(super) fn negamax(&mut self, config: &EngineConfig, depth: u16, color: i8) -> BestMove {
        if depth == 0 || self.board.state == State::Lose {
            self.value = color as i32 * evaluation::evaluate(config.eval_fn, self);
            return self.as_terminal_leaf();
        }

        let nodes = self.gen_next_nodes(config);
        let mut value = BestMove::new(-1_000_000_000);

        if let Some(ref mut vec) = nodes {
            for (index, node) in vec.iter_mut().enumerate() {
                let best_move = -node.negamax(config, depth - 1, -color);
                max_score_move(best_move, &mut value, &node, index);
            }
            self.value = value.score;
            value
        } else {
            //TODO estimate the possibility of cutting a node with non full filling or alpha-beta
            //penalty for losing
            self.value = color as i32 * (evaluation::evaluate(config.eval_fn, self) - 1_000_000);
            self.as_terminal_leaf()
        }
    }
}

fn max_score_move(best_move: BestMove, current_value: &mut BestMove, node: &Node, index: usize) {
    if best_move.score > current_value.score {
        *current_value = BestMove {
            turn: node.turn,
            local_id: index as u8,
            score: best_move.score,
            stat: current_value.stat + best_move.stat,
        }
    } else {
        current_value.stat = current_value.stat + best_move.stat;
    }
}

fn min_score_move(best_move: BestMove, current_value: &mut BestMove, node: &Node, index: usize) {
    if best_move.score < current_value.score {
        *current_value = BestMove {
            turn: node.turn,
            local_id: index as u8,
            score: best_move.score,
            stat: best_move.stat,
        }
    } else {
        current_value.stat = current_value.stat + best_move.stat;
    }
}
