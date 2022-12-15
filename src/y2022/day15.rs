use std::str::FromStr;

use itertools::Itertools;

use crate::Solution;

pub struct Day15;

impl Solution for Day15 {
    fn q1(&self, data: &str) -> String {
        #[cfg(not(test))]
        const Y: i32 = 2000000;
        #[cfg(test)]
        const Y: i32 = 10;

        let sensors = parse1(data);
        let mut ranges = vec![];
        for sensor in sensors {
            if let Some(cur_range) = sensor.vision_on_line(Y) {
                RangeInclX::merge_insert(&mut ranges, cur_range);
                if sensor.beacon.y == Y {
                    RangeInclX::remove_one(&mut ranges, sensor.beacon.x);
                }
            }
        }

        ranges
            .into_iter()
            .map(|r| (r.end - r.start) as u32 + 1)
            .sum::<u32>()
            .to_string()
    }

    fn q2(&self, data: &str) -> String {
        #[cfg(not(test))]
        const MAX: i32 = 4000000;
        #[cfg(test)]
        const MAX: i32 = 20;

        let sensors = parse1(data).collect_vec();
        let mut ranges = vec![];

        // Search from the middle to the end, then from the middle to the start
        let ys = ((MAX / 2)..(MAX + 1)).chain((0..(MAX / 2)).rev());
        for y in ys {
            ranges.clear();

            sensors
                .iter()
                .flat_map(|sensor| sensor.vision_on_line(y))
                .map(|r| RangeInclX {
                    start: i32::max(0, r.start),
                    end: i32::min(MAX, r.end),
                })
                .filter(|r| r.start <= r.end)
                .for_each(|r| RangeInclX::merge_insert(&mut ranges, r));

            if ranges.len() != 1 {
                debug_assert_eq!(2, ranges.len());
                let x = ranges.iter().map(|r| r.end).min().unwrap() as u128 + 1;

                return (x * 4000000 + y as u128).to_string();
            }
        }

        panic!("Nothing found")
    }
}

fn parse1(data: &str) -> impl Iterator<Item = Sensor> + '_ {
    data.lines().flat_map(|s| s.parse())
}

#[derive(Debug)]
struct RangeInclX {
    start: i32,
    end: i32,
}

impl RangeInclX {
    fn contains(&self, x: i32) -> bool {
        self.start <= x && x <= self.end
    }

    /// Try merge both ranges if they intersect, else return None
    fn try_merge(&self, other: &Self) -> Option<Self> {
        if self.end < other.start || other.end < self.start {
            // no intersection
            None
        } else {
            // intersection
            Some(Self {
                start: i32::min(self.start, other.start),
                end: i32::max(self.end, other.end),
            })
        }
    }

    /// Insert a range in the ranges vec, doing the appropriate mergings
    /// so that no range intersects another
    fn merge_insert(ranges: &mut Vec<Self>, me: Self) {
        let mut super_me = me;
        for i in (0..ranges.len()).rev() {
            if let Some(merge) = super_me.try_merge(ranges.get(i).unwrap()) {
                super_me = merge;
                ranges.swap_remove(i);
            }
        }
        ranges.push(super_me);
    }

    /// Remove one position from the ranges. This will do nothing if it not contained in the ranges,
    /// trim one range by 1, or split one range in two
    fn remove_one(ranges: &mut Vec<Self>, x: i32) {
        if let Some((idx, found_range)) = ranges.iter_mut().find_position(|r| r.contains(x)) {
            let x_is_start = found_range.start == x;
            let x_is_end = found_range.end == x;
            match (x_is_start, x_is_end) {
                (true, true) => {
                    ranges.swap_remove(idx);
                }
                (true, false) => found_range.start = x + 1,
                (false, true) => found_range.end = x - 1,
                (false, false) => {
                    let end = found_range.end;
                    found_range.end = x - 1;
                    ranges.push(Self { start: x + 1, end });
                }
            }
        }
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Position,
    beacon: Position,
    dist: u32,
    vision_y_from: i32,
    vision_y_to: i32,
}

impl Sensor {
    /// The sensor vision on the given line. There may be the sensor and/or its beacon inside it
    fn vision_on_line(&self, y: i32) -> Option<RangeInclX> {
        if y < self.vision_y_from || y > self.vision_y_to {
            None
        } else {
            let dist = self.pos.y.abs_diff(y);
            let drest = self.dist as i32 - dist as i32;
            Some(RangeInclX {
                start: self.pos.x - drest,
                end: self.pos.x + drest,
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn dist(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().strip_prefix("Sensor at x=").ok_or(())?;
        let (x, s) = s.split_once(", y=").ok_or(())?;
        let (y, s) = s.split_once(": ").ok_or(())?;
        let s = s.strip_prefix("closest beacon is at x=").ok_or(())?;
        let (bx, s) = s.split_once(", y=").ok_or(())?;
        let by = s;

        let pos = Position {
            x: x.parse().map_err(|_| ())?,
            y: y.parse().map_err(|_| ())?,
        };
        let beacon = Position {
            x: bx.parse().map_err(|_| ())?,
            y: by.parse().map_err(|_| ())?,
        };
        let dist = pos.dist(&beacon);

        Ok(Self {
            pos,
            beacon,
            dist,
            vision_y_from: pos.y - dist as i32,
            vision_y_to: pos.y + dist as i32,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Day15;

    #[test]
    fn q1() {
        let day = Day15 {};

        assert_eq!("26", day.q1(DATA1));
    }

    #[test]
    fn q2() {
        let day = Day15 {};

        assert_eq!("56000011", day.q2(DATA1));
    }

    const DATA1: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3";
}
