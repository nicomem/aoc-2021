use std::ops::{Index, IndexMut};

use crate::Solution;

pub struct Day11;

#[derive(Clone)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

type YX = (usize, usize);

impl<T> Grid<T> {
    /// Try to get a cell data. Returns None if invalid.
    fn get(&self, (y, x): YX) -> Option<&T> {
        if x < self.width && y < self.height {
            self.data.get(x + y * self.width)
        } else {
            None
        }
    }

    /// Try to get a cell data. Returns None if invalid.
    fn get_mut(&mut self, (y, x): YX) -> Option<&mut T> {
        if x < self.width && y < self.height {
            self.data.get_mut(x + y * self.width)
        } else {
            None
        }
    }

    fn left(&self, (y, x): YX) -> Option<YX> {
        (x > 0).then(|| (y, x - 1))
    }

    fn right(&self, (y, x): YX) -> Option<YX> {
        (x + 1 < self.width).then(|| (y, x + 1))
    }

    fn top(&self, (y, x): YX) -> Option<YX> {
        (y > 0).then(|| (y - 1, x))
    }

    fn bottom(&self, (y, x): YX) -> Option<YX> {
        (y + 1 < self.height).then(|| (y + 1, x))
    }

    /// Create an iterator over the (y, x) coordinates.
    fn coordinates(&self) -> impl Iterator<Item = YX> + '_ {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| (y, x)))
    }
}

impl Grid<u8> {
    /// Increment the cell.
    /// Then if it flashes (== 10), push it to the stack.
    fn inc_cell(coord: YX, buf: &mut Grid<u8>, stack: &mut Vec<YX>) {
        let cell = &mut buf[coord];
        *cell += 1;
        if *cell == 10 {
            stack.push(coord);
        }
    }

    /// Run a tick of the dumbo octopus simulation,
    /// returning the number of flashes during that tick.
    /// The buffer must have the same size as this grid.
    fn run_tick(&mut self, buf: &mut Self) -> u64 {
        assert_eq!(self.width, buf.width);
        assert_eq!(self.height, buf.height);

        let mut stack = Vec::with_capacity(8);

        // Copy and increment all by 1
        buf.data.copy_from_slice(&self.data);
        self.coordinates()
            .for_each(|p| Self::inc_cell(p, buf, &mut stack));

        let mut flashes = 0;
        while let Some(coord) = stack.pop() {
            flashes += 1;

            // Top
            if let Some(top) = self.top(coord) {
                if let Some(topleft) = self.left(top) {
                    Self::inc_cell(topleft, buf, &mut stack);
                }
                Self::inc_cell(top, buf, &mut stack);
                if let Some(topright) = self.right(top) {
                    Self::inc_cell(topright, buf, &mut stack);
                }
            }

            // Left/Right
            if let Some(left) = self.left(coord) {
                Self::inc_cell(left, buf, &mut stack);
            }
            if let Some(right) = self.right(coord) {
                Self::inc_cell(right, buf, &mut stack);
            }

            // Bottom
            if let Some(bottom) = self.bottom(coord) {
                if let Some(bottomleft) = self.left(bottom) {
                    Self::inc_cell(bottomleft, buf, &mut stack);
                }
                Self::inc_cell(bottom, buf, &mut stack);
                if let Some(bottomright) = self.right(bottom) {
                    Self::inc_cell(bottomright, buf, &mut stack);
                }
            }
        }

        // Copy back buf into self, and make flashed cells go back to 0
        self.data.copy_from_slice(&buf.data);
        self.data
            .iter_mut()
            .filter(|c| **c >= 10)
            .for_each(|c| *c = 0);

        flashes
    }
}

impl<T> Index<YX> for Grid<T> {
    type Output = T;

    fn index(&self, idx: YX) -> &Self::Output {
        self.get(idx).unwrap()
    }
}

impl<T> IndexMut<YX> for Grid<T> {
    fn index_mut(&mut self, idx: YX) -> &mut Self::Output {
        self.get_mut(idx).unwrap()
    }
}

impl Solution for Day11 {
    /// Run 100 steps of the octopuses simulation.
    /// Count the number of flashes.
    fn q1(&self, data: &str) -> String {
        let mut grid = Self::parse_data(data);
        let mut buf = grid.clone();
        let mut flashes = 0;

        for _step in 0..100 {
            flashes += grid.run_tick(&mut buf);
        }

        flashes.to_string()
    }

    /// Find the first step where all octopuses flashes together.
    fn q2(&self, data: &str) -> String {
        let mut grid = Self::parse_data(data);
        let mut buf = grid.clone();

        for step in 1u32.. {
            grid.run_tick(&mut buf);

            if grid.data.iter().all(|c| *c == 0) {
                // All flashes together
                return step.to_string();
            }
        }

        unreachable!()
    }
}

impl Day11 {
    /// Parse the grid of digits
    fn parse_data(data: &str) -> Grid<u8> {
        let mut lines = data.split_terminator('\n').map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Not digit") as u8)
        });

        // Extract the first line to get the width
        let mut data = lines.next().unwrap().collect::<Vec<_>>();
        let width = data.len();

        // Extract the rest of the grid
        data.extend(lines.flatten());
        let height = data.len() / width;

        Grid {
            data,
            width,
            height,
        }
    }
}
