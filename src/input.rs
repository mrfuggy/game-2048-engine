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

pub fn parse_input(ch: &char) -> Option<Direction> {
    match ch {
        'w' => Some(Direction::Up),
        'a' => Some(Direction::Left),
        's' => Some(Direction::Down),
        'd' => Some(Direction::Right),

        '↑' => Some(Direction::Up),
        '←' => Some(Direction::Left),
        '↓' => Some(Direction::Down),
        '→' => Some(Direction::Right),

        _ => None,
    }
}
