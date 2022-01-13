use crate::{utils::TryCollectArray, Solution};

pub struct Day21;

struct Die {
    n_roll: u64,
    sides: u32,
}

impl Die {
    fn new(sides: u32) -> Self {
        Self { n_roll: 0, sides }
    }

    fn roll(&mut self) -> u32 {
        let r = (self.n_roll % self.sides as u64) as u32 + 1;
        self.n_roll += 1;
        r
    }
}

struct Pawn {
    position: u32,
}

impl Pawn {
    fn new(position: u32) -> Self {
        Self { position }
    }

    fn move_by(&mut self, n: u32, grid_size: u32) -> u32 {
        self.position += n - 1;
        self.position %= grid_size;
        self.position += 1;
        self.position
    }
}

impl Solution for Day21 {
    /// There are 2 players. Each player roll a deterministic die 3 times
    /// and advance in a 10-cells grid by the sum of those 3 rolls.
    ///
    /// /// The die gives the results 1,2,3,...,99,100,1,2,...
    ///
    /// The score of a player is increased by the cell number it lands onto.
    /// Once a player has reached a score of 1000, it has win.
    ///
    /// Simulate the game, and return the product of the score of
    /// the losing player with the number of die rolls.
    fn q1(&self, data: &str) -> String {
        const DIE_SIDES: u32 = 100;
        const GRID_SIZE: u32 = 10;
        const WINNING_SCORE: u32 = 1000;

        let mut pawns = Self::parse_data(data);
        let mut scores = [0u32; 2];
        let mut die = Die::new(DIE_SIDES);

        'outer: loop {
            for (pawn, score) in pawns.iter_mut().zip(&mut scores) {
                let n = die.roll() + die.roll() + die.roll();
                *score += pawn.move_by(n, GRID_SIZE);
                if *score >= WINNING_SCORE {
                    break 'outer;
                }
            }
        }

        let r = *scores.iter().min().unwrap() as u64 * die.n_roll;
        r.to_string()
    }

    /// TODO
    fn q2(&self, data: &str) -> String {
        let _lines = Self::parse_data(data);
        String::new()
    }
}

impl Day21 {
    /// Parse the starting position of both players.
    fn parse_data(data: &str) -> [Pawn; 2] {
        data.lines()
            .map(|s| s.split_once(": ").expect("Could not parse line").1)
            .map(|s| s.parse().expect("Could not parse starting position to u8"))
            .map(Pawn::new)
            .try_collect_array()
            .unwrap()
    }
}
