use std::str::FromStr;

use crate::Solution;

pub struct Day2;

enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, num) = s.split_once(' ').ok_or("Not in '<action> <num>' format")?;
        let num: u64 = num.parse().map_err(|_| "Could not parse number")?;

        match action {
            "forward" => Ok(Command::Forward(num)),
            "down" => Ok(Command::Down(num)),
            "up" => Ok(Command::Up(num)),
            _ => Err("Action not recognized".to_string()),
        }
    }
}

impl Solution for Day2 {
    /// Move based on the commands and multiply the final position and depth
    fn q1(&self, data: &str) -> String {
        let commands = Self::parse_data(data);

        let mut pos = 0;
        let mut depth = 0i64;

        for command in commands {
            match command {
                Command::Forward(n) => pos += n,
                Command::Down(n) => depth += n as i64,
                Command::Up(n) => depth -= n as i64,
            };
        }

        (pos as i64 * depth).to_string()
    }

    /// Same as q1 but adding the aim this time
    fn q2(&self, data: &str) -> String {
        let commands = Self::parse_data(data);

        let mut pos = 0;
        let mut depth = 0i64;
        let mut aim = 0i64;

        for command in commands {
            match command {
                Command::Forward(n) => {
                    pos += n;
                    depth += aim * n as i64
                }
                Command::Down(n) => aim += n as i64,
                Command::Up(n) => aim -= n as i64,
            };
        }

        (pos as i64 * depth).to_string()
    }
}

impl Day2 {
    /// Read one number per line
    fn parse_data(data: &str) -> impl Iterator<Item = Command> + '_ {
        data.split_terminator('\n')
            .map(|s| s.parse::<Command>().expect("Could not parse command"))
    }
}
