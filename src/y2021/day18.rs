use std::ops::Add;

use crate::Solution;

pub struct Day18;

#[derive(Clone)]
enum Value {
    Single(u8),
    Pair(Box<Value>, Box<Value>),
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = Self::Pair(Box::new(self), Box::new(rhs));
        loop {
            let (changed, _, _) = sum.explode(0);
            if changed {
                continue;
            }

            let changed = sum.split();
            if !changed {
                break;
            }
        }
        sum
    }
}

impl Value {
    /// Try to extract a value from the characters iterator.
    /// Return None in case of failure.
    fn extract(chars: &mut dyn Iterator<Item = char>) -> Option<Self> {
        match chars.next()? {
            '[' => {
                let left = Self::extract(chars)?;
                let comma = chars.next()?;
                if comma != ',' {
                    return None;
                }
                let right = Self::extract(chars)?;
                let rbra = chars.next()?;
                if rbra != ']' {
                    return None;
                }
                Some(Self::Pair(Box::new(left), Box::new(right)))
            }
            c @ '0'..='9' => Some(Self::Single(c.to_digit(10)? as _)),
            _ => None,
        }
    }

    fn explode(&mut self, nest_level: u8) -> (bool, u8, u8) {
        match self {
            Self::Single(_) => (false, 0, 0),
            Self::Pair(left, right) => {
                let (changed, l, r) = left.explode(nest_level + 1);
                *right.leftmost_value() += r;
                if changed {
                    return (true, l, 0);
                }

                let (changed, l, r) = right.explode(nest_level + 1);
                *left.rightmost_value() += l;
                if changed {
                    return (true, 0, r);
                }

                if nest_level >= 4 {
                    let left_value = left.value().unwrap();
                    let right_value = right.value().unwrap();
                    *self = Self::Single(0);
                    (true, left_value, right_value)
                } else {
                    (false, 0, 0)
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Single(v) if *v >= 10 => {
                let left_val = *v / 2;
                let left = Box::new(Self::Single(left_val));
                let right = Box::new(Self::Single(*v - left_val));

                *self = Self::Pair(left, right);
                true
            }
            Self::Single(_) => false,
            Self::Pair(left, right) => {
                let changed = left.split();
                if changed {
                    true
                } else {
                    right.split()
                }
            }
        }
    }

    fn value(&self) -> Option<u8> {
        match self {
            Value::Single(v) => Some(*v),
            Value::Pair(_, _) => None,
        }
    }

    fn leftmost_value(&mut self) -> &mut u8 {
        match self {
            Value::Single(v) => v,
            Value::Pair(left, _) => left.leftmost_value(),
        }
    }

    fn rightmost_value(&mut self) -> &mut u8 {
        match self {
            Value::Single(v) => v,
            Value::Pair(_, right) => right.rightmost_value(),
        }
    }
}

impl Solution for Day18 {
    /// Sum all pairs together.
    /// Compute the magnitude of the result.
    fn q1(&self, data: &str) -> String {
        let pairs = Self::parse_data(data);
        let sum = pairs.reduce(|acc, v| acc + v).unwrap();

        fn magnitude(value: &Value) -> u64 {
            match value {
                Value::Single(v) => *v as _,
                Value::Pair(left, right) => 3 * magnitude(left) + 2 * magnitude(right),
            }
        }

        let magnitude = magnitude(&sum);
        magnitude.to_string()
    }

    /// What is the largest magnitude you can get from adding
    /// only two of the snailfish numbers?
    fn q2(&self, data: &str) -> String {
        let pairs = Self::parse_data(data).collect::<Vec<_>>();

        fn magnitude(value: &Value) -> u64 {
            match value {
                Value::Single(v) => *v as _,
                Value::Pair(left, right) => 3 * magnitude(left) + 2 * magnitude(right),
            }
        }

        let mut max_mag = 0;
        for i in 0..(pairs.len() - 1) {
            let a = &pairs[i];
            for b in pairs.iter().skip(i + 1) {
                let mag1 = magnitude(&(a.clone() + b.clone()));
                let mag2 = magnitude(&(b.clone() + a.clone()));
                max_mag = max_mag.max(mag1).max(mag2);
            }
        }
        max_mag.to_string()
    }
}

impl Day18 {
    /// Parse all pairs (one per line)
    fn parse_data(data: &str) -> impl Iterator<Item = Value> + '_ {
        data.split_terminator('\n').map(|line| {
            Value::extract(&mut line.trim().chars()).expect("Could not parse input data")
        })
    }
}
