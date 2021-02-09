/* slide_cache.rs -- save/load cache slide lines.
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

use crate::matrix;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Result;
use std::io::Write;

pub struct SlideCache {
    // 45093 zero entry, 20443 actual entry
    pub table: [SlideLine; 65536],
}

#[derive(Debug, Clone, Copy)]
pub struct SlideLine {
    /// output array
    pub line: u16,
    /// range 0,4..65536
    /// u16 posible if store as -1 or 0
    pub score: u16,
}

impl SlideCache {
    /// Create empty cache
    pub const fn new() -> Self {
        SlideCache {
            table: [SlideLine { line: 0, score: 0 }; 65536],
        }
    }

    /// Create and save info file
    pub fn create_cache() {
        let mut cache = SlideCache::new();
        for a in 0..16 {
            for b in 0..16 {
                for c in 0..16 {
                    for d in 0..16 {
                        let array = [a, b, c, d];
                        let id = matrix::to_u16(array);
                        let res = slide_array(array);
                        if let Some(sl) = res {
                            cache.table[id as usize] = sl;
                        }
                    }
                }
            }
        }
        cache.serialize().expect("cannot write file");
    }

    /// Load cache from file
    pub fn load_cache() -> Self {
        let mut cache = SlideCache::new();
        cache.deserialize().expect("cannot read file");
        cache
    }

    /// Write array to file
    fn serialize(&self) -> Result<()> {
        let mut file = BufWriter::new(File::create("slide-cache.bin")?);
        let mut buf: [u8; 5] = [0u8; 5];
        let mut delta = 0;
        for i in 0..self.table.len() {
            if self.table[i].line != 0 {
                buf[0] = (i - delta) as u8;
                delta = i;
                let score = self.table[i].score;
                /*let score = if self.table[i].score == 0 {
                    0
                } else {
                    self.table[i].score - 1
                };*/
                buf[1] = (self.table[i].line >> 8) as u8;
                buf[2] = self.table[i].line as u8;
                buf[3] = (score >> 8) as u8;
                buf[4] = score as u8;
                file.write_all(&buf)?;
            }
        }
        file.flush()?;
        Ok(())
    }

    /// Read cache from file
    fn deserialize(&mut self) -> Result<()> {
        let mut file = BufReader::new(File::open("slide-cache.bin")?);
        let mut buf: [u8; 5] = [0u8; 5];
        let mut delta = 0usize;
        for _i in 0..20443 {
            file.read_exact(&mut buf)?;
            delta += buf[0] as usize;
            let line = ((buf[1] as u16) << 8) + buf[2] as u16;
            let score = ((buf[3] as u16) << 8) + buf[4] as u16;
            self.table[delta] = SlideLine {
                line,
                //score: if score == 0 { 0 } else { score + 1 },
                score,
            };
        }
        Ok(())
    }
}

/// slide one line
fn slide_array(mut m: [u8; 4]) -> Option<SlideLine> {
    let mut moved = false;
    let mut score: u32 = 0;
    for i in 0..3 {
        // move next non zero to current
        if m[i] == 0 {
            // from current to end of line
            for k in i + 1..4 {
                if m[k] != 0 {
                    m[i] = m[k];
                    m[k] = 0;
                    moved = true;
                    break;
                }
            }
        }

        // exit if rest are zeros
        if m[i] == 0 {
            return return_if_moved(moved, m, score);
        }

        for k in i + 1..4 {
            if m[i] == m[k] {
                m[i] += 1;
                if m[i] > 15 {
                    return None;
                }
                m[k] = 0;
                moved = true;
                // add score
                score += 1 << m[i];
                // one merge per cell
                break;
            } else if m[k] != 0 {
                break;
            }
        }
    }

    return_if_moved(moved, m, score)
}

/// don't store non changed moves
fn return_if_moved(moved: bool, m: [u8; 4], score: u32) -> Option<SlideLine> {
    if moved {
        Some(SlideLine {
            line: matrix::to_u16(m),
            score: if score == 0 { 0 } else { (score - 1) as u16 },
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn slide_test1() {
        let a = [0, 1, 0, 1];
        let actual = slide_array(a);
        match actual {
            Some(sl) => {
                assert_eq!(sl.line, matrix::to_u16([2, 0, 0, 0]));
                assert_eq!(sl.score, 4 - 1);
            }
            None => assert!(false),
        }
    }

    #[test]
    fn slide_test2() {
        let a = [1, 1, 1, 1];
        let actual = slide_array(a);
        match actual {
            Some(sl) => {
                assert_eq!(sl.line, matrix::to_u16([2, 2, 0, 0]));
                assert_eq!(sl.score, 8 - 1);
            }
            None => assert!(false),
        }
    }

    #[test]
    fn slide_test3() {
        let a = [2, 2, 1, 1];
        let actual = slide_array(a);
        match actual {
            Some(sl) => {
                assert_eq!(sl.line, matrix::to_u16([3, 2, 0, 0]));
                assert_eq!(sl.score, 12 - 1);
            }
            None => assert!(false),
        }
    }

    #[test]
    fn slide_test4() {
        let a = [2, 0, 0, 0];
        let actual = slide_array(a);
        match actual {
            Some(sl) => {
                assert_eq!(sl.line, matrix::to_u16([2, 0, 0, 0]));
                assert_eq!(sl.score, 0);
            }
            None => assert!(true),
        }
    }

    #[test]
    fn slide_test5() {
        let a = [0, 2, 0, 0];
        let actual = slide_array(a);
        match actual {
            Some(sl) => {
                assert_eq!(sl.line, matrix::to_u16([2, 0, 0, 0]));
                assert_eq!(sl.score, 0);
            }
            None => assert!(false),
        }
    }

    #[test]
    //#[ignore]
    fn serialize_bytes() {
        SlideCache::create_cache();
    }

    #[test]
    //#[ignore]
    fn deserialize_bytes() {
        let cache = SlideCache::load_cache();
        let actual = cache.table[matrix::to_u16([2, 2, 1, 1]) as usize];
        assert_eq!(actual.line, matrix::to_u16([3, 2, 0, 0]));
        assert_eq!(actual.score, 12 - 1);
    }

    #[test]
    //#[ignore]
    fn deserialize_bytes2() {
        let cache = SlideCache::load_cache();
        let actual = cache.table[matrix::to_u16([0, 2, 1, 2]) as usize];
        assert_eq!(actual.line, matrix::to_u16([2, 1, 2, 0]));
        assert_eq!(actual.score, 0);
    }

    #[test]
    //#[ignore]
    fn deserialize_size_bytes() {
        let cache = SlideCache::load_cache();
        let actual = cache.table.iter().filter(|x| x.line != 0).count();
        assert_eq!(actual, 20443);
    }
}
