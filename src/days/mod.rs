use seq_macro::seq;

use crate::Solution;

seq!(N in 1..=25 {
    use self::day~N::Day~N;
    mod day~N;
});

pub(crate) const DAYS: &[&dyn Solution] = &[
    &Day1, &Day2, &Day3, &Day4, &Day5, &Day6, &Day7, &Day8, &Day9, &Day10, &Day11, &Day12, &Day13,
    &Day14, &Day15, &Day16, &Day17, &Day18, &Day19, &Day20, &Day21, &Day22, &Day23, &Day24, &Day25,
];
