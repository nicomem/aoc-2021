use std::str::FromStr;

use itertools::Itertools;

use crate::Solution;

pub struct Day8;

impl Solution for Day8 {
    fn q1(&self, data: &str) -> String {
        let parcel: Parcel = data.parse().unwrap();
        (0..parcel.height)
            .cartesian_product(0..parcel.width)
            .filter(|&(y, x)| parcel.is_visible(y, x))
            .count()
            .to_string()
    }

    fn q2(&self, data: &str) -> String {
        let parcel: Parcel = data.parse().unwrap();
        (0..parcel.height)
            .cartesian_product(0..parcel.width)
            .map(|(y, x)| parcel.scenic_score(y, x))
            .max()
            .unwrap()
            .to_string()
    }
}

struct Parcel {
    map: Vec<u8>,
    width: usize,
    height: usize,
}

impl FromStr for Parcel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().ok_or(())?.len();

        let map: Vec<u8> = s
            .chars()
            .filter(char::is_ascii_digit)
            .map(|c| c as u8 - b'0')
            .collect();

        let height = map.len() / width;
        Ok(Parcel { map, width, height })
    }
}

impl Parcel {
    fn left(&self, y: usize, x: usize) -> impl Iterator<Item = u8> + '_ {
        let i_start = y * self.width;
        self.map
            .get(i_start..(i_start + x))
            .unwrap()
            .iter()
            .rev()
            .copied()
    }

    fn right(&self, y: usize, x: usize) -> impl Iterator<Item = u8> + '_ {
        let i_start = y * self.width;
        let i_end = i_start + self.width;
        self.map
            .get((i_start + x + 1)..i_end)
            .unwrap()
            .iter()
            .copied()
    }

    fn up(&self, y: usize, x: usize) -> impl Iterator<Item = u8> + '_ {
        (0..y)
            .rev()
            .flat_map(move |i| self.map.get(x + i * self.width))
            .copied()
    }

    fn down(&self, y: usize, x: usize) -> impl Iterator<Item = u8> + '_ {
        ((y + 1)..self.height)
            .flat_map(move |i| self.map.get(x + i * self.width))
            .copied()
    }

    fn is_visible(&self, y: usize, x: usize) -> bool {
        let my_height = *self.map.get(x + y * self.width).unwrap();
        y == 0
            || x == 0
            || y == self.height - 1
            || x == self.width - 1
            || self.left(y, x).all(|h| h < my_height)
            || self.right(y, x).all(|h| h < my_height)
            || self.up(y, x).all(|h| h < my_height)
            || self.down(y, x).all(|h| h < my_height)
    }

    fn scenic_score(&self, y: usize, x: usize) -> usize {
        let my_height = *self.map.get(x + y * self.width).unwrap();
        (self
            .left(y, x)
            .find_position(|&h| h >= my_height)
            .map_or(x, |(n, _)| n + 1))
            * (self
                .right(y, x)
                .find_position(|&h| h >= my_height)
                .map_or(self.width - x - 1, |(n, _)| n + 1))
            * (self
                .up(y, x)
                .find_position(|&h| h >= my_height)
                .map_or(y, |(n, _)| n + 1))
            * (self
                .down(y, x)
                .find_position(|&h| h >= my_height)
                .map_or(self.height - y - 1, |(n, _)| n + 1))
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Day8;

    #[test]
    fn q1() {
        let day = Day8 {};
        assert_eq!(
            "21",
            day.q1("30373
25512
65332
33549
35390")
        );
    }

    #[test]
    fn q2() {
        let day = Day8 {};
        assert_eq!(
            "8",
            day.q2("30373
25512
65332
33549
35390")
        );
    }
}
