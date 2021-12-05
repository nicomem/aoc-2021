use std::str::FromStr;

use crate::Solution;

pub struct Day5;

/// A 2D point
#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        Ok(Self { x, y })
    }
}

/// A 2D line
#[derive(PartialEq, Eq, Hash)]
enum Line {
    /// An horizontal line
    Horizontal { y: u64, x1: u64, x2: u64 },

    /// A vertical line
    Vertical { x: u64, y1: u64, y2: u64 },

    /// Any type of line
    Any { x1: u64, x2: u64, y1: u64, y2: u64 },
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();
        let start: Point = start.parse().unwrap();
        let end: Point = end.parse().unwrap();

        if start.x == end.x {
            let x = start.x;
            let y1 = start.y.min(end.y);
            let y2 = start.y.max(end.y);
            Ok(Line::Vertical { x, y1, y2 })
        } else if start.y == end.y {
            let y = start.y;
            let x1 = start.x.min(end.x);
            let x2 = start.x.max(end.x);
            Ok(Line::Horizontal { y, x1, x2 })
        } else {
            // Make sure start.x <= end.x
            let (start, end) = if start.x <= end.x {
                (start, end)
            } else {
                (end, start)
            };

            Ok(Line::Any {
                x1: start.x,
                x2: end.x,
                y1: start.y,
                y2: end.y,
            })
        }
    }
}

impl Line {
    /// Iterate over the line segment points
    fn walk(&self) -> Box<dyn Iterator<Item = Point>> {
        match *self {
            Line::Horizontal { y, x1, x2 } => Box::new((x1..=x2).map(move |x| Point { x, y })),
            Line::Vertical { x, y1, y2 } => Box::new((y1..=y2).map(move |y| Point { x, y })),
            Line::Any { x1, x2, y1, y2 } => {
                let itx = x1..=x2;
                let ity: Box<dyn Iterator<Item = u64>> = if y1 <= y2 {
                    Box::new(y1..=y2)
                } else {
                    Box::new((y2..=y1).rev())
                };
                Box::new(itx.zip(ity).map(move |(x, y)| Point { x, y }))
            }
        }
    }

    /// Check whether the line is horizontal/vertical
    fn is_horver(&self) -> bool {
        matches!(self, Line::Horizontal { .. } | Line::Vertical { .. })
    }

    /// Check whether the line is diagonal (45 degrees)
    fn is_diag(&self) -> bool {
        if let &Line::Any { x1, x2, y1, y2 } = self {
            (y2 as i64 - y1 as i64).abs() as u64 == x2 - x1
        } else {
            false
        }
    }
}

impl Solution for Day5 {
    /// Count the number of cells where at least 2 horizontal/vertical lines intersects.
    fn q1(&self, data: &str) -> String {
        let lines = Self::parse_data(data);

        // Only consider horizontal/vertical lines
        let lines = lines.filter(Line::is_horver).collect::<Vec<_>>();

        let maxx = *lines
            .iter()
            .map(|l| match l {
                Line::Horizontal { x1, x2, .. } => x1.max(x2),
                Line::Vertical { x, .. } => x,
                Line::Any { x1, x2, .. } => x1.max(x2),
            })
            .max()
            .unwrap() as usize
            + 1;

        let maxy = *lines
            .iter()
            .map(|l| match l {
                Line::Horizontal { y, .. } => y,
                Line::Vertical { y1, y2, .. } => y1.max(y2),
                Line::Any { y1, y2, .. } => y1.max(y2),
            })
            .max()
            .unwrap() as usize
            + 1;

        let mut grid = vec![0u16; maxx * maxy];
        for line in lines {
            for p in line.walk() {
                grid[p.y as usize * maxx + p.x as usize] += 1;
            }
        }

        grid.into_iter().filter(|&c| c >= 2).count().to_string()
    }

    /// Same as q1 but with also diagonal lines
    fn q2(&self, data: &str) -> String {
        let lines = Self::parse_data(data);

        // Only consider horizontal/vertical lines
        let lines = lines
            .filter(|l| l.is_horver() || l.is_diag())
            .collect::<Vec<_>>();

        // Compute the grid size
        let maxx = *lines
            .iter()
            .map(|l| match l {
                Line::Horizontal { x1, x2, .. } => x1.max(x2),
                Line::Vertical { x, .. } => x,
                Line::Any { x1, x2, .. } => x1.max(x2),
            })
            .max()
            .unwrap() as usize
            + 1;

        let maxy = *lines
            .iter()
            .map(|l| match l {
                Line::Horizontal { y, .. } => y,
                Line::Vertical { y1, y2, .. } => y1.max(y2),
                Line::Any { y1, y2, .. } => y1.max(y2),
            })
            .max()
            .unwrap() as usize
            + 1;

        // Fill the grid with the lines
        let mut grid = vec![0u16; maxx * maxy];
        for line in lines {
            for p in line.walk() {
                grid[p.y as usize * maxx + p.x as usize] += 1;
            }
        }

        // Count the number of points where at least 2 lines are
        grid.into_iter().filter(|&c| c >= 2).count().to_string()
    }
}

impl Day5 {
    /// Parse all line segments
    fn parse_data(data: &str) -> impl Iterator<Item = Line> + '_ {
        data.split_terminator('\n').map(|s| s.parse().unwrap())
    }
}
