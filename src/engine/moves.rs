/* moves.rs -- move struct.
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

use crate::direction::Direction;
use std::collections::HashMap;
use std::ops::Neg;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Move {
    Human(Direction),
    Random(u8, u8),
}

impl Move {
    pub(super) fn from_tuple(tuple: (u8, u8)) -> Move {
        let (value, pos) = tuple;
        Move::Random(value, pos)
    }

    pub(super) fn is_human(self) -> bool {
        match self {
            Move::Human(_) => true,
            Move::Random(_, _) => false,
        }
    }

    pub(super) fn unwrap_random(self) -> (u8, u8) {
        match self {
            Move::Random(value, pos) => (value, pos),
            Move::Human(_) => panic!("unwrap random is not random move"),
        }
    }
}

impl Default for Move {
    fn default() -> Move {
        Move::Random(0, 0)
    }
}

#[derive(Debug)]
pub struct BestMove {
    pub turn: Move,
    pub(super) local_id: u8,
    pub(super) score: i32,
    pub(super) stat: Statistics,
}

impl BestMove {
    pub(super) fn new(score: i32) -> BestMove {
        const EMPTY: Move = Move::Random(0, 0);
        BestMove {
            turn: EMPTY,
            local_id: 0,
            score,
            stat: Statistics::default(),
        }
    }
}

impl Neg for BestMove {
    type Output = Self;
    fn neg(self) -> Self::Output {
        BestMove {
            score: -self.score,
            ..self
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Statistics {
    pub total_nodes: u32,
    pub cut_nodes: u32,
    pub cache_hit: HashMap<u64, u32>,
}

impl Statistics {
    pub fn new(board_id: u64) -> Statistics {
        let mut stat = Statistics {
            total_nodes: 1,
            cut_nodes: 0,
            cache_hit: HashMap::new(),
        };
        stat.cache_hit.insert(board_id, 1);
        stat
    }

    fn merge_maps(map: &mut HashMap<u64, u32>, other: &HashMap<u64, u32>) {
        for (k, v) in other {
            map.entry(*k).and_modify(|x| *x += *v).or_insert(*v);
        }
    }

    pub fn add(&mut self, other: &Statistics) {
        self.total_nodes += other.total_nodes;
        self.cut_nodes += other.cut_nodes;
        Statistics::merge_maps(&mut self.cache_hit, &other.cache_hit);
    }

    pub fn print_cache_stat(&self) {
        let mut cache_stat_vec: Vec<u32> = self.cache_hit.iter().map(|(_k, v)| *v).collect();
        cache_stat_vec.sort();
        cache_stat_vec.reverse();
        let total_len = cache_stat_vec.len();
        let total_sum: u32 = cache_stat_vec.iter().sum();
        cache_stat_vec.truncate(10); //just top 10. historgam is better
        let mut top = cache_stat_vec[0];
        let mut c = 0;
        let mut _sum = 0;
        for value in cache_stat_vec {
            _sum += value;
            if value == top {
                c += 1;
            } else {
                println!("count {:?}={:?} value", top, c);
                c = 1;
                top = value;
            }
        }
        println!("count {:?}={:?} value", top, c);
        println!("total {:?}={:?} values", total_sum, total_len);
    }
}
