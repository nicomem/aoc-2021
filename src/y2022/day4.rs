use std::{ops::RangeInclusive, str::FromStr};

use crate::Solution;

pub struct Day4;

impl Solution for Day4 {
    fn q1(&self, data: &str) -> String {
        let pairs = parse1(data);
        pairs
            .filter(|p| {
                // Compute the intersection
                p.intersection()
                    // If the intersection bounds are both start & end bounds of either left or right
                    // Then one of the assignment contains the other
                    .map(|inter| [p.left.size(), p.right.size()].contains(&inter.size()))
                    .unwrap_or(false)
            })
            .count()
            .to_string()
    }

    fn q2(&self, data: &str) -> String {
        let pairs = parse1(data);
        // Even easier this time, we only have to check if there is an intersection
        pairs.flat_map(|p| p.intersection()).count().to_string()
    }
}

fn parse1(s: &str) -> impl Iterator<Item = Pair> + '_ {
    s.split('\n').flat_map(|s| s.parse())
}

#[derive(Debug)]
struct Assignment(RangeInclusive<u8>);

#[derive(Debug)]
struct Pair {
    left: Assignment,
    right: Assignment,
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.trim().split_once(',').ok_or(())?;
        Ok(Pair {
            left: a.parse()?,
            right: b.parse()?,
        })
    }
}

impl FromStr for Assignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.trim().split_once('-').ok_or(())?;
        Ok(Self(
            a.parse().map_err(|_| ())?..=b.parse().map_err(|_| ())?,
        ))
    }
}

impl Assignment {
    fn size(&self) -> u8 {
        self.0.end() - self.0.start() + 1
    }
}

impl Pair {
    fn intersection(&self) -> Option<Assignment> {
        let start = u8::max(*self.left.0.start(), *self.right.0.start());
        let end = u8::min(*self.left.0.end(), *self.right.0.end());

        if start > end {
            None
        } else {
            Some(Assignment(start..=end))
        }
    }
}
