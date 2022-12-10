use std::fmt::Display;
use std::str::FromStr;

use crate::Solution;

pub struct Day10;

impl Solution for Day10 {
    fn q1(&self, data: &str) -> String {
        let mut instructions = parse1(data);
        let mut cpu = Cpu::new();
        let mut tick = 1u16;

        let mut res = 0u16;
        loop {
            if !cpu.tick(&mut instructions) {
                break;
            }
            tick += 1;

            if [20, 60, 100, 140, 180, 220].contains(&tick) {
                res += tick * cpu.reg_x as u16;
            }
        }

        res.to_string()
    }

    fn q2(&self, data: &str) -> String {
        let mut instructions = parse1(data);
        let mut cpu = Cpu::new();
        let mut crt = Crt::new();
        let mut tick = 1u16;

        loop {
            crt.update(&cpu, tick);

            if !cpu.tick(&mut instructions) {
                break;
            }

            tick += 1;
        }

        crt.to_string()
    }
}

fn parse1(data: &str) -> impl Iterator<Item = Instruction> + '_ {
    data.lines().map(str::trim).flat_map(|s| s.parse())
}

struct Cpu {
    reg_x: i16,
    cur_instruction: Option<Instruction>,
    cycles_to_complete: u8,
}

impl Cpu {
    fn new() -> Self {
        Self {
            reg_x: 1,
            cur_instruction: None,
            cycles_to_complete: 0,
        }
    }
    fn tick(&mut self, instructions: &mut impl Iterator<Item = Instruction>) -> bool {
        // Load an instruction if none was present
        if self.cur_instruction.is_none() {
            self.cur_instruction = instructions.next();

            if let Some(instr) = self.cur_instruction {
                self.cycles_to_complete = instr.cycles();
            } else {
                return false;
            }
        }

        match (self.cycles_to_complete, self.cur_instruction) {
            (0, _) | (_, None) => (),

            (1, Some(instr)) => {
                // Execute the instruction
                match instr {
                    Instruction::Noop => (),
                    Instruction::Addx(v) => self.reg_x += v as i16,
                }
                self.cur_instruction = None;
            }

            _ => self.cycles_to_complete -= 1,
        }

        true
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i8),
}

impl Instruction {
    fn cycles(self) -> u8 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::Noop)
        } else {
            let (_addx, v) = s.split_once(' ').ok_or(())?;
            Ok(Self::Addx(v.parse().map_err(|_| ())?))
        }
    }
}

struct Crt {
    data: [[bool; Self::width() as _]; Self::height() as _],
}

impl Crt {
    const fn width() -> u8 {
        40
    }

    const fn height() -> u8 {
        6
    }

    fn update(&mut self, cpu: &Cpu, tick: u16) {
        let tick_y = (tick - 1) / Self::width() as u16;
        let tick_x = (tick - 1) % Self::width() as u16;
        if tick_y < Self::height() as _ {
            self.data[tick_y as usize][tick_x as usize] =
                i16::abs_diff(tick_x as _, cpu.reg_x) <= 1;
        }
    }

    fn new() -> Self {
        Self {
            data: [[false; Self::width() as _]; Self::height() as _],
        }
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..Self::height() {
            for x in 0..Self::width() {
                write!(
                    f,
                    "{}",
                    if self.data[y as usize][x as usize] {
                        '█'
                    } else {
                        ' '
                    }
                )?;
            }
            if y != Self::height() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Day10;

    #[test]
    fn q1() {
        let day = Day10 {};
        assert_eq!("13140", day.q1(DATA1));
    }

    #[test]
    fn q2() {
        let day = Day10 {};

        assert_eq!(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....",
            day.q2(DATA1)
        );
    }

    const DATA1: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
}
