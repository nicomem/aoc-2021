use std::{collections::HashMap, str::FromStr};

use crate::{utils::TryCollectArray, Solution};

pub struct Day8;

/// A 7-segment digit display.
/// The elements correspond to the 'a' -> 'g' signal lines,
/// but they may be shuffled and do not correspond to their naive interpretation.
#[derive(Debug)]
struct SegmentDigit([bool; 7]);

impl SegmentDigit {
    /// Count the number of segments turned on
    fn count(&self) -> usize {
        self.0.iter().filter(|&&b| b).count()
    }

    fn segments(&self) -> impl Iterator<Item = u8> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, &b)| b)
            .map(|(i, _)| i as _)
    }

    /// Try to find to which digit the segment can correspond.
    /// If there is not enough information to decide on a unique
    /// digit, return None.
    fn correspond_digit(&self) -> Option<u8> {
        match self.count() {
            // 2 segments => 1
            2 => Some(1),

            // 3 segments => 7
            3 => Some(7),

            // 4 segments => 4
            4 => Some(4),

            // 7 segments => 8
            7 => Some(8),

            // 5 segments => 2,3,5
            // 6 segments => 0,6,9
            // 0/1/other => No digit
            _ => None,
        }
    }
}

impl FromStr for SegmentDigit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = [false; 7];
        for c in s.chars() {
            let i = c as isize - 'a' as isize;
            if i < 0 || i as usize > segments.len() {
                return Err(());
            }

            segments[i as usize] = true;
        }

        Ok(Self(segments))
    }
}

impl Solution for Day8 {
    /// Count the number of times the digits 1,4,7 or 8 appears.
    fn q1(&self, data: &str) -> String {
        let entries = Self::parse_data(data);
        let queries = entries.map(|(_, query)| query);

        let count_1478 = queries
            .flat_map(|arr| arr.into_iter())
            .flat_map(|seg| seg.correspond_digit())
            .filter(|&n| matches!(n, 1 | 4 | 7 | 8))
            .count();

        count_1478.to_string()
    }

    /// This time, use the patterns to understand how to decode the query.
    /// Then sum all decoded queries together.
    fn q2(&self, data: &str) -> String {
        let entries = Self::parse_data(data);

        let result = entries
            .map(|(patterns, queries)| {
                // Process patterns
                let mapper = Self::process_patterns(patterns);

                // Decode queries
                let [a, b, c, d] = queries.map(|q| mapper.map_segments(&q));

                // Concatenate digits into one number
                1000 * (a as u64) + 100 * (b as u64) + 10 * (c as u64) + (d as u64)
            })
            .sum::<u64>();

        result.to_string()
    }
}

type Patterns = [SegmentDigit; 10];
type Queries = [SegmentDigit; 4];
struct Mapper {
    counts: [u8; 10],
    map: HashMap<[u8; 6], u8>,
}

impl Mapper {
    fn new(counts: [u8; 10]) -> Self {
        Self {
            counts,
            // Sorted segment frequencies for each digit (manual analysis)
            map: HashMap::from([
                ([0, 4, 7, 7, 8, 8], 2),
                ([0, 7, 7, 8, 8, 9], 3),
                ([0, 6, 7, 7, 8, 9], 5),
                ([4, 6, 7, 8, 8, 9], 0),
                ([4, 6, 7, 7, 8, 9], 6),
                ([6, 7, 7, 8, 8, 9], 9),
                // ("89", 1),
                // ("6789", 4),
                // ("889", 7),
                // ("4677889", 8),
            ]),
        }
    }

    fn map_segments(&self, segments: &SegmentDigit) -> u8 {
        // First check if the segment can be decoded using only its count
        if let Some(n) = segments.correspond_digit() {
            n
        } else {
            // If not, create the unique code (sorted array of frequencies)
            let mut arr = [0; 6];
            for (cell, count) in arr
                .iter_mut()
                .zip(segments.segments().map(|i| self.counts[i as usize]))
            {
                *cell = count;
            }
            arr.sort();

            // And ask the map the corresponding number
            self.map[&arr]
        }
    }
}

impl Day8 {
    /// Analyze the 0-9 digits patterns to create a mapper to decode segments
    fn process_patterns(patterns: Patterns) -> Mapper {
        // Since all digits are present exactly once, we can use the segment frequency
        // to create a unique code per digit.
        let mut counts = [0; 10];
        for pat in patterns {
            for i in pat.0.iter().enumerate().filter(|(_, &b)| b).map(|(i, _)| i) {
                counts[i] += 1;
            }
        }

        Mapper::new(counts)
    }

    /// Parse the 10 patterns and 4 query 7-segment digits in each line.
    fn parse_data(data: &str) -> impl Iterator<Item = (Patterns, Queries)> + '_ {
        data.split_terminator('\n').map(|line| {
            let (left, right) = line.split_once(" | ").unwrap();

            let patterns = left
                .split_whitespace()
                .map(|s| s.parse().expect("Could not parse 7-segments digit"))
                .try_collect_array()
                .expect("Could not collect digits into array");

            let query = right
                .split_whitespace()
                .map(|s| s.parse().expect("Could not parse 7-segments digit"))
                .try_collect_array()
                .expect("Could not collect digits into array");

            (patterns, query)
        })
    }
}
