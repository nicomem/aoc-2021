use itertools::Itertools;

use crate::Solution;

pub struct Day1;

impl Solution for Day1 {
    /// Count the number of times a number is greater than the one before
    fn q1(&self, data: &str) -> String {
        let nums = Self::parse_data(data);
        Self::count_increased(nums).to_string()
    }

    /// Same as q1 but on a 3-sliding window
    fn q2(&self, data: &str) -> String {
        const WIN_SIZE: usize = 3;

        let nums: Vec<_> = Self::parse_data(data).collect();
        let win_sums = nums.windows(WIN_SIZE).map(|win| win.iter().sum::<u64>());
        Self::count_increased(win_sums).to_string()
    }
}

impl Day1 {
    /// Read one number per line
    fn parse_data(data: &str) -> impl Iterator<Item = u64> + '_ {
        data.split_terminator('\n')
            .map(|s| s.parse::<u64>().expect("Could not parse number"))
    }

    /// Count the number of times a number is greater than the one before
    fn count_increased(nums: impl Iterator<Item = u64>) -> usize {
        nums.tuple_windows()
            .map(|(a, b)| b > a)
            .filter(|&b| b)
            .count()
    }
}
