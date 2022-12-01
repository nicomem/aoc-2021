use crate::utils::TryCollectArray;
use crate::Solution;

pub struct Day3;

impl Solution for Day3 {
    /// For each binary digit (0 or 1), find the most and least common value.
    /// These forms two binary numbers: the gamma rate (most common) and epsilon rate (least common).
    ///
    /// Finally multiply both numbers together to get the power consumption.
    fn q1(&self, data: &str) -> String {
        let bin_numbers = Self::parse_data(data).collect::<Vec<_>>();

        let most = (0..Self::N_DIGITS)
            .map(|i| Self::most_common_bool(bin_numbers.iter().map(|arr| arr[i])))
            .try_collect_array()
            .unwrap();
        let least = most.map(|e| !e);

        let gamma_rate = Self::binary_to_integer(most);
        let epsilon_rate = Self::binary_to_integer(least);

        let power_consumption = (gamma_rate as u64) * (epsilon_rate as u64);
        power_consumption.to_string()
    }

    /// For each binary digit, find the most (resp. least) common value and keep only the elements with it.
    /// Then repeat with the next digit until only one element remains.
    /// The element with the most (resp. least) common values is the oxygen (resp. co2) rating.
    ///
    /// Finally, multiply both numbers together to get the life support rating.
    fn q2(&self, data: &str) -> String {
        let bin_numbers = Self::parse_data(data).collect::<Vec<_>>();

        let oxygen_rating = Self::find_most_matching(bin_numbers.clone(), false);
        let co2_rating = Self::find_most_matching(bin_numbers, true);

        let oxygen_rating = Self::binary_to_integer(oxygen_rating);
        let co2_rating = Self::binary_to_integer(co2_rating);

        let life_support = (oxygen_rating as u64) * (co2_rating as u64);
        life_support.to_string()
    }
}

impl Day3 {
    /// Number of binary digits for each number in the input data
    const N_DIGITS: usize = 12;

    /// Read each binary number (one per line) into a bool array
    fn parse_data(data: &str) -> impl Iterator<Item = [bool; Self::N_DIGITS]> + '_ {
        data.split_terminator('\n').map(|s| {
            s.chars()
                .map(|c| match c {
                    '0' => false,
                    '1' => true,
                    _ => panic!("Expected binary number, found '{}'", c),
                })
                .try_collect_array()
                .expect("Could not collect binary number into bool array")
        })
    }

    /// Convert a binary representation to an unsigned integer
    fn binary_to_integer(bin: [bool; Self::N_DIGITS]) -> u16 {
        bin.iter()
            .rev()
            .enumerate()
            .filter(|(_, &e)| e)
            .map(|(i, _)| 1u16 << i)
            .sum()
    }

    /// Find the most common value of a boolean iterator
    fn most_common_bool(values: impl Iterator<Item = bool>) -> bool {
        let (count, len) = values.fold((0usize, 0usize), |(count, len), e| {
            (count + e as usize, len + 1)
        });

        2 * count >= len
    }

    /// For each binary digit:
    /// - Find the most common value
    /// - Keep only those which have this value
    /// - Repeat until a unique element remains and return it
    ///
    /// If `inverse` is true, do the same but keeping elements with the least common value.
    fn find_most_matching(
        mut values: Vec<[bool; Self::N_DIGITS]>,
        inverse: bool,
    ) -> [bool; Self::N_DIGITS] {
        for i in 0..Self::N_DIGITS {
            let most = inverse ^ Self::most_common_bool(values.iter().map(|arr| arr[i]));

            values.retain(|arr| arr[i] == most);
            if values.len() == 1 {
                return values[0];
            }
        }

        unreachable!();
    }
}
