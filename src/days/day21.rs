use std::collections::HashMap;

use crate::{utils::TryCollectArray, Solution};

pub struct Day21;

/// A deterministic die, which rolls each side in order in loop: 1,2,...,N,1,2,...
struct Die<const SIDES: u32> {
    n_roll: u64,
}

impl<const SIDES: u32> Die<SIDES> {
    /// Initialize a new die
    fn new() -> Self {
        Self { n_roll: 0 }
    }

    /// Roll the dice, returning the side which it landed on
    fn roll(&mut self) -> u32 {
        let r = (self.n_roll % SIDES as u64) as u32 + 1;
        self.n_roll += 1;
        r
    }
}

/// A player's pawn. It has a position and the player's current score.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pawn {
    position: u8,
    score: u16,
}

impl Pawn {
    /// Initialize the player's pawn at a specific location.
    /// The player's score will be set to 0.
    fn new(position: u8) -> Self {
        Self { position, score: 0 }
    }

    /// Move the pawn by the number of cells, looping back the grid if needed.
    /// The player's score will be increased by the cell number it will land on.
    fn move_by(&mut self, n: u32, grid_size: u32) {
        // Move the pawn
        self.position = ((self.position as u32 + n - 1) % grid_size) as u8;
        self.position += 1;

        // Update its score
        self.score += self.position as u16;
    }
}

/// The result of a turn, either the current player wins, or the game continues.
#[derive(Debug, PartialEq, Eq)]
enum TurnResult {
    Win,
    Continue,
}

/// The state a game. Two different possible game states can be
/// described by different values of this struct.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GameState<const WIN_SCORE: u16, const GRID_SIZE: u32 = 10> {
    pawns: [Pawn; 2],
    turn_player2: bool,
}

impl<const WIN_SCORE: u16, const GRID_SIZE: u32> GameState<WIN_SCORE, GRID_SIZE> {
    /// Initialize a new game
    fn new(pawns: [Pawn; 2]) -> Self {
        Self {
            pawns,
            turn_player2: false,
        }
    }

    /// Move the current player by this number of cells.
    /// Either the current player wins, or the current player switches to the other one.
    fn move_player_by(&mut self, n: u32) -> TurnResult {
        // Select the current player
        let ipawn = self.turn_player2 as usize;
        let pawn = &mut self.pawns[ipawn];

        // Move it
        pawn.move_by(n, GRID_SIZE);

        if pawn.score >= WIN_SCORE {
            // Winning score reached, stop
            TurnResult::Win
        } else {
            // No winning, switch player and continue
            self.turn_player2 = !self.turn_player2;
            TurnResult::Continue
        }
    }
}

impl Solution for Day21 {
    /// There are 2 players. Each player roll a deterministic die 3 times
    /// and advance in a 10-cells grid by the sum of those 3 rolls.
    ///
    /// The die gives the results 1,2,3,...,99,100,1,2,...
    ///
    /// The score of a player is increased by the cell number it lands onto.
    /// Once a player has reached a score of 1000, it has win.
    ///
    /// Simulate the game, and return the product of the score of
    /// the losing player with the number of die rolls.
    fn q1(&self, data: &str) -> String {
        let pawns = Self::parse_data(data);
        let mut game = GameState::<1000>::new(pawns);
        let mut die = Die::<100>::new();

        loop {
            let n = die.roll() + die.roll() + die.roll();
            if game.move_player_by(n) == TurnResult::Win {
                break;
            }
        }

        let r = game.pawns.iter().map(|p| p.score).min().unwrap() as u64 * die.n_roll;
        r.to_string()
    }

    /// Same game but now with a Dirac dice: a "simple" 3-sided die.
    /// At each roll, the universe splits into 3, one per each roll outcome.
    ///
    /// Find the player that wins in more universes.
    /// In how many universes does that player win?
    fn q2(&self, data: &str) -> String {
        let pawns = Self::parse_data(data);
        let game = GameState::new(pawns);

        type Cache = HashMap<GameState<21>, [u64; 2]>;
        let mut cache = HashMap::with_capacity(4096);

        fn play_turn(cache: &mut Cache, state: GameState<21>) -> [u64; 2] {
            // If the result is already in the cache, simply return it
            if let Some(r) = cache.get(&state) {
                return *r;
            }

            let mut wins = [0, 0];
            let iplayer = state.turn_player2 as usize;

            // The player roll the dice 3 times, which means for a 3-sided dice:
            // - n=3 one time (1+1+1)
            // - n=4 three times (1+1+2, 1+2+1, 2+1+1)
            // - ...
            let possibilities = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

            // Play each roll dice
            for (n, times) in possibilities {
                // Copy the game state
                let mut state_copy = state.clone();

                // Move the player
                if state_copy.move_player_by(n) == TurnResult::Win {
                    // The current player has win, count it
                    wins[iplayer] += times;
                } else {
                    // The game continues, the next player will roll the dice
                    let r = play_turn(cache, state_copy);

                    // Add the wins to each player
                    wins[0] += times * r[0];
                    wins[1] += times * r[1];
                }
            }

            // Save the result to the cache for memoization
            cache.insert(state, wins);

            // Return the result
            wins
        }

        // Simulate all possible games
        let wins = play_turn(&mut cache, game);

        // Return the number of universes in which the player with the most wins wins
        wins.iter().max().unwrap().to_string()
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
