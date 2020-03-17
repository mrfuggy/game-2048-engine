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

use crate::board;
use crate::board::{Board, State};
use crate::direction::Direction;
use crate::random;
use crate::random::{Rnd, RndMove};

pub struct Game {
    pub board: Board,
    rnd: Rnd,
}

impl Game {
    /// Create new start position defined by seed
    pub fn with_seed(seed: u32) -> Game {
        let mut start_position = Game {
            board: Board::new(),
            rnd: Rnd::with_seed(seed),
        };

        start_position.init_new();
        start_position
    }

    /// Create new start position
    pub fn start_new() -> Game {
        let mut start_position = Game {
            board: Board::new(),
            rnd: random::get_rnd(),
        };

        start_position.init_new();
        start_position
    }

    fn init_new(&mut self) {
        const CELL_COUNT: u8 = (board::BOARD_SIZE * board::BOARD_SIZE) as u8;
        // 'CELL_COUNT' empty cell at the beginning
        let next_move = self.rnd.next_move(CELL_COUNT);
        self.board.set_move(next_move);

        let next_move = self.rnd.next_move(CELL_COUNT - 1);
        self.board.set_move(next_move);
        self.board.move_count = 0;
    }

    /// Make human move then random move
    pub fn make_move(&mut self, dir: Direction) -> bool {
        let move_made = self.human_move(dir);
        if move_made {
            self.random_move();
        }
        move_made
    }

    /// Make human move
    /// returns: change has been made
    pub fn human_move(&mut self, dir: Direction) -> bool {
        if self.board.state == State::Lose {
            return false;
        }

        let can_move = self.board.empty_count() > 0u8 || self.board.can_move_dir(dir);
        if !can_move {
            return false;
        }

        self.board.slide_to(dir)
    }

    /// Put random value in an empty spot
    pub fn random_move(&mut self) -> Option<(u8, u8)> {
        if self.board.state == State::Lose {
            return None;
        }

        let empty_count = self.board.empty_count();
        let next_move = self.rnd.next_move(empty_count);
        self.board.set_move(next_move);

        if !self.board.can_move() {
            self.board.state = State::Lose;
        }

        Some(next_move)
    }

    /// Count the number of empty cells
    pub fn empty_count(&self) -> u8 {
        self.board.empty_count()
    }

