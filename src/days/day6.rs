use std::{cmp::Ordering, collections::BinaryHeap};

use itertools::Itertools;

use crate::Solution;

pub struct Day6;

/// A point of time in the simulation
type Tick = u16;

/// A collection of fishes with the same timer
#[derive(Debug, Eq)]
struct Fish {
    /// The number of fishes with the same timer grouped together
    number: u64,

    /// The next time the fishes will procreate
    timer: Tick,
}

impl PartialEq for Fish {
    fn eq(&self, other: &Self) -> bool {
        // Compare fishes by their timers
        self.timer == other.timer
    }
}

impl PartialOrd for Fish {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // The lower the timer is, the higher the priority is
        self.timer.partial_cmp(&other.timer).map(Ordering::reverse)
    }
}

impl Ord for Fish {
    fn cmp(&self, other: &Self) -> Ordering {
        // The lower the timer is, the higher the priority is
        self.timer.cmp(&other.timer).reverse()
    }
}

#[derive(Debug)]
struct Pool {
    /// A heap of fishes where the next ones to procreate will be at the top.
    /// The struct will try to minimize the number of elements contained
    /// in the heap, but there may still be multiple elements with the same timer.
    fishes: BinaryHeap<Fish>,
}

impl Pool {
    const PROCREATION_PERIOD: Tick = 7;
    const MATURITY_PERIOD: Tick = 2;

    /// Create a new pool filled with fishes.
    /// Each fish is described by a timer until its next procreation.
    fn new(fishes: impl Iterator<Item = Tick>) -> Self {
        // Group fishes with same timer together
        let fishes = fishes
            .counts()
            .into_iter()
            .map(|(timer, number)| Fish {
                timer,
                number: number as _,
            })
            .collect::<BinaryHeap<_>>();

        Self { fishes }
    }

    /// Pop the next fish if its timer is less than or equal
    /// to the given timer.
    fn pop_if<P>(&mut self, p: P) -> Option<Fish>
    where
        P: FnOnce(&Fish) -> bool,
    {
        if self.fishes.peek().filter(|f| p(f)).is_some() {
            self.fishes.pop()
        } else {
            None
        }
    }

    /// Run the simulation up to the given timer.
    ///
    /// **Does not run for `timer` ticks!**.
    /// Which means that a second call with the same timer will do nothing.
    fn run_until(&mut self, timer: Tick) {
        // Check if more fishes have procreated
        while let Some(mut fish) = self.pop_if(|f| f.timer < timer) {
            // If there are more fishes with the same timer,
            // merge them together to avoid using too much memory
            while let Some(fish_same_timer) = self.pop_if(|f| f.timer == fish.timer) {
                fish.number += fish_same_timer.number;
            }

            // Procreate
            self.fishes.push(Fish {
                number: fish.number,
                timer: fish.timer + Self::MATURITY_PERIOD + Self::PROCREATION_PERIOD,
            });

            // Put the fishes back in the pool
            self.fishes.push(Fish {
                number: fish.number,
                timer: fish.timer + Self::PROCREATION_PERIOD,
            });
        }
    }

    /// Returns the number of fishes in the pool
    fn len(&self) -> u64 {
        self.fishes.iter().map(|f| f.number).sum()
    }
}

impl Solution for Day6 {
    /// Simulate fish exponential procreation for 80 days.
    /// After, return the number of fishes there are.
    fn q1(&self, data: &str) -> String {
        let fishes = Self::parse_data(data);
        let mut pool = Pool::new(fishes);

        pool.run_until(80);

        pool.len().to_string()
    }

    /// Same as q1 but for 256 days.
    fn q2(&self, data: &str) -> String {
        let fishes = Self::parse_data(data);
        let mut pool = Pool::new(fishes);

        pool.run_until(256);

        pool.len().to_string()
    }
}

impl Day6 {
    /// Parse the fishes described by their timers
    fn parse_data(data: &str) -> impl Iterator<Item = Tick> + '_ {
        data.trim_end().split(',').map(|s| s.parse().unwrap())
    }
}
