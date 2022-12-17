use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::{self, Display, Write},
    str::FromStr,
};

use crate::Solution;

pub struct Day13;

/// A coordinate along an axis.
type Coord = u16;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
/// A 2D position.
struct XY(Coord, Coord);

impl XY {
    /// Return the position of the point after the given fold.
    /// If the point does not exist after the fold, return None.
    fn folded(self, fold: Fold) -> Option<Self> {
        match fold {
            Fold::X(x) => match self.0.cmp(&x) {
                Ordering::Less => Some(self),
                Ordering::Equal => None,
                Ordering::Greater => Some(Self(2 * x - self.0, self.1)),
            },
            Fold::Y(y) => match self.1.cmp(&y) {
                Ordering::Less => Some(self),
                Ordering::Equal => None,
                Ordering::Greater => Some(Self(self.0, 2 * y - self.1)),
            },
        }
    }
}

impl FromStr for XY {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(())?;

        let x = x.parse().map_err(|_| ())?;
        let y = y.parse().map_err(|_| ())?;

        Ok(Self(x, y))
    }
}

/// A transparent paper sheet, containing a set of distinct points.
struct Paper {
    points: BTreeSet<XY>,
    width: Coord,
    height: Coord,
}

impl Paper {
    /// Fold the paper along an axis and coordinate.
    /// If points are folded to the same position, they will be merged together.
    fn fold(self, fold: Fold) -> Self {
        let points = self
            .points
            .into_iter()
            .flat_map(|p| p.folded(fold))
            .collect();

        let (width, height) = match fold {
            Fold::X(_) => (self.width / 2, self.height),
            Fold::Y(_) => (self.width, self.height / 2),
        };

        Self {
            points,
            width,
            height,
        }
    }

    /// Count the number of distinct points on the paper.
    fn len(&self) -> usize {
        self.points.len()
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let w = self.width as usize;
        let h = self.height as usize;

        for y in 0..h {
            for x in 0..w {
                f.write_char(if self.points.contains(&XY(x as _, y as _)) {
                    '█'
                } else {
                    ' '
                })?
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
/// A folding instruction along an axis at a specific coordinate.
/// Folds will always be from the higher coordinates to the lower ones.
enum Fold {
    X(Coord),
    Y(Coord),
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once("fold along ").ok_or(())?;
        let (axis, coord) = s.split_once('=').ok_or(())?;

        let coord = coord.parse().map_err(|_| ())?;
        Ok(match axis {
            "x" => Fold::X(coord),
            "y" => Fold::Y(coord),
            _ => return Err(()),
        })
    }
}

impl Solution for Day13 {
    /// Apply the first fold instruction.
    /// How many distinct points are visible?
    fn q1(&self, data: &str) -> String {
        let (paper, mut folds) = Self::parse_data(data);

        let first_fold = folds.next().unwrap();
        let paper = paper.fold(first_fold);

        paper.len().to_string()
    }

    /// Apply the first fold instruction.
    /// The code is 8 capital letters.
    fn q2(&self, data: &str) -> String {
        let (mut paper, folds) = Self::parse_data(data);

        for fold in folds {
            paper = paper.fold(fold);
        }

        // The code is some ASCII art, so print it and let the human brain
        // of the person reading this comment read the letters.
        format!("\n{paper}")
    }
}

impl Day13 {
    /// Parse the points and folding instructions
    fn parse_data(data: &str) -> (Paper, impl Iterator<Item = Fold> + '_) {
        let (points, folds) = data.split_once("\n\n").unwrap();

        let points: BTreeSet<XY> = points
            .split_terminator('\n')
            .map(|line| line.parse().expect("Could not parse XY point"))
            .collect();

        let folds = folds
            .split_terminator('\n')
            .map(|line| line.parse().expect("Could not parse fold instruction"));

        let width = points.iter().map(|p| p.0).max().unwrap();
        let height = points.iter().map(|p| p.1).max().unwrap();
        let paper = Paper {
            points,
            width,
            height,
        };
        (paper, folds)
    }
}
