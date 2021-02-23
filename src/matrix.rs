/* matrix.rs -- matrix utils operations.
Copyright (C) 2020-2021 fuggy

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

type Matrix = [[u8; BOARD_SIZE]; BOARD_SIZE];

/// Max cell value
pub fn max_cell(m: &Matrix) -> u8 {
    let mut max = 0u8;
    for row in m {
        for cell in row {
            if *cell > max {
                max = *cell;
            }
        }
    }
    max
}

/// Count the number of empty cells
pub fn empty_count(m: &Matrix) -> u8 {
    let mut c = 0u8;
    for row in m {
        for cell in row {
            c += (*cell == 0) as u8;
        }
    }
    c
}

/// Multiply vector by vector
//fn vec_multiply()

/// Sum of absolute value of the difference between pairs
pub fn monotonicity(m: &Matrix) -> i32 {
    let mut c = 0u8;
    //horizontally
    for row in m {
        let mut gt = 0u8;
        let mut eq = 0u8;
        for i in 0..BOARD_SIZE - 1 {
            gt += (row[i] < row[i + 1]) as u8;
            eq += (row[i] == row[i + 1]) as u8;
        }

        //sum == 3 or sum == 0 or qt = 0
        //3 increases or 3 decreases or 3 equal
        c += (((gt + eq) ^ 3) == 0 || gt == 0) as u8;
    }

    //vertically
    for i in 0..BOARD_SIZE {
        let mut gt = 0u8;
        let mut eq = 0u8;
        for j in 0..BOARD_SIZE - 1 {
            gt += (m[j][i] < m[j + 1][i]) as u8;
            eq += (m[j][i] == m[j + 1][i]) as u8;
        }

        //sum == 3 or sum == 0 or qt = 0
        //3 increases or 3 decreases or 3 equal
        c += (((gt + eq) ^ 3) == 0 || gt == 0) as u8;
    }
    c as i32
}

/// Sum of absolute value of the difference between pairs
pub fn smoothness(m: &Matrix) -> i32 {
    let mut c = 0i16;
    //horizontally
    for row in m {
        for i in 0..BOARD_SIZE - 1 {
            //abs bit hack =abs(a)
            let a: i8 = row[i] as i8 - row[i + 1] as i8;
            let mask = a >> 7;
            c += ((a + mask) ^ mask) as i16;
        }
    }

    //vertically
    for j in 0..BOARD_SIZE - 1 {
        for i in 0..BOARD_SIZE {
            //abs bit hack =abs(a)
            let a: i8 = m[j][i] as i8 - m[j + 1][i] as i8;
            let mask = a >> 7;
            c += ((a + mask) ^ mask) as i16;
        }
    }
    c as i32
}

pub fn std_dev(m: &Matrix) -> i32 {
    let mut c = 0u8;
    for row in m {
        for cell in row {
            c += *cell;
        }
    }
    let avg = c as f32 / 16.0;

    let mut sd = 0f32;
    for row in m {
        for cell in row {
            let x = *cell as f32 - avg;
            sd += x * x;
        }
    }
    (sd.sqrt() * 29000.0) as i32
}

const SNAKE_COEFFICIENTS: [[i32; BOARD_SIZE]; BOARD_SIZE] = [
    [140, 120, 110, 115],
    [45, 47, 50, 70],
    [35, 25, 22, 20],
    [2, 3, 5, 10],
];

pub fn snakeiness(m: &Matrix) -> i32 {
    let mut c = 0i32;
    for snake_row in SNAKE_COEFFICIENTS.iter() {
        for row in m {
            c += row
                .iter()
                .zip(snake_row.iter())
                .fold(0i32, |acc, (&a, &b)| acc + a as i32 * b);
        }
    }
    c
}

/// Transpose the matrix
pub fn transpose(m: &mut Matrix) {
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
pub fn mirror_h(m: &mut Matrix) {
    for j in 0..BOARD_SIZE / 2 {
        m.swap(j, BOARD_SIZE - 1 - j);
    }
}

#[allow(dead_code)]
/// Convert to u64 id
pub fn to_u64(m: &Matrix) -> u64 {
    let mut res: u64 = 0;
    for row in m {
        for cell in row {
            res = (res << 4) + *cell as u64;
        }
    }
    res
}

#[allow(dead_code)]
/// Create array from u64
pub fn from_u64(mut pos: u64) -> Matrix {
    let mut m = [[0u8; BOARD_SIZE]; BOARD_SIZE];
    for row in m.iter_mut() {
        for cell in row.iter_mut() {
            *cell = (pos & 0b1111) as u8;
            pos >>= 4;
        }
    }
    m
}

/// Convert to u16 id
pub fn to_u16(m: [u8; BOARD_SIZE]) -> u16 {
    let res: u16 =
        ((m[0] as u16) << 12) + ((m[1] as u16) << 8) + ((m[2] as u16) << 4) + (m[3] as u16);
    res
}

/// Create array from u16
pub fn from_u16(m: &mut [u8; BOARD_SIZE], pos: u16) {
    m[0] = ((pos & 0b1111_0000_0000_0000) >> 12) as u8;
    m[1] = ((pos & 0b0000_1111_0000_0000) >> 8) as u8;
    m[2] = ((pos & 0b0000_0000_1111_0000) >> 4) as u8;
    m[3] = (pos & 0b0000_0000_0000_1111) as u8;
}

/// Convert to u16 id
pub fn to_u16_rev(m: [u8; BOARD_SIZE]) -> u16 {
    let res: u16 =
        (m[0] as u16) + ((m[1] as u16) << 4) + ((m[2] as u16) << 8) + ((m[3] as u16) << 12);
    res
}

/// Create array from u16
pub fn from_u16_rev(m: &mut [u8; BOARD_SIZE], pos: u16) {
    m[0] = (pos & 0b0000_0000_0000_1111) as u8;
    m[1] = ((pos & 0b0000_0000_1111_0000) >> 4) as u8;
    m[2] = ((pos & 0b0000_1111_0000_0000) >> 8) as u8;
    m[3] = ((pos & 0b1111_0000_0000_0000) >> 12) as u8;
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
    fn snakeiness0_test() {
        let board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        let actual = snakeiness(&board);
        assert_eq!(actual, 0);
    }

    #[test]
    fn snakeiness_asc_test() {
        let mut board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        for j in 0..BOARD_SIZE {
            for i in 0..BOARD_SIZE {
                board[j][i] = (j + i + 1) as u8;
            }
        }
        let actual = snakeiness(&board);
        assert_eq!(actual, 13046);
    }

    #[test]
    fn snakeiness_desc_test() {
        let mut board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        for j in 0..BOARD_SIZE {
            for i in 0..BOARD_SIZE {
                board[j][i] = (16 - j + i + 1) as u8;
            }
        }
        let actual = snakeiness(&board);
        assert_eq!(actual, 55634);
    }

    #[test]
    fn snakeiness_full_test() {
        let board = [[16u8; BOARD_SIZE]; BOARD_SIZE];
        let actual = snakeiness(&board);
        assert_eq!(actual, 52416);
    }

    #[test]
    fn std_dev_test() {
        let mut board = [[8u8; BOARD_SIZE]; BOARD_SIZE];
        board[0][0] = 4;
        let actual = std_dev(&board);
        assert_eq!(actual, 112316);
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
