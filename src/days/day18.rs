use crate::Solution;

pub struct Day18;

impl Solution for Day18 {
    /// TODO
    fn q1(&self, data: &str) -> String {
        let _lines = Self::parse_data(data);
        todo!()
    }

    /// TODO
    fn q2(&self, data: &str) -> String {
        let _lines = Self::parse_data(data);
        todo!()
    }
}

impl Day18 {
    /// TODO
    fn parse_data(data: &str) -> impl Iterator<Item = &str> + '_ {
        data.split_terminator('\n')
    }
}
