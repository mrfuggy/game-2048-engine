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

use crate::direction::Direction;
use crate::engine::engine_config::EngineConfig;
use crate::engine::evaluation;
use crate::game::Game;
use crate::game::State;
use std::cmp::max;
use std::cmp::min;

const BOARD_SIZE: usize = 4;

pub(crate) struct Node {
    pub(crate) board: [[u8; BOARD_SIZE]; BOARD_SIZE],
    turn: Move,
    state: NodeState,
    value: i32,
    pub(crate) score: u32,
    pub(crate) move_count: u16,
    childred: Option<Box<Vec<Node>>>,
}

#[derive(PartialEq)]
enum NodeState {
    NotEnded,
    Illegal,
    Terminal,
}

impl NodeState {
    fn from_game_state(state: &State) -> NodeState {
        match state {
            State::InGame => NodeState::NotEnded,
            State::Lose => NodeState::Terminal,
        }
    }
}

#[derive(Clone, Copy)]
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

/*fn get_node(node: &Node, local_id: u8) -> &Node {
    match &node.childred {
        None => node,
        Some(vec) => &vec[local_id as usize],
    }
}*/

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
            state: NodeState::from_game_state(&game.state),
            value: 0,
            score: game.score,
            move_count: game.move_count,
            childred: None,
        }
    }

    fn from_next_random(&self, new_board: [[u8; BOARD_SIZE]; BOARD_SIZE], game_move: Move) -> Node {
        Node {
            board: new_board,
            turn: game_move,
            state: NodeState::NotEnded,
            value: 0,
            score: self.score,
            move_count: self.move_count,
            childred: None,
        }
    }

    fn gen_next_nodes(&mut self, config: &EngineConfig) -> &mut Box<Vec<Node>> {
        if self.childred.is_none() {
            let nodes = if self.turn.is_human() {
                self.next_human_moves()
            } else {
                self.next_random_moves()
            };
            self.childred = Some(Box::new(nodes));
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
            let moved = game.human_move(dir.clone());
            if moved {
                let node = Node::from_game(&game, Move::Human(*dir));
                nodes.push(node);
            }
        }
        nodes
    }

    fn next_random_moves(&self) -> Vec<Node> {
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
                if self.board[j][i] == 0 {
                    let mut new_board = self.board.clone();
                    new_board[j][i] = value;
                    let node = self.from_next_random(new_board, Move::Random(c, 1));
                    nodes.push(node);
                    c += 1;
                }
            }
        }
    }
}

//algorithms
//impl Engine {
impl Node {
    pub(super) fn minimax(&mut self, config: &EngineConfig, depth: u16, max_player: bool) -> i32 {
        if depth == 0 || self.state == NodeState::Terminal {
            self.value = evaluation::evaluate(config.eval_fn, self);
            return self.value; //(node.turn, node.score);
        }

        if max_player {
            let mut value = -1;
            let nodes = self.gen_next_nodes(config);
            for node in nodes.iter_mut() {
                value = max(value, node.minimax(config, depth - 1, false));
            }
            self.value = value;
            self.value
        } else {
            let mut value = 1;
            let nodes = self.gen_next_nodes(config);
            for node in nodes.iter_mut() {
                value = max(value, node.minimax(config, depth - 1, true));
            }
            self.value = value;
            self.value
        }
        //(Move::Human(Direction::Left), 0)
    }
}
