use std::{ops::Range, str::FromStr};

use crate::{unwrap_or_continue, utils::TryCollectArray, Solution};

pub struct Day22;

#[derive(Debug, Clone)]
struct Cuboid {
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
}

impl Cuboid {
    /// Compute the intersection of two cuboids.
    fn intersect(&self, other: &Self) -> Option<Self> {
        fn process_axis(ax1: &Range<i32>, ax2: &Range<i32>) -> Range<i32> {
            ax1.start.max(ax2.start)..ax1.end.min(ax2.end)
        }

        let x = process_axis(&self.x, &other.x);
        if x.is_empty() {
            return None;
        }

        let y = process_axis(&self.y, &other.y);
        if y.is_empty() {
            return None;
        }

        let z = process_axis(&self.z, &other.z);
        if z.is_empty() {
            return None;
        }

        Some(Self { x, y, z })
    }

    /// Remove a smaller cuboid from the cuboid, splitting it into multiple smaller parts.
    /// The returned cuboids do not contain the removed cuboid.
    fn remove_cuboid(mut self, cub_to_remove: &Self) -> Vec<Self> {
        let mut res = vec![];

        macro_rules! split_axis {
            ($ax:ident) => {
                // Split left
                if self.$ax.start < cub_to_remove.$ax.start {
                    res.push(Self {
                        $ax: self.$ax.start..cub_to_remove.$ax.start,
                        ..self.clone()
                    });
                }

                // Split right
                if self.$ax.end > cub_to_remove.$ax.end {
                    res.push(Self {
                        $ax: cub_to_remove.$ax.end..self.$ax.end,
                        ..self.clone()
                    })
                }

                self.$ax = cub_to_remove.$ax.clone();
            };
        }

        split_axis!(x);
        split_axis!(y);
        split_axis!(z);

        res
    }

    /// Return the number of cubes inside the cuboid.
    fn size(&self) -> usize {
        self.x.len() * self.y.len() * self.z.len()
    }
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_axis = |s: &str| {
            let (_, s) = s.split_once('=')?;
            let (start, end) = s.split_once("..")?;

            let start = start.parse().ok()?;
            let end = end.parse::<i32>().ok()? + 1;

            Some(start..end)
        };

        let [x, y, z] = s.split(',').try_collect_array().ok_or(())?;

        let x = parse_axis(x).ok_or(())?;
        let y = parse_axis(y).ok_or(())?;
        let z = parse_axis(z).ok_or(())?;

        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Clone)]
struct RebootStep {
    power_on: bool,
    cuboid: Cuboid,
}

impl FromStr for RebootStep {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (power_on, cuboid) = s.split_once(' ').ok_or(())?;

        let power_on = power_on == "on";
        let area = cuboid.parse()?;

        Ok(Self {
            power_on,
            cuboid: area,
        })
    }
}

#[derive(Debug)]
struct World {
    cuboids: Vec<Cuboid>,
}

impl World {
    fn new() -> Self {
        Self { cuboids: vec![] }
    }

    fn apply(&mut self, step: RebootStep, from_idx: usize) {
        // Find the first element that intersects with the cuboid
        let mut intersections = self.cuboids[from_idx..]
            .iter()
            .zip(from_idx..)
            .flat_map(|(cub, idx)| step.cuboid.intersect(cub).map(|int| (idx, int)));

        if let Some((idx, intersection)) = intersections.next() {
            // If intersecting and powering on, simply trim the step cuboid instead of
            // also trimming the intersected cuboid
            if step.power_on {
                let step_cubs = step.cuboid.remove_cuboid(&intersection);
                for cuboid in step_cubs {
                    self.apply(
                        RebootStep {
                            power_on: step.power_on,
                            cuboid,
                        },
                        idx,
                    )
                }
            } else {
                // Pop the intersected cuboid
                let int_cub = self.cuboids.swap_remove(idx);
                let mut int_cubs = int_cub.remove_cuboid(&intersection);

                // Try to re-apply the step on the rest of the cuboids
                self.apply(step.clone(), idx);

                // Re-add the non-intersected parts of the intersected cuboid
                self.cuboids.append(&mut int_cubs);
            }
        } else if step.power_on {
            // If no intersection, apply the step
            self.cuboids.push(step.cuboid);
        }
    }
}

impl Solution for Day22 {
    /// Apply all reboot steps only on cubes between -50 and 50 (on each axis, both values included).
    /// Count the number of cubes powered on.
    fn q1(&self, data: &str) -> String {
        const AX_RANGE: Range<i32> = -50..51;

        let range_to_bounds = |range: Range<i32>| {
            let start = range.start.max(AX_RANGE.start);
            let end = range.end.min(AX_RANGE.end);

            if start > end {
                None
            } else {
                Some(start..end)
            }
        };

        let steps = Self::parse_data(data);
        let mut world = World::new();

        for RebootStep {
            power_on,
            cuboid: area,
        } in steps
        {
            // Restrict the cuboids to the specified range
            let x = unwrap_or_continue!(range_to_bounds(area.x));
            let y = unwrap_or_continue!(range_to_bounds(area.y));
            let z = unwrap_or_continue!(range_to_bounds(area.z));

            let step = RebootStep {
                power_on,
                cuboid: Cuboid { x, y, z },
            };
            world.apply(step, 0);
        }

        world
            .cuboids
            .iter()
            .map(Cuboid::size)
            .sum::<usize>()
            .to_string()
    }

    /// Same as q1 but without the range restriction
    fn q2(&self, data: &str) -> String {
        let steps = Self::parse_data(data);
        let mut world = World::new();

        for step in steps {
            world.apply(step, 0);
        }

        world
            .cuboids
            .iter()
            .map(Cuboid::size)
            .sum::<usize>()
            .to_string()
    }
}

impl Day22 {
    /// Parse the reboot steps
    fn parse_data(data: &str) -> impl Iterator<Item = RebootStep> + '_ {
        data.lines()
            .map(|line| line.parse().expect("Could not parse input line"))
    }
}
