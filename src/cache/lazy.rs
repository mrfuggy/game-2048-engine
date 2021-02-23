/* lazy.rs -- static lazy init once.
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

use std::cell::UnsafeCell;

pub struct Lazy<T> {
    inner: UnsafeCell<T>,
}

unsafe impl<T> Sync for Lazy<T> {}

impl<T> Lazy<T> {
    /// Create initial value
    pub const fn new(value: T) -> Self {
        Lazy {
            inner: UnsafeCell::new(value),
        }
    }

    /// Set new value in runtime non-thread safe
    pub fn set(&self, value: T) {
        unsafe { *self.inner.get() = value }
    }

    pub fn get(&self) -> &T {
        let raw_ref = self.inner.get();
        unsafe { &*raw_ref }
    }
}
