use std::{
    mem::MaybeUninit,
    ops::{BitOr, Shl},
};

use itertools::{EitherOrBoth, Itertools};

mod grid;

pub use grid::{CheckedYX, Grid, YX};

/// Build the URL to get an AOC input
pub fn get_input_url(year: u16, day: u8) -> String {
    format!("https://adventofcode.com/{}/day/{}/input", year, day)
}

pub trait TryCollectArray {
    /// Try to collect an iterator into an array.
    ///
    /// If the number of elements in the iterator is not **exactly** equal
    /// to the expected output array length, return None.
    fn try_collect_array<const N: usize>(self) -> Option<[Self::Item; N]>
    where
        Self: Iterator + Sized,
    {
        // SAFETY: MaybeUninit can have uninitialized values
        let mut array: [MaybeUninit<Self::Item>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let len = self
            .zip_longest(&mut array)
            .fold(0, |len, tuple| match tuple {
                EitherOrBoth::Both(e, cell) => {
                    cell.write(e);
                    len + 1
                }
                EitherOrBoth::Left(_e) => len + 1,
                EitherOrBoth::Right(_cell) => len,
            });

        if len == N {
            // SAFETY: if len == N, then the entire array have been initialized
            Some(array.map(|e| unsafe { e.assume_init() }))
        } else {
            None
        }
    }
}

impl<I: Iterator + Sized> TryCollectArray for I {}

/// Collect a number of bits into a integer.
/// The last element in the iterator will be the LSB.
/// Previous element will constitute the more significant bits in order.
///
/// If the iterator does not contain the number of bits required, return None.
pub fn collect_n_bits<T>(bits: &mut dyn Iterator<Item = bool>, n: u8) -> Option<T>
where
    bool: Into<T>,
    T: Shl<u8, Output = T> + BitOr<T, Output = T> + Default,
{
    let mut r = T::default();
    for _ in 0..n {
        let b = bits.next()?;
        r = (r << 1) | b.into();
    }
    Some(r)
}

#[macro_export]
macro_rules! unwrap_or_continue {
    ($opt:expr) => {
        if let Some(r) = $opt {
            r
        } else {
            continue;
        }
    };
}
