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
pub(super) struct Rnd {
    pub(super) seed: u32,
}

// Linear congruential generator
const MODULUS: u32 = 6075;
const MULTIPLIER: u32 = 106;
const INCREMENT: u32 = 1283;

pub(super) trait RndMove {
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
        let mut rnd = Rnd::new_with_seed(SEED.load(Ordering::Relaxed));
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

    pub(super) fn new_with_seed(seed: u32) -> Rnd {
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

        (value, (next % empty_count as u32) as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
