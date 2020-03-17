/* input.rs -- input direction from console.
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

use super::Direction;

fn parse_input(ch: char) -> Direction {
    match ch {
        'w' => Direction::Up,
        'a' => Direction::Left,
        's' => Direction::Down,
        'd' => Direction::Right,

        '↑' => Direction::Up,
        '←' => Direction::Left,
        '↓' => Direction::Down,
        '→' => Direction::Right,

        _ => panic!("invalid key"),
    }
}
