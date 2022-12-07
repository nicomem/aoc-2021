use std::{fmt::Debug, str::FromStr};

use itertools::Itertools;

use crate::{utils::TryCollectArray, Solution};

pub struct Day5;

impl Solution for Day5 {
    fn q1(&self, data: &str) -> String {
        let (mut drawing, moves) = parse1(data);

        for mov in moves {
            drawing.apply1(mov);
        }

        drawing
            .stacks
            .into_iter()
            .map(|stack| *stack.last().unwrap())
            .join("")
    }

    fn q2(&self, data: &str) -> String {
        let (mut drawing, moves) = parse1(data);

        for mov in moves {
            drawing.apply2(mov);
        }

        drawing
            .stacks
            .into_iter()
            .map(|stack| *stack.last().unwrap())
            .join("")
    }
}

fn parse1(data: &str) -> (Drawing, Vec<Move>) {
    let (part1, part2) = data.split_once("\n\n").unwrap();

    (
        part1.parse().expect("Could not parse drawing"),
        part2.split('\n').flat_map(|s| s.parse::<Move>()).collect(),
    )
}

type Crate = char;
type Stack = Vec<Crate>;
#[derive(Debug)]
struct Drawing {
    stacks: Vec<Stack>,
}

impl Drawing {
    fn apply1(&mut self, mov: Move) {
        for _ in 0..mov.quantity {
            let crat = self
                .stacks
                .get_mut(mov.from as usize - 1)
                .unwrap()
                .pop()
                .unwrap();
            self.stacks.get_mut(mov.to as usize - 1).unwrap().push(crat);
        }
    }

    fn apply2(&mut self, mov: Move) {
        let crats: Vec<Crate> = (0..mov.quantity)
            .map(|_| {
                self.stacks
                    .get_mut(mov.from as usize - 1)
                    .unwrap()
                    .pop()
                    .unwrap()
            })
            .collect();
        for crat in crats.into_iter().rev() {
            self.stacks.get_mut(mov.to as usize - 1).unwrap().push(crat);
        }
    }
}

impl FromStr for Drawing {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.split('\n').collect();

        let nb_stacks = lines.last().unwrap().split_whitespace().count();

        let mut stacks: Vec<Stack> = Vec::with_capacity(nb_stacks);
        for i_stack in 0..nb_stacks {
            stacks.push(
                lines
                    .iter()
                    .rev()
                    .skip(1)
                    .flat_map(|line| line.as_bytes().get(1 + (4 * i_stack)))
                    .flat_map(|byte| char::from_u32(*byte as _))
                    .filter(|c| !c.is_whitespace())
                    .collect(),
            );
        }

        Ok(Drawing { stacks })
    }
}

#[derive(Debug)]
struct Move {
    quantity: u8,
    from: u8,
    to: u8,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [quantity, from, to]: [u8; 3] = s
            .split_whitespace()
            .flat_map(|s| s.parse::<u8>().ok())
            .try_collect_array()
            .ok_or(())?;

        Ok(Self { quantity, from, to })
    }
}

#[cfg(test)]
mod test {
    const DATA: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    use crate::Solution;

    use super::Day5;

    #[test]
    fn q1() {
        assert_eq!("CMZ", Day5 {}.q1(DATA));
    }

    #[test]
    fn q2() {
        assert_eq!("MCD", Day5 {}.q2(DATA));
    }
}
