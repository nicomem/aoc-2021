use crate::{
    utils::{CheckedYX, Grid, TryCollectArray},
    Solution,
};

pub struct Day20;

/// The image enhancement algorithm. A mapper from a 9-bit number to an alive/dead state.
struct Algorithm([bool; 512]);

impl Algorithm {
    fn map(&self, bits: [bool; 9]) -> bool {
        let idx: usize = bits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &b)| (b as usize) << i)
            .sum();

        self.0[idx]
    }
}

/// The image, as a set of alive cells (Y,X) coordinates.
struct Image(Grid<bool>);

impl Image {
    /// Count the number of alive cells.
    fn count_alive(&self) -> usize {
        self.0.coordinates().filter(|&yx| *self.0.get(yx)).count()
    }

    /// Run one iteration of the image enhancement algorithm on the image.
    fn run_algo(&mut self, algo: &Algorithm, buf: &mut Vec<bool>) {
        buf.clear();

        // Apply the algorithm to the image and save the result into the buffer
        for yx in self.0.coordinates() {
            let square = self.get_3x3_square(yx);
            let mapped = algo.map(square);
            buf.push(mapped);
        }

        // Swap the image buffer and the updated buffer
        std::mem::swap(&mut self.0.data, buf);
    }

    /// Get the 3x3 square values in (top->bottom, left->right) order.
    /// In case of out-of-bound, use the current cell value.
    fn get_3x3_square(&self, yx: CheckedYX) -> [bool; 9] {
        let cell = *self.0.get(yx);

        // Generate the 3x3 coordinates
        let (y, x) = *yx;
        let (y, x) = (y as isize, x as isize);
        let square_yx = [
            (y - 1, x - 1),
            (y - 1, x),
            (y - 1, x + 1),
            (y, x - 1),
            (y, x),
            (y, x + 1),
            (y + 1, x - 1),
            (y + 1, x),
            (y + 1, x + 1),
        ];

        // Get the coordinates values. If OOB, use the current cell value
        square_yx.map(|yx| CheckedYX::new_signed(&self.0, yx).map_or(cell, |yx| *self.0.get(yx)))
    }
}

impl Solution for Day20 {
    /// There is a starting image and an image enhancement algorithm.
    /// Run the algorithm twice on the image and count the number of alive cells.
    fn q1(&self, data: &str) -> String {
        const STEPS: usize = 2;

        let (algo, mut img) = Self::parse_data(data);

        // Grow the image to be able to correctly handle the algorithm
        let grow_side_by = STEPS;
        img.0 = img.0.resized(
            img.0.height + 2 * grow_side_by,
            img.0.width + 2 * grow_side_by,
            (grow_side_by, grow_side_by),
            false,
        );

        let mut buf = vec![];

        // Run the algo twice
        for _ in 0..STEPS {
            img.run_algo(&algo, &mut buf)
        }

        // Count the number of alive cells
        img.count_alive().to_string()
    }

    /// Do the same but with 50 iterations
    fn q2(&self, data: &str) -> String {
        const STEPS: usize = 50;

        let (algo, mut img) = Self::parse_data(data);

        // Grow the image to be able to correctly handle the algorithm
        // Do this every step to:
        // - Avoid processing too many unnecessary cells
        // - Avoid copying the grid too many times
        //
        // Alternatively, we could modify run_algo to be able to work on a
        // subset of the grid (dist from the center).
        for _ in 0..5 {
            let grow_side_by = STEPS / 5;
            img.0 = img.0.resized(
                img.0.height + 2 * grow_side_by,
                img.0.width + 2 * grow_side_by,
                (grow_side_by, grow_side_by),
                false,
            );

            let mut buf = vec![];

            // Run the algo twice
            for _ in 0..(STEPS / 5) {
                img.run_algo(&algo, &mut buf)
            }
        }

        // Count the number of alive cells
        img.count_alive().to_string()
    }
}

impl Day20 {
    /// Parse the algorithm line and the 2d image grid
    fn parse_data(data: &str) -> (Algorithm, Image) {
        let mut lines = data.lines();

        // Parse the algorithm
        let algo_line = lines.next().expect("Could not get the algorithm line");
        let algo_bin = algo_line.chars().map(|c| c == '#');
        let algorithm = Algorithm(
            algo_bin
                .try_collect_array()
                .expect("Algorithm does not contain the expected number of elements"),
        );

        // Empty line
        let _ = lines.next();

        // Parse the input image
        let image = Image(Grid::from_str_map(lines, |c| c == '#'));

        (algorithm, image)
    }
}
