use crate::Solution;

pub struct Day7;

type Position = u16;

impl Solution for Day7 {
    /// Move all crabs to the same position so that it minimizes the total fuel spent.
    fn q1(&self, data: &str) -> String {
        let crabs = Self::parse_data(data).collect::<Vec<_>>();

        let min = *crabs.iter().min().unwrap();
        let max = *crabs.iter().max().unwrap();

        // The brute force method is simple and "fast-enough" for the input:
        // compute all fuel consumptions and take the minimum.
        //
        // However we can do better: one can find that the fuel consumption is sorted descendingly
        // up to the optimal, then sorted ascendingly (proof left to the reader as an exercise).
        //
        // So, we can design a "binary search-ish" function to find the optimal while avoiding
        // computing many fuel consumptions.
        let values = (min..max).collect::<Vec<_>>();
        let (_ibest, best_pos) = Self::desc_asc_min_by_key(&values, |p| {
            Self::fuel_consumption(crabs.iter().cloned(), *p)
        });

        best_pos.to_string()
    }

    /// Same as q1 but fuel consumption per distance increases with distance.
    fn q2(&self, data: &str) -> String {
        let crabs = Self::parse_data(data).collect::<Vec<_>>();

        let min = *crabs.iter().min().unwrap();
        let max = *crabs.iter().max().unwrap();

        // Here, the new fuel consumption method does not impact the order of the values
        // since the function "sum of integers up to N" is stricly ascending.
        let values = (min..max).collect::<Vec<_>>();
        let (_ibest, best_pos) = Self::desc_asc_min_by_key(&values, |p| {
            Self::fuel_consumption2(crabs.iter().cloned(), *p)
        });

        best_pos.to_string()
    }
}

impl Day7 {
    /// Find the element that gives the minimum in the descending then ascending array mapped by the given function.
    /// e.g. f([1..10]) = [5,3,2, 1, 3,4,5,6,8] -> 4
    fn desc_asc_min_by_key<T, U, F>(values: &[T], mut f: F) -> (&T, U)
    where
        U: Ord + std::fmt::Display,
        F: FnMut(&T) -> U,
    {
        // Do a kind of binary search but by subdivising the slice in 4 thirds.
        // By doing so, we can each time remove 1 or 2 thirds of the data.
        // And so like binary search, we avoid computing many values.
        let mut ileft = 0;
        let mut left = f(values.first().unwrap());
        let mut iright = values.len();
        let mut right = f(values.last().unwrap());

        while ileft < iright {
            let size = iright - ileft;

            let iqleft = ileft + (size / 4).max(1);
            let qleft = f(&values[iqleft]);
            let iqright = iright - (size / 4).max(1);
            let qright = f(&values[iqright - 1]);

            if qleft <= qright {
                iright = iqright;
                right = qright;

                if left <= qleft {
                    iright = iqleft;
                    right = qleft;
                }
            } else {
                ileft = iqleft;
                left = qleft;

                if right <= qright {
                    ileft = iqright;
                    left = qright;
                }
            }
        }

        // May be the next one
        if ileft + 1 < values.len() {
            right = f(&values[ileft + 1]);
            if right < left {
                ileft += 1;
                left = right;
            }
        }

        (&values[ileft], left)
    }

    /// Compute the fuel to spend to move all crabs to the given position
    fn fuel_consumption(crabs: impl Iterator<Item = Position>, move_to: Position) -> u64 {
        crabs
            .map(|p| (move_to as i64 - p as i64).unsigned_abs())
            .sum()
    }

    /// Compute the fuel to spend to move all crabs to the given position
    fn fuel_consumption2(crabs: impl Iterator<Item = Position>, move_to: Position) -> u64 {
        crabs
            .map(|p| Self::sum_integers((move_to as i64 - p as i64).unsigned_abs()))
            .sum()
    }

    /// Compute the sum of integers from 1 to N
    fn sum_integers(n: u64) -> u64 {
        n * (n + 1) / 2
    }

    /// Parse all crab positions, separated by commas
    fn parse_data(data: &str) -> impl Iterator<Item = Position> + '_ {
        data.trim_end().split(',').map(|s| s.parse().unwrap())
    }
}
