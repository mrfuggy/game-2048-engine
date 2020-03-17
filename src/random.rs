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

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub(super) struct Rnd {
    pub(super) seed: u32,
}

//Linear congruential generator
const MODULUS: u32 = 6075;
const MULTIPLIER: u32 = 106;
const INCREMENT: u32 = 1283;

pub(super) trait RndMove {
    fn next_move(&mut self, empty_count: u8) -> (u8, u8);
}

impl Rnd {
    pub(super) fn new() -> Rnd {
        let start = SystemTime::now();
        let now = start.duration_since(UNIX_EPOCH);
        Rnd {
            seed: now.unwrap().as_nanos() as u32 % MODULUS,
        }
    }

    pub(super) fn next(&mut self) -> u32 {
        let next = (self.seed * MULTIPLIER + INCREMENT) % MODULUS;
        self.seed = next;
        return next;
    }
}

impl RndMove for Rnd {
    fn next_move(&mut self, empty_count: u8) -> (u8, u8) {
        let next = self.next();
        let value: u8;

        //10% double value
        if next > (MODULUS - 1) / 10 {
            value = 1;
        } else {
            value = 2;
        }

        (value, (next % empty_count as u32) as u8)
    }
}
