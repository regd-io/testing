// Copyright 2025 Shingo OKAWA. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module contains a set of extensions of the existing Rust types.

/// A trait providing extension methods for slices.
///
/// This trait adds several useful methods for working with slices. It provides:
/// - [`choose`]: Randomly selects an element from the slice.
/// - [`choose_mut`]: Randomly selects and mutably borrows an element from the slice.
/// - [`shuffle`]: Shuffles the slice in place.
///
/// These methods operate on slices of any type `T` and assume that `T` is a type
/// that can be accessed and modified within the slice.
///
/// # Examples
/// ```
/// use regd_testing::prelude::*;
///
/// let mut numbers = [1, 2, 3, 4, 5];
/// numbers.shuffle();
/// println!("Shuffled numbers: {:?}", numbers);
///
/// if let Some(choice) = numbers.choose() {
///     println!("Random choice: {}", choice);
/// }
///
/// if let Some(choice) = numbers.choose_mut() {
///     *choice = 10;
///     println!("Modified choice: {}", choice);
/// }
/// ```
///
/// [`choose`]: Self::choose
/// [`choose_mut`]: Self::choose_mut
/// [`shuffle`]: Self::shuffle
pub trait SliceExt {
    /// The type of elements in the slice.
    type Item;

    /// Randomly selects an element from the slice.
    ///
    /// # Returns
    /// - `Some(&Self::Item)` if the slice is non-empty.
    /// - `None` if the slice is empty.
    fn choose(&self) -> Option<&Self::Item>;

    /// Randomly selects and mutably borrows an element from the slice.
    ///
    /// # Returns
    /// - `Some(&mut Self::Item)` if the slice is non-empty.
    /// - `None` if the slice is empty.
    fn choose_mut(&mut self) -> Option<&mut Self::Item>;

    /// Shuffles the elements of the slice in place.
    ///
    /// This method shuffles the slice, reordering its elements randomly.
    fn shuffle(&mut self);
}

/// Generates a random index within the specified upper bound.
///
/// This function returns a random integer between 0 (inclusive) and `sup` (exclusive),
/// using a range-based random number generation strategy. It intelligently chooses between
/// using a `u32` or `usize` bound based on the value of `sup`, ensuring the appropriate
/// range is selected for generating random numbers.
///
/// # Parameters
/// - `sup`: The upper bound (exclusive) for the generated random index. The function
///   handles both small and large upper bounds, using a `u32` range for smaller values
///   and a `usize` range for larger values.
///
/// # Returns
/// - A random `usize` integer in the range `[0, sup)`.
#[inline]
fn generate_index(sup: usize) -> usize {
    if sup <= (u32::MAX as usize) {
        crate::rand::generate_range(0..sup as u32) as usize
    } else {
        crate::rand::generate_range(0..sup)
    }
}

impl<T> SliceExt for [T] {
    type Item = T;

    fn choose(&self) -> Option<&Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(&self[generate_index(self.len())])
        }
    }

    fn choose_mut(&mut self) -> Option<&mut Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(&mut self[generate_index(self.len())])
        }
    }

    fn shuffle(&mut self) {
        for i in (1..self.len()).rev() {
            self.swap(i, generate_index(i + 1));
        }
    }
}
