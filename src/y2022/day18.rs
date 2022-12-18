use std::collections::HashSet;
use std::str::FromStr;

use crate::Solution;

pub struct Day18;

impl Solution for Day18 {
    fn q1(&self, data: &str) -> String {
        let droplets: HashSet<Pos3> = parse1(data).collect();
        droplets
            .iter()
            .map(|pos| {
                [
                    pos.add_x(1),
                    pos.add_x(-1),
                    pos.add_y(1),
                    pos.add_y(-1),
                    pos.add_z(1),
                    pos.add_z(-1),
                ]
                .into_iter()
                .filter(|pos| !droplets.contains(pos))
                .count()
            })
            .sum::<usize>()
            .to_string()
    }

    fn q2(&self, data: &str) -> String {
        let droplets: HashSet<Pos3> = parse1(data).collect();
        count_outside(&droplets).to_string()
    }
}

fn parse1(data: &str) -> impl Iterator<Item = Pos3> + '_ {
    data.lines().flat_map(|s| s.parse())
}

fn count_outside(droplets: &HashSet<Pos3>) -> usize {
    const START_POS: Pos3 = Pos3 {
        x: -1,
        y: -1,
        z: -1,
    };
    const MIN: i8 = -1;
    const MAX: i8 = 20;

    let mut visited = HashSet::new();
    let mut to_visit = vec![START_POS];
    visited.insert(START_POS);

    let mut count = 0;
    let mut visited_here = Vec::with_capacity(6);
    while let Some(pos) = to_visit.pop() {
        let neighs = [
            pos.add_x(1),
            pos.add_x(-1),
            pos.add_y(1),
            pos.add_y(-1),
            pos.add_z(1),
            pos.add_z(-1),
        ]
        .into_iter()
        .filter(|p| p.in_bounds(MIN, MAX))
        .filter(|p| !visited.contains(p));

        for neigh in neighs {
            if droplets.contains(&neigh) {
                count += 1;
            } else {
                visited_here.push(neigh);
                to_visit.push(neigh);
            }
        }

        visited.extend(visited_here.iter());
        visited_here.clear();
    }
    count
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos3 {
    x: i8,
    y: i8,
    z: i8,
}

impl Pos3 {
    const fn add_x(self, dx: i8) -> Self {
        Self {
            x: self.x + dx,
            ..self
        }
    }

    const fn add_y(self, dy: i8) -> Self {
        Self {
            y: self.y + dy,
            ..self
        }
    }

    const fn add_z(self, dz: i8) -> Self {
        Self {
            z: self.z + dz,
            ..self
        }
    }

    const fn in_bounds(self, min: i8, max: i8) -> bool {
        self.x >= min
            && self.x <= max
            && self.y >= min
            && self.y <= max
            && self.z >= min
            && self.z <= max
    }
}

impl FromStr for Pos3 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, s) = s.trim().split_once(',').ok_or(())?;
        let (y, z) = s.split_once(',').ok_or(())?;
        Ok(Self {
            x: x.parse().map_err(|_| ())?,
            y: y.parse().map_err(|_| ())?,
            z: z.parse().map_err(|_| ())?,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Day18;

    #[test]
    fn q1() {
        let day = Day18 {};

        assert_eq!("64", day.q1(DATA1));
    }

    #[test]
    fn q2() {
        let day = Day18 {};

        assert_eq!("58", day.q2(DATA1));
    }

    const DATA1: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
}
