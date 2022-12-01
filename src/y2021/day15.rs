use std::{cmp::Ordering, collections::BinaryHeap};

use crate::{
    utils::{CheckedYX, Grid},
    Solution,
};

pub struct Day15;

type Risk = u16;

impl Solution for Day15 {
    /// Find the lowest risk path from top-left to bottom-right.
    /// Return its risk.
    fn q1(&self, data: &str) -> String {
        let grid = Self::parse_data(data);

        let risk = Self::min_risk_path(&grid);
        risk.to_string()
    }

    /// Same as q1 but with a grid 5x larger in both dimensions,
    /// extending the grid by copying it and adding +1 (mod 9) each time
    /// (+2 for the 2nd copied grid, +3 for the 3rd, etc.)
    fn q2(&self, data: &str) -> String {
        let grid = Self::extend_grid(Self::parse_data(data), 5);

        let risk = Self::min_risk_path(&grid);
        risk.to_string()
    }
}

#[derive(PartialEq, Eq)]
struct HeapItem(Risk, CheckedYX);

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse since max-heap and we want lower risks first
        self.0.cmp(&other.0).reverse()
    }
}

impl Day15 {
    /// Parse the grid of digits
    fn parse_data(data: &str) -> Grid<Risk> {
        data.parse().expect("Could not parse grid")
    }

    // Extend the grid `n` times in each dimension by copying it
    /// and adding +1 (mod 9) each time (+2 for the 2nd copied grid, +3 for the 3rd, etc.)
    fn extend_grid(grid: Grid<Risk>, n: usize) -> Grid<Risk> {
        let height = grid.height * n;
        let width = grid.width * n;
        let data = vec![0; height * width];

        let mut new_grid = Grid {
            data,
            width,
            height,
        };

        let ynum_y = (0..n).flat_map(|ynum| (0..grid.height).map(move |y| (ynum, y)));
        for (ynum, y) in ynum_y {
            let xnum_x = (0..n).flat_map(|xnum| (0..grid.width).map(move |x| (xnum, x)));
            for (xnum, x) in xnum_x {
                let pos =
                    CheckedYX::new(&new_grid, (ynum * grid.height + y, xnum * grid.width + x))
                        .unwrap();
                let new_cell = new_grid.get_mut(pos);

                let pos = CheckedYX::new(&grid, (y, x)).unwrap();
                let cell = grid.get(pos);

                *new_cell = (*cell - 1 + xnum as Risk + ynum as Risk) % 9 + 1;
            }
        }

        new_grid
    }

    /// Find the path from top-left to bottom-right that
    /// minimizes the risk and return its total risk.
    fn min_risk_path(grid: &Grid<Risk>) -> Risk {
        let mut risks = grid.clone();
        risks
            .coordinates()
            .for_each(|p| *risks.get_mut(p) = Risk::MAX);

        let topleft = grid.coordinates().next().unwrap();
        let mut heap = BinaryHeap::with_capacity(1024);
        heap.push(HeapItem(0, topleft));

        let bottom_right = grid.coordinates().last().unwrap();

        while let Some(HeapItem(heap_risk, pos)) = heap.pop() {
            if pos == bottom_right {
                return heap_risk;
            }

            let cell_risk = risks.get_mut(pos);
            if *cell_risk <= heap_risk {
                continue;
            }
            *cell_risk = heap_risk;

            let mut add_if_better = |pos: Option<CheckedYX>| {
                if let Some(pos) = pos {
                    let cur_risk = *risks.get(pos);
                    let new_risk = *grid.get(pos) + heap_risk;

                    if new_risk < cur_risk {
                        heap.push(HeapItem(new_risk, pos));
                    }
                }
            };

            add_if_better(grid.bottom(pos));
            add_if_better(grid.right(pos));
            add_if_better(grid.top(pos));
            add_if_better(grid.left(pos));
        }

        unreachable!()
    }
}
