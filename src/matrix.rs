/* matrix.rs -- matrix utils operations.
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

use super::game::BOARD_SIZE;

/// Count the number of empty cells
pub fn empty_count(m: &[[u8; BOARD_SIZE]; BOARD_SIZE]) -> u8 {
    let mut c = 0u8;
    for j in 0..BOARD_SIZE {
        for i in 0..BOARD_SIZE {
            c += (m[j][i] == 0) as u8;
        }
    }
    c
}

/// Transpose the matrix
pub fn transpose(m: &mut [[u8; BOARD_SIZE]; BOARD_SIZE]) {
    for j in 0..BOARD_SIZE {
        for i in 0..BOARD_SIZE {
            if i > j {
                let tmp = m[j][i];
                m[j][i] = m[i][j];
                m[i][j] = tmp;
            }
        }
    }
}

/// Mirror a matrix horizontally
pub fn mirror_h(m: &mut [[u8; BOARD_SIZE]; BOARD_SIZE]) {
    for j in 0..BOARD_SIZE / 2 {
        let tmp = m[j];
        m[j] = m[BOARD_SIZE - 1 - j];
        m[BOARD_SIZE - 1 - j] = tmp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_test() {
        let mut actual = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        actual[0][1] = 1;
        actual[2][3] = 2;

        transpose(&mut actual);

        let mut excepted = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        excepted[1][0] = 1;
        excepted[3][2] = 2;
        assert_eq!(actual, excepted);
    }

    #[test]
    fn mirror_h_test() {
        let mut actual = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        actual[0][1] = 1;
        actual[1][2] = 2;

        mirror_h(&mut actual);

        let mut excepted = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        excepted[3][1] = 1;
        excepted[2][2] = 2;
        assert_eq!(actual, excepted);
    }
}
