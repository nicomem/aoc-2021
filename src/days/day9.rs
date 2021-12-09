use std::ops::{Index, IndexMut};

use crate::Solution;

pub struct Day9;

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

    /// Check that the point at the coordinates is
    /// a low point: lower than the adjacent cells.
    fn is_low_point(&self, pos: YX) -> bool
    where
        T: Ord,
    {
        let left = self.left(pos).map(|p| &self[p]);
        let right = self.right(pos).map(|p| &self[p]);
        let top = self.top(pos).map(|p| &self[p]);
        let bottom = self.bottom(pos).map(|p| &self[p]);

        if let Some(min_neigh) = [left, right, top, bottom].into_iter().flatten().min() {
            &self[pos] < min_neigh
        } else {
            unreachable!()
        }
    }

    /// Create an iterator over the (y, x) coordinates.
    fn coordinates(&self) -> impl Iterator<Item = YX> + '_ {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| (y, x)))
    }

    /// Compute the risk level of the coordinate.
    fn risk_level(&self, pos: YX) -> u64
    where
        T: Copy + Into<u64>,
    {
        self[pos].into() + 1
    }

    /// Fill the basin and count the number of filled values.
    fn flood_fill(pos: YX, visited: &mut Grid<bool>, stack: &mut Vec<YX>) -> u64 {
        stack.clear();
        stack.push(pos);

        let mut count = 0;
        while let Some(pos) = stack.pop() {
            let cell = &mut visited[pos];
            if *cell {
                continue;
            }

            *cell = true;
            count += 1;

            let mut push_if_not_visited = |opt_p| {
                if let Some(p) = opt_p {
                    if !visited[p] {
                        stack.push(p)
                    }
                }
            };

            push_if_not_visited(visited.left(pos));
            push_if_not_visited(visited.right(pos));
            push_if_not_visited(visited.top(pos));
            push_if_not_visited(visited.bottom(pos));
        }

        count
    }

    /// Find the 3 largest basins in the grid and return their sizes.
    /// The returned vector will be sorted ascendingly.
    fn top3_basins(&self, top_limit: T) -> [u64; 3]
    where
        T: PartialEq,
    {
        let mut top4 = [0; 4];
        let mut visited = Grid {
            data: self
                .data
                .iter()
                .map(|n| n == &top_limit)
                .collect::<Vec<_>>(),
            width: self.width,
            height: self.height,
        };
        let mut stack = vec![];

        for pos in self.coordinates() {
            // If already visited, pass
            if visited[pos] {
                continue;
            }

            // Fill the basin and count the number of filled cells
            let count = Self::flood_fill(pos, &mut visited, &mut stack);

            // Replace the 4th largest basin with this one and sort the array.
            // If it is in top3, it will move the new 4th at index 0.
            top4[0] = count;
            top4.sort_unstable();
        }

        // Remove the smallest element of the top4
        let [_, top3 @ ..] = top4;
        top3
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

impl Solution for Day9 {
    /// Find the low points in the grid and sum their
    /// risk level: 1 + their height.
    fn q1(&self, data: &str) -> String {
        let grid = Self::parse_data(data);

        let res = grid
            .coordinates()
            .filter(|&pos| grid.is_low_point(pos))
            .map(|pos| grid.risk_level(pos))
            .sum::<u64>();

        res.to_string()
    }

    /// Find the 3 largest basins and multiply their sizes together.
    fn q2(&self, data: &str) -> String {
        let grid = Self::parse_data(data);

        let [a, b, c] = grid.top3_basins(9);
        (a * b * c).to_string()
    }
}

impl Day9 {
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
