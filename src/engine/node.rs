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
use crate::game::Game;
use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::ops::Neg;

const BOARD_SIZE: usize = 4;

#[derive(Debug)]
pub(crate) struct Node {
    pub(crate) board: Board,
    turn: Move,
    value: BestMove,
    childred: Option<Vec<Node>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Human(Direction),
    Random(u8, u8),
}

impl Move {
    fn is_human(&self) -> bool {
        match self {
            Move::Human(_) => true,
            Move::Random(_, _) => false,
        }
    }
    fn is_random(&self) -> bool {
        !self.is_human()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BestMove(pub Move, i32);

impl BestMove {
    fn new(score: i32) -> BestMove {
        const EMPTY: Move = Move::Random(0, 0);
        BestMove(EMPTY, score)
    }
}

impl Ord for BestMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
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
        self.1 == other.1
    }
}

impl Neg for BestMove {
    type Output = BestMove;
    fn neg(self) -> Self::Output {
        BestMove(self.0, -self.1)
    }
}

const DIRICTION_CYCLE: [Direction; 4] = [
    Direction::Left,
    Direction::Right,
    Direction::Down,
    Direction::Up,
];

impl Node {
    pub(super) fn from_game(game: &Game, game_move: Move) -> Node {
        Node {
            board: game.board,
            turn: game_move,
            value: BestMove::new(0),
            childred: None,
        }
    }

    fn with_board(new_board: Board, game_move: Move) -> Node {
        Node {
            board: new_board,
            turn: game_move,
            value: BestMove::new(0),
            childred: None,
        }
    }

    fn gen_next_nodes(&mut self, config: &EngineConfig) -> &mut Vec<Node> {
        if self.childred.is_none() {
            let nodes = if self.turn.is_human() {
                self.next_random_moves(config)
            } else {
                self.next_human_moves()
            };
            self.childred = Some(nodes);
        }

        match self.childred {
            Some(ref mut vec) => vec,
            _ => unreachable!(),
        }
    }

    fn next_human_moves(&self) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::with_capacity(4);
        for dir in &DIRICTION_CYCLE {
            let mut game = Game::from_node(self);
            let moved = game.human_move(*dir);
            if moved {
                let node = Node::from_game(&game, Move::Human(*dir));
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
            self.value = BestMove(self.turn, score);
            return self.value;
        }

        if max_player {
            let mut value = BestMove::new(-1000);
            let nodes = self.gen_next_nodes(config);
            for node in nodes.iter_mut() {
                value = max(node.minimax(config, depth - 1, false), value);
            }
            self.value = value;
            self.value
        } else {
            let mut value = BestMove::new(1000);
            let nodes = self.gen_next_nodes(config);

            for node in nodes.iter_mut() {
                value = min(node.minimax(config, depth - 1, true), value);
            }
            self.value = value;
            self.value
        }
    }

    pub(super) fn negamax(&mut self, config: &EngineConfig, depth: u16, color: i8) -> BestMove {
        if depth == 0 || self.board.state == State::Lose {
            let score = color as i32 * evaluation::evaluate(config.eval_fn, self);
            self.value = BestMove(self.turn, score);
            return self.value;
        }

        let mut value = BestMove::new(-1000);
        let nodes = self.gen_next_nodes(config);
        for node in nodes.iter_mut() {
            value = max(-node.negamax(config, depth - 1, -color), value);
        }
        self.value = value;
        self.value
    }
}
