use std::{fmt::Debug, str::FromStr};

use itertools::Itertools;

use crate::Solution;

pub struct Day3;

impl Solution for Day3 {
    fn q1(&self, data: &str) -> String {
        let rucksacks = parse1(data);
        rucksacks
            .map(|sack| sack.unique_item().0 as u64)
            .sum::<u64>()
            .to_string()
    }

    fn q2(&self, data: &str) -> String {
        let rucksacks = parse2(data);
        rucksacks
            .map(|sack| sack.unique_item().0 as u64)
            .sum::<u64>()
            .to_string()
    }
}

fn parse1(s: &str) -> impl Iterator<Item = Rucksack> + '_ {
    s.split('\n').flat_map(|s| s.parse())
}

fn parse2(s: &str) -> impl Iterator<Item = Rucksack2> + '_ {
    s.split('\n')
        .flat_map(|s| s.parse::<Compartment>())
        .batching(|it| Some(Rucksack2(it.next()?, it.next()?, it.next()?)))
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Item(u8);

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 < 27 {
            write!(f, "{}", (b'a' + self.0 - 1) as char)
        } else {
            write!(f, "{}", (b'A' + self.0 - 27) as char)
        }
    }
}

struct Compartment(Vec<Item>);

impl Debug for Compartment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.0 {
            write!(f, "{item:?}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Rucksack(Compartment, Compartment);

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(());
        }
        let (first, second) = s.split_at(s.len() / 2);
        Ok(Rucksack(first.parse()?, second.parse()?))
    }
}

impl FromStr for Compartment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Compartment(s.trim().chars().map(Item::parse).collect()))
    }
}

impl Item {
    fn parse(c: char) -> Item {
        Item(if ('a'..='z').contains(&c) {
            (c as u8) - b'a' + 1
        } else {
            (c as u8) - b'A' + 27
        })
    }

    fn bit_mask(self) -> u64 {
        1 << (self.0 - 1)
    }

    fn from_bit_mask(mask: u64) -> Item {
        Item((mask.ilog2() + 1) as _)
    }
}

impl Compartment {
    fn item_mask(&self) -> u64 {
        self.0
            .iter()
            .map(|item| item.bit_mask())
            .fold(0, |acc, e| acc | e)
    }
}

impl Rucksack {
    fn unique_item(&self) -> Item {
        let mask1 = self.0.item_mask();
        let mask2 = self.1.item_mask();
        Item::from_bit_mask(mask1 & mask2)
    }
}

struct Rucksack2(Compartment, Compartment, Compartment);

impl Rucksack2 {
    fn unique_item(&self) -> Item {
        let mask1 = self.0.item_mask();
        let mask2 = self.1.item_mask();
        let mask3 = self.2.item_mask();
        Item::from_bit_mask(mask1 & mask2 & mask3)
    }
}
