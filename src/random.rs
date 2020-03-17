/* random.rs -- make random number.
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

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
/// Linear congruential generator
pub(super) struct Rnd {
    pub(super) seed: u32,
}

const MODULUS: u32 = 6075;
const MULTIPLIER: u32 = 106;
const INCREMENT: u32 = 1283;

pub(super) trait RndMove {
    /// returns: (value 1 or 2, position in empty cell)
    fn next_move(&mut self, empty_count: u8) -> (u8, u8);
}

static SEED: AtomicU32 = AtomicU32::new(0);
static HAS_INIT: AtomicBool = AtomicBool::new(false);

/// create new 'Rnd' every game
pub(super) fn get_rnd() -> Rnd {
    if !HAS_INIT.compare_and_swap(false, true, Ordering::Relaxed) {
        let rnd = Rnd::new();
        SEED.store(rnd.seed, Ordering::Relaxed);
        rnd
    } else {
        let mut rnd = Rnd::with_seed(SEED.load(Ordering::Relaxed));
        SEED.store(rnd.next(), Ordering::Relaxed);
        rnd
    }
}

impl Rnd {
    pub(super) fn new() -> Rnd {
        let start = SystemTime::now();
        let now = start.duration_since(UNIX_EPOCH);
        Rnd {
            seed: now.unwrap().as_nanos() as u32 % MODULUS,
        }
    }

    pub(super) fn with_seed(seed: u32) -> Rnd {
        Rnd {
            seed: seed as u32 % MODULUS,
        }
    }

    pub(super) fn next(&mut self) -> u32 {
        let next = (self.seed * MULTIPLIER + INCREMENT) % MODULUS;
        self.seed = next;
        next
    }
}

impl RndMove for Rnd {
    fn next_move(&mut self, empty_count: u8) -> (u8, u8) {
        let next = self.next();
        // 10% double value
        let value = if next > (MODULUS - 1) / 10 { 1 } else { 2 };

        (value, (next & (empty_count as u32 - 1)) as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stddev(a: &Vec<i32>) -> f64 {
        let mut count = 0;
        let mut sum = 0;
        for i in a {
            count += 1;
            sum += i;
        }
        let avg = sum as f64 / count as f64;
        let mut var = 0f64;

        for i in a {
            var += (*i as f64 - avg) * (*i as f64 - avg);
        }

        var.sqrt()
    }

    #[test]
    fn next_move_test() {
        let mut rnd = Rnd::new();
        for c in 1..17 {
            for _ in 0..1000 {
                let (value, pos) = rnd.next_move(c);
                assert!(value == 1 || value == 2);
                assert!(pos < c);
            }
        }
    }

    #[test]
    fn next_move_freq_test() {
        let mut rnd = Rnd::new();
        let mut pos_freq = [0; 16];
        let mut value_freq = [0; 2];
        for _ in 0..100000 {
            let (value, pos) = rnd.next_move(16);
            assert!(value == 1 || value == 2);
            assert!(pos < 16);
            pos_freq[pos as usize] += 1;
            value_freq[value as usize - 1] += 1;
        }
        println!("{:?} D: {:.3}", pos_freq, stddev(&pos_freq.to_vec()));
        println!(
            "{:?} R: {:.3}",
            value_freq,
            (value_freq[0] + value_freq[1]) as f64 / value_freq[1] as f64
        );
    }
}
