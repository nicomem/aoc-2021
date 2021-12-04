use itertools::Itertools;

use crate::{utils::TryCollectArray, Solution};

pub struct Day4;

#[derive(Debug)]
struct Grid {
    numbers: [[u8; 5]; 5],
    drawn: [[bool; 5]; 5],
}

impl Grid {
    /// Register a new drawn number into the grid.
    /// Return whether this has resulted in a winning grid.
    fn draw(&mut self, number: u8) -> bool {
        if let Some((i, _)) = self
            .numbers
            .iter()
            .flatten()
            .find_position(|&&n| n == number)
        {
            let y = (i / 5) as usize;
            let x = (i % 5) as usize;

            self.drawn[y][x] = true;
            self.check_line(y) || self.check_col(x)
        } else {
            false
        }
    }

    /// Check whether the line is complete.
    fn check_line(&self, y: usize) -> bool {
        (0..5).all(|x| self.drawn[y][x])
    }

    /// Check whether the column is complete.
    fn check_col(&self, x: usize) -> bool {
        (0..5).all(|y| self.drawn[y][x])
    }

    /// Compute the sum of all not drawn numbers.
    fn sum_unmarked(&self) -> u64 {
        self.numbers
            .iter()
            .flatten()
            .zip(self.drawn.iter().flatten())
            .filter(|(_, &d)| !d)
            .map(|(&n, _)| n as u64)
            .sum()
    }
}

impl Solution for Day4 {
    /// Run the bingo.
    /// Once a grid has won, compute the sum of all its not drawn numbers,
    /// and multiply it with the last drawn number.
    fn q1(&self, data: &str) -> String {
        let (draws, grids) = Self::parse_data(data);
        let mut grids = grids.collect::<Vec<_>>();

        for draw in draws {
            for grid in &mut grids {
                if grid.draw(draw) {
                    // Winning grid
                    let res = grid.sum_unmarked() * draw as u64;
                    return res.to_string();
                }
            }
        }

        unreachable!("No winning grid found")
    }

    /// Same as q1 but with the last winning grid.
    fn q2(&self, data: &str) -> String {
        let (draws, grids) = Self::parse_data(data);
        let mut grids = grids.collect::<Vec<_>>();
        let mut won = vec![false; grids.len()];
        let mut count_not_won = grids.len();

        for draw in draws {
            for (grid, w) in grids.iter_mut().zip(&mut won) {
                if !*w && grid.draw(draw) {
                    // Winning grid
                    *w = true;
                    count_not_won -= 1;

                    if count_not_won == 0 {
                        // Last winning grid => compute score
                        let res = grid.sum_unmarked() * draw as u64;
                        return res.to_string();
                    }
                }
            }
        }

        unreachable!("No last winning grid found")
    }
}

impl Day4 {
    /// Parse the line of drawn numbers
    fn parse_draws(line: &str) -> impl Iterator<Item = u8> + '_ {
        line.split(',')
            .map(|s| s.parse().expect("Could not parse drawn number"))
    }

    /// Parse one grid
    fn parse_grid(data: &str) -> Grid {
        let grid_data = data
            .split_terminator('\n')
            .map(|s| {
                s.split_whitespace()
                    .map(|s| s.parse::<u8>().expect("Could not parse grid cells"))
                    .try_collect_array::<5>()
                    .expect("Could not collect grid line")
            })
            .try_collect_array::<5>()
            .expect("Could not collect grid");

        Grid {
            numbers: grid_data,
            drawn: [[false; 5]; 5],
        }
    }

    /// Parse the input file
    fn parse_data(
        data: &str,
    ) -> (
        impl Iterator<Item = u8> + '_,
        impl Iterator<Item = Grid> + '_,
    ) {
        let mut paragraphs = data.split("\n\n");

        let drawn = Self::parse_draws(paragraphs.next().expect("No drawn line"));
        let grids = paragraphs.map(Self::parse_grid);

        (drawn, grids)
    }
}