    /// Count the number of empty cells
    pub fn max_cell(&self) -> u16 {
        self.board.max_cell()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn should_can_play() {
        let mut game = Game::start_new();
        game.board.board =
           [[1, 2, 1, 2],
            [2, 1, 2, 1],
            [1, 0, 1, 2],
            [2, 1, 2, 1]];

        assert!(game.board.can_move());

        let mut game = Game::start_new();
        game.board.board =
           [[3, 2, 1, 1],
            [1, 1, 2, 2],
            [3, 2, 1, 1],
            [2, 1, 2, 1]];

        assert!(game.board.can_move());
    }

    #[test]
    #[rustfmt::skip]
    fn should_lose_game() {
        let mut game = Game::start_new();
        game.board.board =
           [[1, 2, 1, 2],
            [2, 1, 2, 1],
            [1, 2, 1, 2],
            [2, 1, 2, 1]];

        assert!(!game.board.can_move());
    }

    #[test]
    #[rustfmt::skip]
    fn should_slide_left_with_zero_start() {
        let mut game = Game::start_new();
        game.board.board =
           [[1, 2, 1, 2],
            [0, 1, 2, 1],
            [1, 2, 1, 2],
            [2, 1, 2, 1]];

        let moved = game.human_move(Direction::Left);
        assert!(moved);
    }

    #[test]
    #[rustfmt::skip]
    fn should_slide_left_with_zero_end() {
        let mut game = Game::start_new();
        game.board.board =
           [[1, 2, 1, 2],
            [2, 1, 2, 0],
            [1, 2, 1, 2],
            [2, 1, 2, 1]];

        let moved = game.human_move(Direction::Left);
        assert!(!moved);
    }

    #[test]
    #[rustfmt::skip]
    fn should_slide_left_with_merge() {
        let mut game = Game::start_new();
        game.board.board =
           [[3, 2, 1, 1],
            [1, 1, 2, 2],
            [3, 2, 1, 1],
            [2, 1, 2, 1]];

        let moved = game.human_move(Direction::Left);
        assert!(moved);
    }

    #[test]
    #[rustfmt::skip]
    fn should_slide_right_with_zero_start() {
        let mut game = Game::start_new();
        game.board.board =
           [[1, 2, 1, 2],
            [2, 1, 2, 0],
            [1, 2, 1, 2],
            [2, 1, 2, 1]];

        let moved = game.human_move(Direction::Right);
        assert!(moved);
    }

    #[test]
    #[rustfmt::skip]
    fn should_slide_right_with_zero_end() {
        let mut game = Game::start_new();
        game.board.board =
           [[1, 2, 1, 2],
            [0, 1, 2, 1],
            [1, 2, 1, 2],
            [2, 1, 2, 1]];

        let moved = game.human_move(Direction::Right);
        assert!(!moved);
    }

    #[test]
    #[rustfmt::skip]
    fn should_slide_right_with_merge() {
        let mut game = Game::start_new();
        game.board.board =
           [[3, 2, 1, 1],
            [1, 1, 2, 2],
            [3, 2, 1, 1],
            [2, 1, 2, 1]];

        let moved = game.human_move(Direction::Right);
        assert!(moved);
    }

    #[test]
    #[rustfmt::skip]
    fn should_slide_up_with_zero_start() {
        let mut game = Game::start_new();
        game.board.board =
           [[1, 2, 0, 2],
            [2, 1, 2, 1],
            [1, 2, 1, 2],
            [2, 1, 2, 1]];

        let moved = game.human_move(Direction::Up);
        assert!(moved);
    }

    #[test]
    #[rustfmt::skip]
    fn should_slide_up_with_zero_end() {
        let mut game = Game::start_new();
        game.board.board =
           [[1, 2, 1, 2],
            [2, 1, 2, 1],
            [1, 2, 1, 2],
            [2, 1, 0, 1]];

        let moved = game.human_move(Direction::Up);
        assert!(!moved);
    }

    #[test]
    #[rustfmt::skip]
    fn should_slide_up_with_merge() {
        let mut game = Game::start_new();
        game.board.board =
           [[1, 3, 2, 3],
            [2, 1, 2, 1],
            [1, 2, 1, 2],
            [2, 1, 2, 1]];

        let moved = game.human_move(Direction::Up);
        assert!(moved);
    }

    #[test]
    #[rustfmt::skip]
    fn should_slide_down_with_zero_start() {
        let mut game = Game::start_new();
        game.board.board =
           [[1, 2, 1, 2],
            [2, 1, 2, 1],
            [1, 2, 1, 2],
            [2, 1, 0, 1]];

        let moved = game.human_move(Direction::Down);
        assert!(moved);
    }

    #[test]
    #[rustfmt::skip]
    fn should_slide_down_with_zero_end() {
        let mut game = Game::start_new();
        game.board.board =
           [[1, 2, 0, 2],
            [2, 1, 2, 1],
            [1, 2, 1, 2],
            [2, 1, 2, 1]];

        let moved = game.human_move(Direction::Down);
        assert!(!moved);
    }

    #[test]
    #[rustfmt::skip]
    fn should_slide_down_with_merge() {
        let mut game = Game::start_new();
        game.board.board =
           [[1, 2, 1, 2],
            [2, 1, 2, 1],
            [1, 2, 1, 2],
            [2, 3, 1, 3]];

        let moved = game.human_move(Direction::Down);
        assert!(moved);
    }

    #[test]
    #[rustfmt::skip]
    fn should_slide_down_with_merge2() { //TODO
        let mut game = Game::start_new();
        game.board.board =
           [[3, 4, 2, 1],
            [3, 6, 4, 3],
            [4, 8, 6, 4],
            [1, 3, 7, 1]];

        let moved = game.human_move(Direction::Down);
        assert!(moved);
    }
}
