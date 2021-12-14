use std::collections::HashMap;

use crate::{utils::TryCollectArray, Solution};

pub struct Day14;

type Atom = u8;
type Pair = [Atom; 2];
type AtomCount = u64;

struct Polymerizer {
    /// The polymer components.
    pair_counts: HashMap<Pair, AtomCount>,

    /// Rules to grow the polymer.
    rules: HashMap<Pair, Atom>,

    /// The number of each atom in the polymer.
    atom_counts: HashMap<Atom, AtomCount>,
}

impl Polymerizer {
    fn run_step(&mut self, buf: &mut HashMap<Pair, AtomCount>) {
        // Clear the buffer so that it is empty
        buf.clear();

        // Take the atom count out of the struct to be able to
        // increment it while avoiding mut issues
        let mut atom_counts = std::mem::take(&mut self.atom_counts);

        // Process each pair of atoms in the polymer
        for (pair @ &[a, b], &count) in self.pair_counts.iter() {
            // Find the created atom
            let m = self.rules[pair];

            // Add the 2 created pairs: [a, b] -> [a, m] & [m, b]
            *buf.entry([a, m]).or_insert(0) += count;
            *buf.entry([m, b]).or_insert(0) += count;

            // Add the created atom to the atom counts
            // The atoms a & b have already been counted
            *atom_counts.entry(m).or_insert(0) += count;
        }

        // Swap the buf (new state) with the pair counts (old state)
        std::mem::swap(&mut self.pair_counts, buf);

        // Place back the atom counts into the struct
        self.atom_counts = atom_counts;
    }
}

impl Solution for Day14 {
    /// Apply the rules to the polymer for 10 steps.
    /// Then find the most/least common pairs and count their appearances.
    /// Finally return the difference between the two.
    fn q1(&self, data: &str) -> String {
        let mut polymerizer = Self::parse_data(data);
        let mut buf = HashMap::new();

        for _ in 0..10 {
            polymerizer.run_step(&mut buf);
        }

        let atom_counts = &polymerizer.atom_counts;
        let most_count = atom_counts.iter().max_by_key(|(_, c)| **c).unwrap().1;
        let least_count = atom_counts.iter().min_by_key(|(_, c)| **c).unwrap().1;

        (most_count - least_count).to_string()
    }

    /// Same as q1 but for 40 steps.
    fn q2(&self, data: &str) -> String {
        let mut polymerizer = Self::parse_data(data);
        let mut buf = HashMap::new();

        for _ in 0..40 {
            polymerizer.run_step(&mut buf);
        }

        let atom_counts = &polymerizer.atom_counts;
        let most_count = atom_counts.iter().max_by_key(|(_, c)| **c).unwrap().1;
        let least_count = atom_counts.iter().min_by_key(|(_, c)| **c).unwrap().1;

        (most_count - least_count).to_string()
    }
}

impl Day14 {
    /// Parse the polymer template and all rules.
    fn parse_data(data: &str) -> Polymerizer {
        let mut lines = data.split_terminator('\n');

        let polymer = lines.next().unwrap();
        let _ = lines.next(); // Empty line
        let rules = lines;

        let polymer = polymer.as_bytes();
        assert!(polymer.is_ascii());

        let mut atom_counts = HashMap::new();
        for atom in polymer.iter() {
            *atom_counts.entry(*atom).or_insert(0) += 1;
        }

        let mut pair_counts = HashMap::new();
        for pair in polymer.windows(2) {
            let pair = pair.iter().copied().try_collect_array().unwrap();
            *pair_counts.entry(pair).or_insert(0) += 1;
        }

        let rules = rules
            .map(|line| {
                let (left, right) = line.split_once(" -> ").unwrap();
                let input = left.bytes().try_collect_array().unwrap();
                let [out] = right.bytes().try_collect_array().unwrap();

                (input, out)
            })
            .collect();

        Polymerizer {
            pair_counts,
            rules,
            atom_counts,
        }
    }
}
