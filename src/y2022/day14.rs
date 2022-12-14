use std::collections::HashSet;
use std::hash::Hash;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::Solution;

pub struct Day14;

impl Solution for Day14 {
    fn q1(&self, data: &str) -> String {
        let mut map = Map::new(parse1(data).collect(), false);
        let before_blocked = map.blocked.len();
        while map.fall_sand().is_some() {
            continue;
        }
        (map.blocked.len() - before_blocked).to_string()
    }

    fn q2(&self, data: &str) -> String {
        let mut map = Map::new(parse1(data).collect(), true);
        let before_blocked = map.blocked.len();
        while let Some(p) = map.fall_sand() {
            if p.y == 0 {
                break;
            }
        }
        (map.blocked.len() - before_blocked).to_string()
    }
}

fn parse1(data: &str) -> impl Iterator<Item = Path> + '_ {
    data.lines().flat_map(|s| s.trim().parse())
}

struct Map {
    blocked: HashSet<Position>,
    the_void_y: u16,
    part_two: bool,
    prev_path: Vec<Position>,
}

impl Map {
    fn new(paths: Vec<Path>, part_two: bool) -> Self {
        let the_void_y = paths
            .iter()
            .flat_map(|path| path.segments.iter())
            .map(|seg| match seg {
                Segment::Hor { y, .. } => *y,
                Segment::Ver { y, .. } => *y.end(),
            })
            .max()
            .unwrap()
            + 2;

        let mut blocked = HashSet::new();

        paths
            .into_iter()
            .flat_map(|path| path.segments)
            .for_each(|seg| match seg {
                Segment::Hor { y, x } => x.map(|x| Position { y, x: x as _ }).for_each(|p| {
                    blocked.insert(p);
                }),
                Segment::Ver { x, y } => y.map(|y| Position { y, x: x as _ }).for_each(|p| {
                    blocked.insert(p);
                }),
            });

        Self {
            blocked,
            the_void_y,
            part_two,
            prev_path: vec![],
        }
    }

    fn fall_sand(&mut self) -> Option<Position> {
        let mut pos = self.prev_path.pop().unwrap_or(Position { x: 500, y: 0 });

        while pos.y < self.the_void_y {
            let down = Position {
                x: pos.x,
                y: pos.y + 1,
            };
            if !self.is_blocked(down) {
                pos = down;
                self.prev_path.push(pos);
            } else {
                let down_left = Position {
                    x: pos.x - 1,
                    y: pos.y + 1,
                };
                if !self.is_blocked(down_left) {
                    pos = down_left;
                    self.prev_path.push(pos);
                } else {
                    let down_right = Position {
                        x: pos.x + 1,
                        y: pos.y + 1,
                    };
                    if !self.is_blocked(down_right) {
                        pos = down_right;
                        self.prev_path.push(pos);
                    } else {
                        self.blocked.insert(pos);
                        return Some(pos);
                    }
                }
            }
        }
        None
    }

    fn is_blocked(&self, pos: Position) -> bool {
        (self.part_two && pos.y >= self.the_void_y) || self.blocked.contains(&pos)
    }
}

#[derive(Debug)]
struct Path {
    segments: Vec<Segment>,
}

#[derive(Debug)]
enum Segment {
    Hor { y: u16, x: RangeInclusive<u16> },
    Ver { x: u16, y: RangeInclusive<u16> },
}

impl Segment {
    fn new(p1: Position, p2: Position) -> Option<Self> {
        if p1.x == p2.x {
            let mut ys = [p1.y, p2.y];
            ys.sort();
            Some(Self::Ver {
                x: p1.x as _,
                y: ys[0]..=ys[1],
            })
        } else if p1.y == p2.y {
            let mut xs = [p1.x as _, p2.x as _];
            xs.sort();
            Some(Self::Hor {
                y: p1.y,
                x: xs[0]..=xs[1],
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i16,
    y: u16,
}

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Position> = s.split(" -> ").flat_map(|s| s.parse()).collect();
        let segments = points
            .windows(2)
            .flat_map(|sl| Segment::new(sl[0], sl[1]))
            .collect();
        Ok(Self { segments })
    }
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(',').ok_or(())?;
        Ok(Self {
            x: a.parse().map_err(|_| ())?,
            y: b.parse().map_err(|_| ())?,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Day14;

    #[test]
    fn q1() {
        let day = Day14 {};

        assert_eq!("24", day.q1(DATA1));
    }

    #[test]
    fn q2() {
        let day = Day14 {};

        assert_eq!("93", day.q2(DATA1));
    }

    const DATA1: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
}
