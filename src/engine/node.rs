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
use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::mem::take;

const BOARD_SIZE: usize = 4;

#[derive(Debug, Default)]
pub(super) struct Node {
    pub(super) board: Board,
    turn: Move,
    pub(super) value: i32,
    pub(super) children: Option<Vec<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Node {}

//TODO estimate the possibility of cutting a node with non full filling or alpha-beta
const PENALTY: i32 = 1_000_000;

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
            stat: Statistics {
                total_nodes: 1,
                cut_nodes: 0,
            },
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
            let mut nodes = if self.turn.is_human() {
                self.next_random_moves(config)
            } else {
                self.next_human_moves()
            };

            if !nodes.is_empty() {
                if config.order_moves {
                    for node in nodes.iter_mut() {
                        node.value = evaluation::evaluate(config.eval_fn, &node);
                    }
                    nodes.sort();
                }
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
    pub(super) fn minimax_alphabeta(
        &mut self,
        config: &EngineConfig,
        depth: u16,
        mut alpha: i32,
        mut beta: i32,
        max_player: bool,
    ) -> BestMove {
        if depth == 0 || self.board.state == State::Lose {
            self.value = evaluation::evaluate(config.eval_fn, self);
            return self.as_terminal_leaf();
        }

        if max_player {
            let nodes = self.gen_next_nodes(config);
            let mut value = BestMove::new(i32::min_value());

            if let Some(ref mut vec) = nodes {
                for (index, node) in vec.iter_mut().enumerate() {
                    let best_move = node.minimax_alphabeta(config, depth - 1, alpha, beta, false);
                    max_score_move(best_move, &mut value, &node, index);

                    alpha = max(alpha, value.score);

                    if alpha >= beta {
                        value.stat.cut_nodes += 1;
                        break;
                    }
                }
                self.value = value.score;
                value
            } else {
                //penalty for losing
                self.value = evaluation::evaluate(config.eval_fn, self) - PENALTY;
                self.as_terminal_leaf()
            }
        } else {
            let nodes = self.gen_next_nodes(config);
            let mut value = BestMove::new(i32::max_value());

            if let Some(ref mut vec) = nodes {
                for (index, node) in vec.iter_mut().enumerate() {
                    let best_move = node.minimax_alphabeta(config, depth - 1, alpha, beta, true);
                    min_score_move(best_move, &mut value, &node, index);

                    beta = min(beta, value.score);
                    if alpha >= beta {
                        value.stat.cut_nodes += 1;
                        break;
                    }
                }
                self.value = value.score;
                value
            } else {
                //penalty for losing
                self.value = evaluation::evaluate(config.eval_fn, self) + PENALTY;
                self.as_terminal_leaf()
            }
        }
    }

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

        let mut closure = |initial, penalty, cmp_fn: fn(BestMove, &mut BestMove, &Node, usize)| {
            self.minimax_part(config, depth, max_player, initial, penalty, cmp_fn)
        };

        if max_player {
            closure(i32::min_value(), -PENALTY, max_score_move)
        } else {
            closure(i32::max_value(), PENALTY, min_score_move)
        }
    }

    fn minimax_part(
        &mut self,
        config: &EngineConfig,
        depth: u16,
        max_player: bool,
        initial: i32,
        penalty: i32,
        cmp_fn: fn(BestMove, &mut BestMove, &Node, usize),
    ) -> BestMove {
        let nodes = self.gen_next_nodes(config);
        let mut value = BestMove::new(initial);

        if let Some(ref mut vec) = nodes {
            for (index, node) in vec.iter_mut().enumerate() {
                let best_move = node.minimax(config, depth - 1, !max_player);
                cmp_fn(best_move, &mut value, &node, index);
            }
            self.value = value.score;
            value
        } else {
            //penalty for losing
            self.value = evaluation::evaluate(config.eval_fn, self) + penalty;
            self.as_terminal_leaf()
        }
    }

    pub(super) fn negamax(&mut self, config: &EngineConfig, depth: u16, color: i8) -> BestMove {
        if depth == 0 || self.board.state == State::Lose {
            self.value = color as i32 * evaluation::evaluate(config.eval_fn, self);
            return self.as_terminal_leaf();
        }

        let nodes = self.gen_next_nodes(config);
        let mut value = BestMove::new(i32::min_value());

        if let Some(ref mut vec) = nodes {
            for (index, node) in vec.iter_mut().enumerate() {
                let best_move = -node.negamax(config, depth - 1, -color);
                max_score_move(best_move, &mut value, &node, index);
            }
            self.value = value.score;
            value
        } else {
            //penalty for losing
            self.value = color as i32 * (evaluation::evaluate(config.eval_fn, self) - PENALTY);
            self.as_terminal_leaf()
        }
    }

    pub(super) fn negamax_alphabeta(
        &mut self,
        config: &EngineConfig,
        depth: u16,
        mut alpha: i32,
        beta: i32,
        color: i8,
    ) -> BestMove {
        if depth == 0 || self.board.state == State::Lose {
            self.value = color as i32 * evaluation::evaluate(config.eval_fn, self);
            return self.as_terminal_leaf();
        }

        let nodes = self.gen_next_nodes(config);
        let mut value = BestMove::new(i32::min_value());

        if let Some(ref mut vec) = nodes {
            for (index, node) in vec.iter_mut().enumerate() {
                debug_assert_ne!(beta, i32::min_value());
                let best_move = -node.negamax_alphabeta(config, depth - 1, -beta, -alpha, -color);
                max_score_move(best_move, &mut value, &node, index);

                alpha = max(alpha, value.score);
                if alpha >= beta {
                    value.stat.cut_nodes += 1;
                    break;
                }
            }
            self.value = value.score;
            value
        } else {
            //penalty for losing
            self.value = color as i32 * (evaluation::evaluate(config.eval_fn, self) - PENALTY);
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
            stat: current_value.stat + best_move.stat,
        }
    } else {
        current_value.stat = current_value.stat + best_move.stat;
    }
}
