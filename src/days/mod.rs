use crate::Solution;

use self::day1::Day1;
use self::day2::Day2;
use self::day3::Day3;
use self::day4::Day4;
use self::day5::Day5;
use self::day6::Day6;
use self::day7::Day7;
use self::day8::Day8;
use self::day9::Day9;

use self::day10::Day10;
use self::day11::Day11;
use self::day12::Day12;
use self::day13::Day13;
use self::day14::Day14;
use self::day15::Day15;
use self::day16::Day16;
use self::day17::Day17;
use self::day18::Day18;
use self::day19::Day19;

use self::day20::Day20;
use self::day21::Day21;
use self::day22::Day22;
use self::day23::Day23;
use self::day24::Day24;
use self::day25::Day25;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub(crate) const DAYS: &[&dyn Solution] = &[
    &Day1, &Day2, &Day3, &Day4, &Day5, &Day6, &Day7, &Day8, &Day9, &Day10, &Day11, &Day12, &Day13,
    &Day14, &Day15, &Day16, &Day17, &Day18, &Day19, &Day20, &Day21, &Day22, &Day23, &Day24, &Day25,
];
