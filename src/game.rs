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
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

#[derive(Debug)]
pub struct Game {
    board: [[u8; 4]; 4],
    rnd: Rnd,
    state: State,
    score: u32,
}

pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug)]
pub enum State {
    InGame,
    Lose,
}

pub enum MoveResult {
    Valid,
    InValid,
}

//create new Rnd every game
static SEED: AtomicU32 = AtomicU32::new(0);
static HAS_INIT: AtomicBool = AtomicBool::new(false);

fn get_rnd() -> Rnd {
    if !HAS_INIT.compare_and_swap(false, true, Ordering::Relaxed) {
        let rnd = Rnd::new();
        SEED.store(rnd.seed, Ordering::Relaxed);
        return rnd;
    } else {
        let mut rnd = Rnd::new_with_seed(SEED.load(Ordering::Relaxed));
        SEED.store(rnd.next(), Ordering::Relaxed);
        return rnd;
    }
}

impl Game {
    pub fn start_new() -> Game {
        let rnd = get_rnd();
        let b = [[0u8; 4]; 4];
        let mut start_position = Game {
            board: b,
            rnd: rnd,
            state: State::InGame,
            score: 0,
        };

        //16 empty cell at the beginning
        let next_move = start_position.rnd.next_move(16);
        start_position.set_move(next_move);

        let next_move = start_position.rnd.next_move(15);
        start_position.set_move(next_move);
        return start_position;
    }

    pub fn make_move(&mut self, dir: Direction) {
        self.human_move(dir);
        self.random_move();
    }

    fn human_move(&mut self, dir: Direction) {
        let can_move = self.can_move(dir);
    }

    fn can_move(&self, dir: Direction) -> bool {
        match dir {
            Direction::Right => self.can_move_helper((0, 1)),
            Direction::Left => self.can_move_helper((0, 1)),
            Direction::Up => self.can_move_helper((1, 0)),
            Direction::Down => self.can_move_helper((1, 0)),
        }
    }

    fn can_move_helper(&self, mask: (usize, usize)) -> bool {
        let (x, y) = mask;
        for i in 0..(4 - y) {
            for j in 0..(4 - x) {
                if self.board[j][i] == self.board[j + y][i + x] {
                    return true;
                }
            }
        }
        return false;
    }

    fn random_move(&mut self) {
        let empty_count = self.empty_count();
        let next_move = self.rnd.next_move(empty_count);
        self.set_move(next_move);
    }

    fn set_move(&mut self, gmove: (u8, u8)) {
        let (value, pos) = gmove;
        let mut c = 0u8;
        'outer: for i in 0..4 {
            for j in 0..4 {
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
        for i in 0..4 {
            for j in 0..4 {
                c += (self.board[j][i] == 0) as u8;
            }
        }
        return c;
    }
}
