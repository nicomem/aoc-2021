use std::{ops::RangeInclusive, str::FromStr};

use crate::{unwrap_or_continue, utils::TryCollectArray, Solution};

pub struct Day22;

/// A 3D grid. The `SUM` generic parameter is a workaround until
/// `generic_const_exprs` is stable. It must be set to `X * Y * Z` like so:
/// `Grid3d<_, X, Y, Z, { X * Y * Z }>`
struct Grid3d<T, const X: usize, const Y: usize, const Z: usize, const SUM: usize> {
    data: [T; SUM],
}

/// A wrapper around an axis index, assuring that the value is below the specified bound.
#[derive(Clone, Copy)]
struct AxisIdx<const SIZE: usize>(usize);

impl<const SIZE: usize> AxisIdx<SIZE> {
    /// Verify that the value is between bounds and create the index.
    fn new(v: usize) -> Option<Self> {
        if v < SIZE {
            Some(Self(v))
        } else {
            None
        }
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize, const SUM: usize> Grid3d<T, X, Y, Z, SUM> {
    /// Create a new grid filled with the given value.
    fn new(v: T) -> Self
    where
        T: Copy,
    {
        if X * Y * Z == SUM {
            Self { data: [v; SUM] }
        } else {
            // This could nearly be a compile error, however we cannot change this function
            // to a const fn as T requires to be Copy (#57563)
            panic!("SUM != X * Y * Z")
        }
    }

    /// Get an element of the grid mutably.
    fn get_mut(&mut self, (x, y, z): (AxisIdx<X>, AxisIdx<Y>, AxisIdx<Z>)) -> &mut T {
        let idx = z.0 * (Y * X) + y.0 * X + x.0;
        let cell = self.data.get_mut(idx);

        // SAFETY: We control with `AxisIdx` that each index is within bounds,
        // so the computed index is always within bounds.
        unsafe { cell.unwrap_unchecked() }
    }
}

impl<const X: usize, const Y: usize, const Z: usize, const SUM: usize> Grid3d<bool, X, Y, Z, SUM> {
    /// Count the number of powered on blocks.
    fn count(&self) -> usize {
        self.data.iter().filter(|&&b| b).count()
    }
}

struct Cuboid {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_axis = |s: &str| {
            let (_, s) = s.split_once('=')?;
            let (start, end) = s.split_once("..")?;

            let start = start.parse().ok()?;
            let end = end.parse().ok()?;

            Some(start..=end)
        };

        let [x, y, z] = s.split(',').try_collect_array().ok_or(())?;

        let x = parse_axis(x).ok_or(())?;
        let y = parse_axis(y).ok_or(())?;
        let z = parse_axis(z).ok_or(())?;

        Ok(Self { x, y, z })
    }
}

struct RebootStep {
    power_on: bool,
    area: Cuboid,
}

impl FromStr for RebootStep {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (power_on, cuboid) = s.split_once(' ').ok_or(())?;

        let power_on = power_on == "on";
        let area = cuboid.parse()?;

        Ok(Self { power_on, area })
    }
}

impl Solution for Day22 {
    /// Apply all reboot steps only on cubes between -50 and 50 (on each axis, both values included).
    /// Count the number of cubes powered on.
    fn q1(&self, data: &str) -> String {
        const AX_RANGE: RangeInclusive<i32> = -50..=50;
        const AX: usize = (*AX_RANGE.end() - *AX_RANGE.start()) as usize + 1;

        let steps = Self::parse_data(data);
        let mut grid: Grid3d<bool, AX, AX, AX, { AX * AX * AX }> = Grid3d::new(false);

        let world_to_idx = |range: RangeInclusive<i32>| {
            let start = *range.start().max(AX_RANGE.start());
            let end = *range.end().min(AX_RANGE.end());

            if start > end {
                return None;
            }

            let start = (start - *AX_RANGE.start()) as usize;
            let end = (end - *AX_RANGE.start()) as usize;

            Some((start..=end).map(|i| AxisIdx::<AX>::new(i).unwrap()))
        };

        for RebootStep { power_on, area } in steps {
            let xrange = unwrap_or_continue!(world_to_idx(area.x));
            let yrange = unwrap_or_continue!(world_to_idx(area.y));
            let zrange = unwrap_or_continue!(world_to_idx(area.z));

            for z in zrange {
                for y in yrange.clone() {
                    for x in xrange.clone() {
                        *grid.get_mut((x, y, z)) = power_on;
                    }
                }
            }
        }

        grid.count().to_string()
    }

    /// TODO
    fn q2(&self, data: &str) -> String {
        let _lines = Self::parse_data(data);
        String::new()
    }
}

impl Day22 {
    /// Parse the reboot steps
    fn parse_data(data: &str) -> impl Iterator<Item = RebootStep> + '_ {
        data.lines()
            .map(|line| line.parse().expect("Could not parse input line"))
    }
}
