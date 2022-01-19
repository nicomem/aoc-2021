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
    /// Check that the coordinate is valid for the given grid.
    pub fn new<T>(grid: &Grid<T>, (y, x): YX) -> Option<Self> {
        if y < grid.height && x < grid.width {
            Some(Self((y, x)))
        } else {
            None
        }
    }

    /// Same as [`CheckedYX::new`], but can take signed coordinates.
    /// As grid coordinates are not valid for negative values, will return None for any negative value.
    pub fn new_signed<T>(grid: &Grid<T>, (y, x): (isize, isize)) -> Option<Self> {
        if y < 0 || x < 0 {
            None
        } else {
            Self::new(grid, (y as usize, x as usize))
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

    /// Parse a 2D string of characters into a grid.
    pub fn from_str_map<'a>(
        mut lines: impl Iterator<Item = &'a str>,
        mut map_char: impl FnMut(char) -> T,
    ) -> Self {
        // Extract and process the first line to get the width
        let mut data: Vec<_> = lines.next().unwrap().chars().map(&mut map_char).collect();
        let width = data.len();

        // Extract the rest of the grid and add it to the data vector
        let chars = lines.flat_map(|line| line.chars()).map(map_char);
        data.extend(chars);

        let height = data.len() / width;
        Grid {
            data,
            width,
            height,
        }
    }

    /// Create a new grid with different sizes than the current.
    /// The data copy begins at the given coordinate and may be truncated if go out of bounds.
    /// Vacant cells will be filled with the specified data element.
    #[must_use]
    pub fn resized(&self, height: usize, width: usize, at: YX, fill: T) -> Self
    where
        T: Clone,
    {
        // Create a new grid filled with the default element
        let mut new_grid = Self {
            data: vec![fill; height * width],
            width,
            height,
        };

        // Copy back the original grid data to the new one
        let (y0, x0) = at;
        for yd in 0..self.height {
            for xd in 0..self.width {
                if let Some(yx) = CheckedYX::new(&new_grid, (y0 + yd, x0 + xd)) {
                    let v = self.get(CheckedYX::new(self, (yd, xd)).unwrap()).clone();
                    *new_grid.get_mut(yx) = v;
                }
            }
        }

        new_grid
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
