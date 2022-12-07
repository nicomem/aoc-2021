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

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Day6;

    #[test]
    fn q1() {
        let day = Day6 {};
        assert_eq!("7", day.q1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!("5", day.q1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!("6", day.q1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!("10", day.q1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!("11", day.q1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn q2() {
        let day = Day6 {};
        assert_eq!("19", day.q2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!("23", day.q2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!("23", day.q2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!("29", day.q2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!("26", day.q2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
