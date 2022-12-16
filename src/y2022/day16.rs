use itertools::Itertools;
use rayon::prelude::*;

use crate::Solution;

pub struct Day16;

impl Solution for Day16 {
    fn q1(&self, data: &str) -> String {
        const START_VALVE: &str = "AA";
        const REM_TIME: u8 = 30;

        let map: valve::Map = data.parse().unwrap();
        let fast_paths = FastPaths::from(&map, map.idx_of_name(START_VALVE).unwrap());

        fast_paths.max_pressure1(REM_TIME).to_string()
    }

    fn q2(&self, data: &str) -> String {
        const START_VALVE: &str = "AA";
        const REM_TIME: u8 = 26;

        let map: valve::Map = data.parse().unwrap();
        let fast_paths = FastPaths::from(&map, map.idx_of_name(START_VALVE).unwrap());

        fast_paths.max_pressure2(REM_TIME).to_string()
    }
}

struct FastPaths {
    time_from_start_to_valves: Vec<u8>,
    flow_rates: Vec<u8>,
    time_between_valves: Vec<Vec<u8>>,
}

impl FastPaths {
    fn from(map: &valve::Map, start_idx: valve::Idx) -> Self {
        let indexes_with_flow = map
            .indexes()
            .filter(|i| map.get_flow_rate(*i) > 0)
            .collect_vec();

        let flow_rates = indexes_with_flow
            .iter()
            .map(|i| map.get_flow_rate(*i))
            .collect_vec();

        let time_from_start_to_valves = indexes_with_flow
            .iter()
            .map(|i| map.fastest_path_time(start_idx, *i))
            .collect_vec();

        let time_between_valves = indexes_with_flow
            .iter()
            .map(|i1| {
                indexes_with_flow
                    .iter()
                    .map(|i2| map.fastest_path_time(*i1, *i2))
                    .collect_vec()
            })
            .collect_vec();

        Self {
            time_from_start_to_valves,
            flow_rates,
            time_between_valves,
        }
    }

    fn max_pressure1(&self, rem_time: u8) -> u64 {
        let mut visited = vec![false; self.time_from_start_to_valves.len()];
        self.time_from_start_to_valves
            .iter()
            .enumerate()
            .map(|(i, t1)| {
                visited[i] = true;
                let r = self.max_pressure12(i, rem_time - *t1 - 1, &mut visited);
                visited[i] = false;
                r
            })
            .max()
            .unwrap()
    }

