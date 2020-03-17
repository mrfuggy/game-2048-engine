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

use super::board::BOARD_SIZE;

/// Max cell value
pub fn max_cell(m: &[[u8; BOARD_SIZE]; BOARD_SIZE]) -> u8 {
    let mut c = 0u8;
    for j in 0..BOARD_SIZE {
        for i in 0..BOARD_SIZE {
            if m[j][i] > c {
                c = m[j][i];
            }
        }
    }
    c
}

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

/// Sum of absolute value of the difference between pairs
pub fn monotonicity(m: &[[u8; BOARD_SIZE]; BOARD_SIZE]) -> i32 {
    let mut c = 0;
    //horizontally
    for j in 0..BOARD_SIZE {
        let gt = (m[j][0] < m[j][1]) as u8 + (m[j][1] < m[j][2]) as u8 + (m[j][2] < m[j][3]) as u8;
        let eq =
            (m[j][0] == m[j][1]) as u8 + (m[j][1] == m[j][2]) as u8 + (m[j][2] == m[j][3]) as u8;

        if gt + eq == 3 || gt + eq == 0 || gt == 0 {
            c += 1;
        }
    }
    //vertically
    for i in 0..BOARD_SIZE {
        let gt = (m[0][i] < m[1][i]) as u8 + (m[1][i] < m[2][i]) as u8 + (m[2][i] < m[3][i]) as u8;
        let eq =
            (m[0][i] == m[1][i]) as u8 + (m[1][i] == m[2][i]) as u8 + (m[2][i] == m[3][i]) as u8;

        if gt + eq == 3 || gt + eq == 0 || gt == 0 {
            c += 1;
        }
    }
    c
}

/// Sum of absolute value of the difference between pairs
pub fn smoothness(m: &[[u8; BOARD_SIZE]; BOARD_SIZE]) -> i32 {
    let mut c = 0i32;
    //horizontally
    for j in 0..BOARD_SIZE {
        for i in 0..BOARD_SIZE - 1 {
            c += (m[j][i] as i32 - m[j][i + 1] as i32).abs();
        }
    }
    //vertically
    for j in 0..BOARD_SIZE - 1 {
        for i in 0..BOARD_SIZE {
            c += (m[j][i] as i32 - m[j + 1][i] as i32).abs();
        }
    }
    c
}

#[allow(dead_code)]
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

#[allow(dead_code)]
/// Mirror a matrix horizontally
pub fn mirror_h(m: &mut [[u8; BOARD_SIZE]; BOARD_SIZE]) {
    for j in 0..BOARD_SIZE / 2 {
        m.swap(j, BOARD_SIZE - 1 - j);
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
    fn smoothness_monotone0_test() {
        let board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        let actual = monotonicity(&board);
        assert_eq!(actual, 8);
    }

    #[test]
    fn smoothness_monotone1_test() {
        let mut board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        board[0] = [0, 0, 1, 1];
        let actual = monotonicity(&board);
        assert_eq!(actual, 8);
    }

    #[test]
    fn smoothness_monotone2_test() {
        let mut board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        board[0] = [0, 0, 1, 2];
        let actual = monotonicity(&board);
        assert_eq!(actual, 8);
    }

    #[test]
    fn smoothness_monotone3_test() {
        let mut board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        board[0] = [1, 1, 0, 0];
        let actual = monotonicity(&board);
        assert_eq!(actual, 8);
    }

    #[test]
    fn smoothness_monotone4_test() {
        let mut board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        board[0] = [2, 1, 0, 0];
        let actual = monotonicity(&board);
        assert_eq!(actual, 8);
    }

    #[test]
    fn smoothness_monotone6_test() {
        let mut board = [[16u8; BOARD_SIZE]; BOARD_SIZE];
        board[0] = [2, 1, 0, 0];
        let actual = monotonicity(&board);
        assert_eq!(actual, 8);
    }

    #[test]
    fn smoothness_monotone7_test() {
        let mut board = [[16u8; BOARD_SIZE]; BOARD_SIZE];
        board[0] = [0, 0, 1, 2];
        let actual = monotonicity(&board);
        assert_eq!(actual, 8);
    }

    #[test]
    fn smoothness_nonmonotone0_test() {
        let mut board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        board[0] = [0, 0, 1, 0];
        let actual = monotonicity(&board);
        assert_eq!(actual, 7);
    }

    #[test]
    fn smoothness_nonmonotone1_test() {
        let mut board = [[16u8; BOARD_SIZE]; BOARD_SIZE];
        board[0] = [0, 0, 1, 0];
        let actual = monotonicity(&board);
        assert_eq!(actual, 7);
    }

    #[test]
    fn smoothness_smooth0_test() {
        let board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        let actual = smoothness(&board);
        assert_eq!(actual, 0);
    }

    #[test]
    fn smoothness_smooth16_test() {
        let board = [[16u8; BOARD_SIZE]; BOARD_SIZE];
        let actual = smoothness(&board);
        assert_eq!(actual, 0);
    }

    #[test]
    fn smoothness_unsmooth_test() {
        let mut board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        for j in 0..BOARD_SIZE {
            for i in 0..BOARD_SIZE {
                if j + i & 1 == 1 {
                    board[j][i] = 16;
                }
            }
        }
        let actual = smoothness(&board);
        assert_eq!(actual, 384);
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
