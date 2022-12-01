use std::str::FromStr;

use itertools::Itertools;

use crate::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn q1(&self, data: &str) -> String {
        // Parse the data
        let meals: Meals = data.parse().unwrap();

        meals
            .0
            .into_iter()
            // Sum each meal calories
            .map(meal_calories)
            // Find the max element
            .max()
            .unwrap()
            .to_string()
    }

    fn q2(&self, data: &str) -> String {
        // Parse the data
        let meals: Meals = data.parse().unwrap();

        meals
            .0
            .into_iter()
            // Sum each meal calories
            .map(meal_calories)
            // Find the top 3. itertools only have k_smallest, so negate
            // the numbers to make it work, then transform them back
            .map(|n| -(n as i64))
            .k_smallest(3)
            .map(|n| -n as u64)
            // Sum them
            .sum::<u64>()
            .to_string()
    }
}

type FoodCalories = u64;
type Meal = Vec<FoodCalories>;
struct Meals(Vec<Meal>);

impl FromStr for Meals {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Meals(
            // Split by blank lines to get groups
            s.split("\n\n")
                .map(|s| {
                    // Get all of the group calories line per line
                    s.split('\n')
                        // Trim & remove potentially empty lines
                        .map(|line| line.trim())
                        .filter(|line| !line.is_empty())
                        // Parse into an int
                        .map(|line| line.parse().unwrap())
                        .collect::<Meal>()
                })
                // Do not keep potentially empty groups
                .filter(|v| !v.is_empty())
                .collect(),
        ))
    }
}

/// Compute a meal total calories count
fn meal_calories(meal: Meal) -> u64 {
    meal.into_iter().sum::<u64>()
}