    fn max_pressure12(&self, i: usize, rem_time: u8, visited: &mut [bool]) -> u64 {
        self.time_between_valves[i]
            .iter()
            .enumerate()
            .filter(|(_, t)| rem_time > *t + 1)
            .map(|(i, t)| {
                if !visited[i] {
                    visited[i] = true;
                    let r = self.max_pressure12(i, rem_time - *t - 1, visited);
                    visited[i] = false;
                    r
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0)
            + self.flow_rates[i] as u64 * rem_time as u64
    }

    fn max_pressure2(&self, rem_time: u8) -> u64 {
        self.time_from_start_to_valves
            .iter()
            .enumerate()
            .cartesian_product(self.time_from_start_to_valves.iter().enumerate())
            .filter(|((i1, _), (i2, _))| i1 != i2)
            .par_bridge()
            .map(|((i1, t1), (i2, t2))| {
                let mut visited = vec![false; self.time_from_start_to_valves.len()];
                visited[i1] = true;
                visited[i2] = true;
                let rem_time1 = rem_time - *t1 - 1;
                let rem_time2 = rem_time - *t2 - 1;
                let r = self.max_pressure22(i1, i2, rem_time1, rem_time2, &mut visited);
                visited[i1] = false;
                visited[i2] = false;
                r + (self.flow_rates[i1] as u64 * rem_time1 as u64)
                    + (self.flow_rates[i2] as u64 * rem_time2 as u64)
            })
            .max()
            .unwrap()
    }

    fn max_pressure22(
        &self,
        i1: usize,
        i2: usize,
        rem_time1: u8,
        rem_time2: u8,
        visited: &mut [bool],
    ) -> u64 {
        let r1 = self.time_between_valves[i1]
            .iter()
            .enumerate()
            .filter(|(_, t1)| rem_time1 > *t1 + 1)
            .map(|(i1, t1)| {
                if !visited[i1] {
                    visited[i1] = true;
                    let r = self.max_pressure22(i1, i2, rem_time1 - *t1 - 1, rem_time2, visited);
                    visited[i1] = false;
                    r + (self.flow_rates[i1] as u64 * (rem_time1 - *t1 - 1) as u64)
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0);

        let r2 = self.time_between_valves[i2]
            .iter()
            .enumerate()
            .filter(|(_, t2)| rem_time2 > *t2 + 1)
            .map(|(i2, t2)| {
                if !visited[i2] {
                    visited[i2] = true;
                    let r = self.max_pressure22(i1, i2, rem_time1, rem_time2 - *t2 - 1, visited);
                    visited[i2] = false;
                    r + (self.flow_rates[i2] as u64 * (rem_time2 - *t2 - 1) as u64)
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0);

        r1.max(r2)
    }
}

mod valve {
    use std::collections::VecDeque;
    use std::fmt::Debug;
    use std::fmt::Display;
    use std::str::FromStr;

    use itertools::Itertools;
    use regex::Regex;

    #[derive(Debug)]
    pub(super) struct Map {
        names: Vec<String>,
        flow_rates: Vec<u8>,
        tunnels: Vec<Vec<Idx>>,
    }

    impl Map {
        fn add_valve(&mut self, name: String, flow_rate: u8) -> Idx {
            self.names.push(name);
            self.flow_rates.push(flow_rate);

            Idx((self.names.len() - 1) as _)
        }

        pub fn len(&self) -> usize {
            self.names.len()
        }

        pub fn indexes(&self) -> impl Iterator<Item = Idx> {
            (0..self.len()).map(|i| Idx(i as _))
        }

        pub fn idx_of_name(&self, name: &str) -> Option<Idx> {
            self.names
                .iter()
                .position(|s| s == name)
                .map(|i| Idx(i as _))
        }

        pub fn get_name(&self, idx: Idx) -> &str {
            unsafe { self.names.get_unchecked(idx.0 as usize) }
        }

        pub fn get_flow_rate(&self, idx: Idx) -> u8 {
            unsafe { *self.flow_rates.get_unchecked(idx.0 as usize) }
        }

        pub fn get_tunnels(&self, idx: Idx) -> &[Idx] {
            unsafe { self.tunnels.get_unchecked(idx.0 as usize) }
        }

        pub fn fastest_path_time(&self, from: Idx, to: Idx) -> u8 {
            let mut visited = vec![false; self.len()];
            let mut to_visit = VecDeque::new();
            to_visit.push_back((0, from));
            visited[from.get() as usize] = true;

            while let Some((time, idx)) = to_visit.pop_front() {
                if idx == to {
                    return time;
                }

                for neigh in self.get_tunnels(idx) {
                    if !visited[neigh.get() as usize] {
                        visited[neigh.get() as usize] = true;
                        to_visit.push_back((time + 1, *neigh));
                    }
                }
            }
            panic!("No path between the valves");
        }
    }

    impl Display for Map {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for name in &self.names {
                let idx = self.idx_of_name(name).unwrap();
                write!(
                    f,
                    "Valve {name} has flow rate={}; ",
                    self.get_flow_rate(idx)
                )?;

                let tunnels = self.get_tunnels(idx);
                if tunnels.len() == 1 {
                    writeln!(
                        f,
                        "tunnel leads to valve {}",
                        self.get_name(*tunnels.get(0).unwrap())
                    )?;
                } else {
                    writeln!(
                        f,
                        "tunnels lead to valves {}",
                        tunnels.iter().map(|i| self.get_name(*i)).join(", ")
                    )?;
                }
            }
            Ok(())
        }
    }

    impl FromStr for Map {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut map = Self {
                names: vec![],
                flow_rates: vec![],
                tunnels: vec![],
            };

            let mut tunnels_names = vec![];

            let re = Regex::new(
                r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z]{2}(?:, [A-Z]{2})*)",
            )
            .unwrap();

            for line in s.lines().map(str::trim).filter(|s| !s.is_empty()) {
                let cap = re.captures(line).unwrap();
                let name = cap.get(1).unwrap().as_str();
                let flow_rate = cap.get(2).unwrap().as_str().parse().unwrap();
                let tunnels = cap.get(3).unwrap().as_str().split(", ");

                // Add to the map
                map.add_valve(name.to_owned(), flow_rate);

                tunnels_names.push(tunnels);
            }

            map.tunnels = tunnels_names
                .into_iter()
                .map(|names| names.flat_map(|name| map.idx_of_name(name)).collect_vec())
                .collect_vec();

            Ok(map)
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(super) struct Idx(u8);

    impl Idx {
        pub fn get(self) -> u8 {
            self.0
        }
    }

    impl Debug for Idx {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(&self.0, f)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Day16;

    #[test]
    fn q1() {
        let day = Day16 {};

        assert_eq!("1651", day.q1(DATA1));
    }

    #[test]
    fn q2() {
        let day = Day16 {};

        assert_eq!("1707", day.q2(DATA1));
    }

    const DATA1: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
}
