use std::cell::RefCell;
use std::fmt::Display;
use std::str::FromStr;

use itertools::Itertools;

use crate::Solution;

pub struct Day11;

impl Solution for Day11 {
    fn q1(&self, data: &str) -> String {
        let monkeys: Monkeys = parse1(data).map(RefCell::new).collect();
        let mut inspected = vec![0u64; monkeys.len()];

        for _round in 0..20 {
            for (i, monkey) in monkeys.iter().enumerate() {
                // Will inspect all its elements during this round
                inspected[i] += monkey.borrow().items.len() as u64;

                monkey.borrow_mut().inspect_items(&monkeys, None);
            }
        }

        inspected.sort_unstable();
        (inspected[inspected.len() - 1] * inspected[inspected.len() - 2]).to_string()
    }

    fn q2(&self, data: &str) -> String {
        let monkeys: Monkeys = parse1(data).map(RefCell::new).collect();
        let mut inspected = vec![0u64; monkeys.len()];

        let convenient_modulo = monkeys
            .iter()
            .map(|monkey| monkey.borrow().div_by)
            .reduce(|acc, e| acc * e);

        for _round in 0..10000 {
            for (i, monkey) in monkeys.iter().enumerate() {
                // Will inspect all its elements during this round
                inspected[i] += monkey.borrow().items.len() as u64;

                monkey
                    .borrow_mut()
                    .inspect_items(&monkeys, convenient_modulo);
            }
        }

        inspected.sort_unstable();
        (inspected[inspected.len() - 1] * inspected[inspected.len() - 2]).to_string()
    }
}

fn parse1(data: &str) -> impl Iterator<Item = Monkey> + '_ {
    data.split("\n\n").map(str::trim).flat_map(|s| s.parse())
}

type Monkeys = Vec<RefCell<Monkey>>;
type ItemWorry = u64;
type MonkeyIdx = u8;

struct Monkey {
    items: Vec<ItemWorry>,
    op: fn(ItemWorry, ItemWorry) -> ItemWorry,
    op_val: Option<ItemWorry>,
    div_by: ItemWorry,
    if_true_throw_to: MonkeyIdx,
    if_false_throw_to: MonkeyIdx,
}

impl Monkey {
    fn inspect_items(&mut self, monkeys: &Monkeys, modulo: Option<ItemWorry>) {
        for &item in &self.items {
            // Inspect the item
            let item = self.operation(item);

            let item = if let Some(modulo) = modulo {
                // Part2: Monkey does not get bored, manage the worriness by taking a convenient modulo
                item % modulo
            } else {
                // Part1: Monkey gets bored
                item / 3
            };

            monkeys[if self.test(item) {
                self.if_true_throw_to
            } else {
                self.if_false_throw_to
            } as usize]
                .borrow_mut()
                .items
                .push(item);
        }
        self.items.clear();
    }

    const fn test(&self, item: ItemWorry) -> bool {
        item % self.div_by == 0
    }

    fn operation(&self, item: ItemWorry) -> ItemWorry {
        if let Some(v) = self.op_val {
            (self.op)(item, v)
        } else {
            (self.op)(item, item)
        }
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Monkey:")?;

        let joined_items = self.items.iter().map(|item| item.to_string()).join(", ");
        writeln!(f, "  Items: {joined_items}")?;

        let hint0 = self.operation(0);
        let hint1 = self.operation(1);
        let op_hint = match (hint0, hint1) {
            (0, 1) => "* old".to_string(),
            (0, e) => format!("* {e}"),
            (x, y) if y == x + 1 => format!("+ {x}"),
            _ => unreachable!(),
        };
        writeln!(f, "  Operation: new = old {op_hint}")?;

        writeln!(f, "  Divisible by {}", self.div_by)?;

        writeln!(f, "    If true: throw to monkey {}", self.if_true_throw_to)?;
        writeln!(
            f,
            "    If false: throw to monkey {}",
            self.if_false_throw_to
        )?;

        Ok(())
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n').map(str::trim);

        // Monkey X:
        lines.next();

        // Starting items: X, Y, Z
        let starting_items = lines.next().ok_or(())?;
        let starting_items = starting_items.strip_prefix("Starting items: ").ok_or(())?;
        let starting_items: Vec<ItemWorry> =
            starting_items.split(", ").flat_map(|s| s.parse()).collect();

        // Operation: new = old (+|*) (old|X)
        let operation = lines.next().ok_or(())?;
        let operation = operation.strip_prefix("Operation: new = old ").ok_or(())?;
        let op: fn(ItemWorry, ItemWorry) -> ItemWorry = match operation.chars().next().unwrap() {
            '+' => |old, v| old + v,
            '*' => |old, v| old * v,
            _ => unreachable!(),
        };
        let op_val = match operation.split(' ').last().unwrap() {
            "old" => None,
            v => v.parse().ok(),
        };

        // Test: divisible by X
        let test = lines.next().ok_or(())?;
        let div_by = test
            .strip_prefix("Test: divisible by ")
            .ok_or(())?
            .parse::<ItemWorry>()
            .map_err(|_| ())?;

        // If true: throw to monkey X
        let if_true = lines.next().ok_or(())?;
        let if_true_throw_to = if_true
            .strip_prefix("If true: throw to monkey ")
            .ok_or(())?
            .parse::<MonkeyIdx>()
            .map_err(|_| ())?;

        // If false: throw to monkey X
        let if_false = lines.next().ok_or(())?;
        let if_false_throw_to = if_false
            .strip_prefix("If false: throw to monkey ")
            .ok_or(())?
            .parse::<MonkeyIdx>()
            .map_err(|_| ())?;

        Ok(Self {
            items: starting_items,
            op,
            op_val,
            div_by,
            if_true_throw_to,
            if_false_throw_to,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Day11;

    #[test]
    fn q1() {
        let day = Day11 {};

        assert_eq!("10605", day.q1(DATA1));
    }

    #[test]
    fn q2() {
        let day = Day11 {};

        assert_eq!("2713310158", day.q2(DATA1));
    }

    const DATA1: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
}
