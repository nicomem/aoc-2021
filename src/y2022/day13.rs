use std::cmp::Ordering;
use std::fmt::Display;
use std::iter::Peekable;
use std::vec;

use itertools::Itertools;

use crate::Solution;

pub struct Day13;

impl Solution for Day13 {
    fn q1(&self, data: &str) -> String {
        let packet_pairs = parse1(data);

        packet_pairs
            .enumerate()
            .map(|(i, pair)| (i + 1, pair))
            .filter(|(_, (p1, p2))| p1 < p2)
            .map(|(i, _)| i)
            .sum::<usize>()
            .to_string()
    }

    fn q2(&self, data: &str) -> String {
        let div1 = Packet {
            items: vec![Item::Packet(Box::new(Packet {
                items: vec![Item::Num(2)],
            }))],
        };
        let div2 = Packet {
            items: vec![Item::Packet(Box::new(Packet {
                items: vec![Item::Num(6)],
            }))],
        };

        let sorted_packets: Vec<Packet> = parse2(data)
            .chain([div1.clone(), div2.clone()])
            .sorted()
            .collect();
        let pos_div1 = sorted_packets.iter().position(move |p| div1.eq(p)).unwrap() + 1;
        let pos_div2 = sorted_packets.iter().position(move |p| div2.eq(p)).unwrap() + 1;

        (pos_div1 * pos_div2).to_string()
    }
}

fn parse1(data: &str) -> impl Iterator<Item = (Packet, Packet)> + '_ {
    data.split("\n\n").flat_map(|s| {
        let (p1, p2) = s.split_once('\n')?;
        Some((
            Packet::parse(&mut p1.trim().chars().peekable()),
            Packet::parse(&mut p2.trim().chars().peekable()),
        ))
    })
}

fn parse2(data: &str) -> impl Iterator<Item = Packet> + '_ {
    data.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|s| Packet::parse(&mut s.chars().peekable()))
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    Num(u8),
    Packet(Box<Packet>),
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Num(n) => n.fmt(f),
            Item::Packet(p) => p.fmt(f),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Num(n1), Item::Num(n2)) => n1.cmp(n2),
            (Item::Packet(p1), Item::Packet(p2)) => p1.cmp(p2),
            (Item::Num(n), Item::Packet(p)) => Packet {
                items: vec![Item::Num(*n)],
            }
            .cmp(p),
            (Item::Packet(p), Item::Num(n)) => Packet {
                items: vec![Item::Num(*n)],
            }
            .cmp(p)
            .reverse(),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Packet {
    items: Vec<Item>,
}

impl Packet {
    fn parse(chars: &mut Peekable<impl Iterator<Item = char>>) -> Self {
        let c = chars.next();
        debug_assert_eq!(Some('['), c);

        let mut packet = Self { items: vec![] };
        while let Some(c) = chars.peek() {
            match c {
                ']' => {
                    let c = chars.next();
                    debug_assert_eq!(Some(']'), c);
                    break;
                }
                ',' => {
                    let c = chars.next();
                    debug_assert_eq!(Some(','), c);
                }
                '[' => {
                    packet
                        .items
                        .push(Item::Packet(Box::new(Packet::parse(chars))));
                }
                _ => {
                    let num = chars
                        .peeking_take_while(char::is_ascii_digit)
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    packet.items.push(Item::Num(num));
                }
            }
        }
        packet
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.items
            .iter()
            .zip(&other.items)
            .flat_map(|(i1, i2)| match i1.cmp(i2) {
                Ordering::Equal => None,
                e => Some(e),
            })
            .next()
            .unwrap_or_else(|| self.items.len().cmp(&other.items.len()))
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        if !self.items.is_empty() {
            write!(f, "{}", &self.items[0])?;
            for item in &self.items[1..] {
                write!(f, ",{item}")?;
            }
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Day13;

    #[test]
    fn q1() {
        let day = Day13 {};

        assert_eq!("13", day.q1(DATA1));
    }

    #[test]
    fn q2() {
        let day = Day13 {};

        assert_eq!("140", day.q2(DATA1));
    }

    const DATA1: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
}
