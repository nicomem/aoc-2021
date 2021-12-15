use std::{
    fmt::{Display, Write},
    ops::Deref,
    str::FromStr,
};

#[derive(Clone)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

pub type YX = (usize, usize);

/// A wrapper around `YX` that attests that the position is inside the grid bounds.
///
/// ** WARNING: Currently does not link a position to a specific grid
/// (can UB when used with a different grid than created with) **
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CheckedYX(YX);

impl CheckedYX {
    pub fn new<T>(grid: &Grid<T>, pos: YX) -> Option<Self> {
        if pos.0 < grid.width && pos.1 < grid.height {
            Some(Self(pos))
        } else {
            None
        }
    }
}

impl Deref for CheckedYX {
    type Target = YX;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Grid<T> {
    /// Get a cell data
    pub fn get(&self, pos: CheckedYX) -> &T {
        let CheckedYX((y, x)) = pos;

        // SAFETY: CheckedYX already checked bounds
        unsafe { self.data.get(x + y * self.width).unwrap_unchecked() }
    }

    /// Get a mutable cell data
    pub fn get_mut(&mut self, pos: CheckedYX) -> &mut T {
        let CheckedYX((y, x)) = pos;

        // SAFETY: CheckedYX already checked bounds
        unsafe { self.data.get_mut(x + y * self.width).unwrap_unchecked() }
    }

    pub fn left(&self, pos: CheckedYX) -> Option<CheckedYX> {
        let CheckedYX((y, x)) = pos;
        (x > 0).then(|| CheckedYX((y, x - 1)))
    }

    pub fn right(&self, pos: CheckedYX) -> Option<CheckedYX> {
        let CheckedYX((y, x)) = pos;
        (x + 1 < self.width).then(|| CheckedYX((y, x + 1)))
    }

    pub fn top(&self, pos: CheckedYX) -> Option<CheckedYX> {
        let CheckedYX((y, x)) = pos;
        (y > 0).then(|| CheckedYX((y - 1, x)))
    }

    pub fn bottom(&self, pos: CheckedYX) -> Option<CheckedYX> {
        let CheckedYX((y, x)) = pos;
        (y + 1 < self.height).then(|| CheckedYX((y + 1, x)))
    }

    /// Create an iterator over the (y, x) coordinates.
    pub fn coordinates(&self) -> impl Iterator<Item = CheckedYX> {
        let height = self.height;
        let width = self.width;
        (0..height).flat_map(move |y| (0..width).map(move |x| CheckedYX((y, x))))
    }
}

impl<T> FromStr for Grid<T>
where
    u32: TryInto<T>,
{
    type Err = ();

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut lines = data.split_terminator('\n').map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().map_err(|_| ()).unwrap())
        });

        // Extract the first line to get the width
        let mut data = lines.next().unwrap().collect::<Vec<_>>();
        let width = data.len();

        // Extract the rest of the grid
        data.extend(lines.flatten());
        let height = data.len() / width;

        Ok(Grid {
            data,
            width,
            height,
        })
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut old_y = 0;
        for pos @ CheckedYX((y, _x)) in self.coordinates() {
            if y != old_y {
                f.write_char('\n')?;
                old_y = y;
            }
            let v = self.get(pos);
            write!(f, "{} ", *v)?;
        }

        Ok(())
    }
}
