use std::ops::RangeInclusive;

use crate::Solution;

pub struct Day17;

struct Rect {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl Solution for Day17 {
    /// The probe starts at (0,0).
    /// Find the velocity that makes it go the highest before landing
    /// on the target.
    /// Return this highest y value.
    fn q1(&self, data: &str) -> String {
        // The starting x & y coordinates are independant.
        // This means that since we only care about y, we can forget about x.
        // We only need to know that for every correct y, we can find a correct x.

        // A first way of thinking:
        // // Starting with a y velocity Vy, the highest point we get is:
        // // * sum[i:1->Vy]( i )
        // // Which can be simplified to:
        // // * Vy * (Vy+1) / 2
        // // Let's call this highest y point: Hy

        // // Now that we reached Hy, we need to find whether we land on the target.
        // // We need to find whether the following can be found:
        // // * Ty_min <= Hy - sum[i:1->N]( i ) <= Ty_max
        // // For any N integer.
        // // Again simplified to:
        // // * Ty_min <= Hy - N(N+1)/2 <= Ty_max
        // // * Hy - Ty_max <= N(N+1)/2 <= Hy - Ty_min
        // // * 2(Hy - Ty_max) <= N(N+1) <= 2(Hy - Ty_min)
        // // * min_bound <= N(N+1) <= max_bound

        // A simpler way (assuming that the target is below 0):
        // // After going to the highest point, it will do the same movements
        // // as it did going upwards (but now going downwards, movements are mirrored)
        // // This means that it will always go down to 0 at one point.

        // // Which means that the largest next correct step it could make
        // // is the min point of the target.
        // // And to obtain this step, we need to throw it at this step velocity
        // // (since the movements are mirorred)

        let target = Self::parse_data(data);
        let vel_y = -*target.y.start() - 1;
        let hy = vel_y * (vel_y + 1) / 2;
        hy.to_string()
    }

    /// Now find every initial velocities that goes to the target.
    /// And count them.
    fn q2(&self, data: &str) -> String {
        let target = Self::parse_data(data);

        // Too much maths on q1, let's brute force this
        // But use a bit of maths to reduce the searching bounds
        let x_range = {
            let mut x_min = (*target.x.start() as f32).sqrt() as i32;
            if x_min * (x_min + 1) < 2 * *target.x.start() {
                x_min += 1;
            }
            let x_max = *target.x.end();
            x_min..=x_max
        };

        let y_range = {
            let y_min = *target.y.start();
            let y_max = -*target.y.start() - 1;
            y_min..=y_max
        };

        let mut count: u32 = 0;
        for vel_y in y_range {
            for mut vel_x in x_range.clone() {
                let mut vel_y = vel_y;
                let mut x = 0;
                let mut y = 0;
                loop {
                    x += vel_x;
                    y += vel_y;
                    vel_x -= vel_x.signum();
                    vel_y -= 1;
                    if y > *target.y.end() {
                        continue;
                    }

                    if y < *target.y.start() {
                        break;
                    }

                    if target.x.contains(&x) {
                        count += 1;
                        break;
                    }
                }
            }
        }

        count.to_string()
    }
}

impl Day17 {
    /// Parse the single line containing the target area
    fn parse_data(data: &str) -> Rect {
        let (xdata, ydata) = data
            .trim()
            .trim_start_matches("target area: ")
            .split_once(", ")
            .unwrap();

        let xdata = xdata.trim_start_matches("x=");
        let ydata = ydata.trim_start_matches("y=");

        let (xs, xe) = xdata.split_once("..").unwrap();
        let (ys, ye) = ydata.split_once("..").unwrap();

        let xs = xs.parse().unwrap();
        let xe = xe.parse().unwrap();
        let ys = ys.parse().unwrap();
        let ye = ye.parse().unwrap();

        Rect {
            x: xs..=xe,
            y: ys..=ye,
        }
    }
}
