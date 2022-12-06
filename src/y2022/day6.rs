use itertools::Itertools;

use crate::Solution;

pub struct Day6;

impl Solution for Day6 {
    fn q1(&self, data: &str) -> String {
        find_start_packets(data.as_bytes())
            .next()
            .unwrap()
            .0
            .to_string()
    }

    fn q2(&self, data: &str) -> String {
        find_start_messages(data.as_bytes())
            .next()
            .unwrap()
            .0
            .to_string()
    }
}

fn find_start_packets(data: &[u8]) -> impl Iterator<Item = (usize, &[u8])> + '_ {
    const WIN_SIZE: usize = 4;
    data.windows(WIN_SIZE)
        .enumerate()
        .map(|(i, win)| (i + WIN_SIZE, win))
        .filter(|(_, win)| win.iter().all_unique())
}

fn find_start_messages(data: &[u8]) -> impl Iterator<Item = (usize, &[u8])> + '_ {
    const WIN_SIZE: usize = 14;
    data.windows(WIN_SIZE)
        .enumerate()
        .map(|(i, win)| (i + WIN_SIZE, win))
        .filter(|(_, win)| win.iter().all_unique())
}
