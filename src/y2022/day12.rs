use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Index;
use std::str::FromStr;

use itertools::Itertools;

use crate::Solution;

pub struct Day12;

impl Solution for Day12 {
    fn q1(&self, data: &str) -> String {
        let map: Map = data.parse().unwrap();
        map.dijkstra1().to_string()
    }

    fn q2(&self, data: &str) -> String {
        let map: Map = data.parse().unwrap();
        map.dijkstra2().to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct Map {
    map: Vec<u8>,
    width: usize,
    height: usize,
    start: Position,
    end: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeapElt {
    pos: Position,
    cost: u16,
}

impl PartialOrd for HeapElt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapElt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl Map {
    fn dijkstra1(&self) -> u16 {
        let mut visited = vec![false; self.height * self.width];
        let mut heap = BinaryHeap::new();

        heap.push(Reverse(HeapElt {
            pos: self.start,
            cost: 0,
        }));

        while let Some(Reverse(elt)) = heap.pop() {
            if elt.pos == self.end {
                return elt.cost;
            }

            if visited[self.idx(elt.pos)] {
                continue;
            }

            visited[self.idx(elt.pos)] = true;

            [
                self.left(elt.pos),
                self.right(elt.pos),
                self.up(elt.pos),
                self.down(elt.pos),
            ]
            .into_iter()
            .flatten()
            .filter(|pos| !visited[self.idx(*pos)])
            .filter(|pos| self[*pos] <= self[elt.pos] + 1)
            .map(|pos| HeapElt {
                pos,
                cost: elt.cost + 1,
            })
            .for_each(|elt| heap.push(Reverse(elt)));
        }

        panic!("End not found!");
    }

    fn dijkstra2(&self) -> u16 {
        let mut visited = vec![false; self.height * self.width];
        let mut heap = BinaryHeap::new();

        heap.push(Reverse(HeapElt {
            pos: self.end,
            cost: 0,
        }));

        while let Some(Reverse(elt)) = heap.pop() {
            if self[elt.pos] == 0 {
                return elt.cost;
            }

            if visited[self.idx(elt.pos)] {
                continue;
            }

            visited[self.idx(elt.pos)] = true;

            [
                self.left(elt.pos),
                self.right(elt.pos),
                self.up(elt.pos),
                self.down(elt.pos),
            ]
            .into_iter()
            .flatten()
            .filter(|pos| !visited[self.idx(*pos)])
            .filter(|pos| self[*pos] + 1 >= self[elt.pos])
            .map(|pos| HeapElt {
                pos,
                cost: elt.cost + 1,
            })
            .for_each(|elt| heap.push(Reverse(elt)));
        }

        panic!("End not found!");
    }

    fn idx(&self, pos: Position) -> usize {
        pos.x as usize + pos.y as usize * self.width
    }

    fn up(&self, pos: Position) -> Option<Position> {
        if pos.y > 0 {
            Some(Position {
                y: pos.y - 1,
                x: pos.x,
            })
        } else {
            None
        }
    }

    fn down(&self, pos: Position) -> Option<Position> {
        if pos.y < self.height as u8 - 1 {
            Some(Position {
                y: pos.y + 1,
                x: pos.x,
            })
        } else {
            None
        }
    }

    fn left(&self, pos: Position) -> Option<Position> {
        if pos.x > 0 {
            Some(Position {
                y: pos.y,
                x: pos.x - 1,
            })
        } else {
            None
        }
    }

    fn right(&self, pos: Position) -> Option<Position> {
        if pos.x < self.width as u8 - 1 {
            Some(Position {
                y: pos.y,
                x: pos.x + 1,
            })
        } else {
            None
        }
    }
}

impl Index<Position> for Map {
    type Output = u8;

    fn index(&self, index: Position) -> &Self::Output {
        self.map.index(self.idx(index))
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start = s
            .lines()
            .enumerate()
            .filter_map(|(y, line)| {
                line.chars()
                    .find_position(|c| *c == 'S')
                    .map(|(x, _)| Position {
                        y: y as _,
                        x: x as _,
                    })
            })
            .next()
            .ok_or(())?;
        let end = s
            .lines()
            .enumerate()
            .filter_map(|(y, line)| {
                line.chars()
                    .find_position(|c| *c == 'E')
                    .map(|(x, _)| Position {
                        y: y as _,
                        x: x as _,
                    })
            })
            .next()
            .ok_or(())?;

        let width = s.lines().next().ok_or(())?.len();

        let map: Vec<u8> = s
            .chars()
            .filter_map(|c| match c {
                'a'..='z' => Some(c as u8 - b'a'),
                'S' => Some(0),
                'E' => Some(b'z' - b'a'),
                _ => None,
            })
            .collect();

        let height = map.len() / width;
        Ok(Map {
            map,
            width,
            height,
            start,
            end,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Day12;

    #[test]
    fn q1() {
        let day = Day12 {};

        assert_eq!(
            "31",
            day.q1("Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi")
        );
    }

    #[test]
    fn q2() {
        let day = Day12 {};

        assert_eq!(
            "29",
            day.q2("Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi")
        );
    }
}
