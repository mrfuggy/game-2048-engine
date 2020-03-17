/* game.rs -- manage the game process.
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

use super::random::{Rnd, RndMove};
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

#[derive(Debug)]
pub struct Game {
    board: [[u8; 4]; 4],
    rnd: Rnd,
    state: State,
    score: u32,
    move_count: u16,
}

pub const BOARD_SIZE: usize = 4;
const CELL_COUNT: u8 = (BOARD_SIZE as u8) * (BOARD_SIZE as u8);

#[derive(Copy, Clone)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn get_mask(&self) -> (usize, usize) {
        match self {
            Direction::Right => (0usize, 1usize),
            Direction::Left => (0usize, 1usize),
            Direction::Up => (1usize, 0usize),
            Direction::Down => (1usize, 0usize),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    InGame,
    Lose,
}

//create new 'Rnd' every game
static SEED: AtomicU32 = AtomicU32::new(0);
static HAS_INIT: AtomicBool = AtomicBool::new(false);

fn get_rnd() -> Rnd {
    if !HAS_INIT.compare_and_swap(false, true, Ordering::Relaxed) {
        let rnd = Rnd::new();
        SEED.store(rnd.seed, Ordering::Relaxed);
        rnd
    } else {
        let mut rnd = Rnd::new_with_seed(SEED.load(Ordering::Relaxed));
        SEED.store(rnd.next(), Ordering::Relaxed);
        rnd
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "score: {}", self.score)?;

        for j in 0..BOARD_SIZE {
            for i in 0..BOARD_SIZE {
                let value = if self.board[j][i] != 0 {
                    2 << self.board[j][i] - 1
                } else {
                    0
                };
                write!(f, "{:>6}", value)?
            }
            writeln!(f)?
        }
        write!(f, "")
    }
}

impl Game {
    pub fn start_new() -> Game {
        let rnd = get_rnd();
        let b = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        let mut start_position = Game {
            board: b,
            rnd: rnd,
            state: State::InGame,
            score: 0,
            move_count: 0,
        };

        //'CELL_COUNT' empty cell at the beginning
        let next_move = start_position.rnd.next_move(CELL_COUNT);
        start_position.set_move(next_move);

        let next_move = start_position.rnd.next_move(CELL_COUNT - 1);
        start_position.set_move(next_move);
        start_position
    }

    pub fn make_move(&mut self, dir: Direction) {
        let move_made = self.human_move(dir);
        if move_made {
            self.random_move();
        }
    }

    pub fn human_move(&mut self, dir: Direction) -> bool {
        if self.state == State::Lose {
            return false;
        }

        let can_move = self.empty_count() > 0u8 || self.can_move_dir(dir);
        if !can_move {
            return false;
        }

        let score = self.score;
        self.slide_to(dir);
        //extra check when closing zeros
        if score != self.score {
            self.move_count += 1;
            return true;
        }

        //todo
        return true;
    }

    fn slide_to(&mut self, _dir: Direction) {
        for j in 0..BOARD_SIZE {
            for i in 0..BOARD_SIZE - 1 {
                //move next non zero to current
                if self.board[j][i] == 0 {
                    //from current to end of line
                    for k in i + 1..BOARD_SIZE {
                        if self.board[j][k] != 0 {
                            self.board[j][i] = self.board[j][k];
                            self.board[j][k] = 0;
                            break;
                        }
                    }
                }

                //return if rest are zeros
                if self.board[j][i] == 0 {
                    break;
                }

                for k in i + 1..BOARD_SIZE {
                    if self.board[j][i] == self.board[j][k] {
                        self.board[j][i] += 1;
                        self.board[j][k] = 0;
                        //add score
                        self.score += 2 << self.board[j][i] - 1;
                        //one merge per cell
                        break;
                    } else if self.board[j][k] != 0 {
                        break;
                    }
                }
            }
        }
    }

    fn can_move(&self) -> bool {
        self.empty_count() > 0u8
            // two perpendicular enough
            || self.can_move_dir(Direction::Left)
            || self.can_move_dir(Direction::Up)
    }

    fn can_move_dir(&self, dir: Direction) -> bool {
        let (x, y) = dir.get_mask();
        for j in 0..BOARD_SIZE - y {
            for i in 0..BOARD_SIZE - x {
                if self.board[j][i] == self.board[j + y][i + x] {
                    return true;
                }
            }
        }
        false
    }

    fn random_move(&mut self) {
        if self.state == State::Lose {
            return;
        }

        if !self.can_move() {
            self.state = State::Lose;
            return;
        }

        let empty_count = self.empty_count();
        let next_move = self.rnd.next_move(empty_count);
        self.set_move(next_move);
    }

    fn set_move(&mut self, game_move: (u8, u8)) {
        let (value, pos) = game_move;
        let mut c = 0u8;
        'outer: for j in 0..BOARD_SIZE {
            for i in 0..BOARD_SIZE {
                if self.board[j][i] == 0 {
                    if c == pos {
                        self.board[j][i] = value;
                        break 'outer;
                    }
                    c += 1;
                }
            }
        }
    }

    fn empty_count(&self) -> u8 {
        let mut c = 0u8;
        for j in 0..BOARD_SIZE {
            for i in 0..BOARD_SIZE {
                c += (self.board[j][i] == 0) as u8;
            }
        }
        c
    }
}
