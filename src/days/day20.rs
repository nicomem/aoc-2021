use crate::Solution;

pub struct Day20;

impl Solution for Day20 {
    /// TODO
    fn q1(&self, data: &str) -> String {
        let _lines = Self::parse_data(data);
        String::new()
    }

    /// TODO
    fn q2(&self, data: &str) -> String {
        let _lines = Self::parse_data(data);
        String::new()
    }
}

impl Day20 {
    /// TODO
    fn parse_data(data: &str) -> impl Iterator<Item = &str> + '_ {
        data.split_terminator('\n')
    }
}
