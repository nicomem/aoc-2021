use std::{cmp::Ordering, str::FromStr};

use crate::Solution;

pub struct Day2;

impl Solution for Day2 {
    fn q1(&self, data: &str) -> String {
        let rounds = parse1(data);
        rounds
            .map(|round| round.score() as u64)
            .sum::<u64>()
            .to_string()
    }

    fn q2(&self, data: &str) -> String {
        let rounds = parse2(data);
        rounds
            .map(|round| round.score() as u64)
            .sum::<u64>()
            .to_string()
    }
}

fn parse1(data: &str) -> impl Iterator<Item = Round> + '_ {
    data.split('\n').flat_map(|line| line.parse().ok())
}

fn parse2(data: &str) -> impl Iterator<Item = Round2> + '_ {
    data.split('\n').flat_map(|line| line.parse().ok())
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn score(self) -> u8 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn get_better(self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn get_worse(self) -> Choice {
        self.get_better().get_better()
    }

    fn cmp(self, other: Choice) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if self.get_better() == other {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

#[derive(Clone, Copy)]
struct Round {
    opp: Choice,
    my: Choice,
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let opp = s.chars().next().ok_or(())?;
        let my = s.chars().next_back().ok_or(())?;

        let opp = match opp {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            _ => return Err(()),
        };
        let my = match my {
            'X' => Choice::Rock,
            'Y' => Choice::Paper,
            'Z' => Choice::Scissors,
            _ => return Err(()),
        };

        Ok(Self { opp, my })
    }
}

impl Round {
    fn score(self) -> u8 {
        let diff = match self.my.cmp(self.opp) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };
        diff + self.my.score()
    }
}

#[derive(Clone, Copy)]
struct Round2 {
    opp: Choice,
    expected: Ordering,
}

impl FromStr for Round2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let opp = s.chars().next().ok_or(())?;
        let my = s.chars().next_back().ok_or(())?;

        let opp = match opp {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            _ => return Err(()),
        };
        let expected = match my {
            'X' => Ordering::Less,
            'Y' => Ordering::Equal,
            'Z' => Ordering::Greater,
            _ => return Err(()),
        };

        Ok(Self { opp, expected })
    }
}

impl Round2 {
    fn score(self) -> u8 {
        let my = self.what_is_mine();

        let diff = match my.cmp(self.opp) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };
        diff + my.score()
    }

    fn what_is_mine(self) -> Choice {
        match self.expected {
            Ordering::Less => self.opp.get_worse(),
            Ordering::Equal => self.opp,
            Ordering::Greater => self.opp.get_better(),
        }
    }
}
