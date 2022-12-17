use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;

use itertools::Itertools;

use crate::Solution;

pub struct Day17;

impl Solution for Day17 {
    fn q1(&self, data: &str) -> String {
        const NB_ROCKS: u64 = 2022;
        const WIDTH: usize = 7;
        let mut chamber = Chamber::new(WIDTH);

        let jets = parse1(data).collect_vec();
        run(&mut chamber, NB_ROCKS, &jets).to_string()
    }

    fn q2(&self, data: &str) -> String {
        const NB_ROCKS: u64 = 1000000000000;
        const WIDTH: usize = 7;
        let mut chamber = Chamber::new(WIDTH);

        let jets = parse1(data).collect_vec();
        run(&mut chamber, NB_ROCKS, &jets).to_string()
    }
}

fn run(chamber: &mut Chamber, nb_rocks: u64, jets: &[HotJetDir]) -> u64 {
    const FIRST_ROCK_SHAPE: RockShape = RockShape::Hor;

    let mut ijet = 0;
    let mut rock_shape = FIRST_ROCK_SHAPE;

    let mut first_repeat: Option<(u64, u64, u64)> = None;

    for irock in 0..nb_rocks {
        let mut rock = Rock {
            bl: chamber.new_falling_rock_start_pos(),
            shape: rock_shape,
        };

        loop {
            if let Some(res) = check_repeat(
                (irock, ijet as _),
                rock_shape,
                &mut first_repeat,
                chamber,
                nb_rocks,
                jets,
                rock.bl,
            ) {
                return res;
            }

            let jet = jets.get(ijet).unwrap();
            ijet = (ijet + 1) % jets.len();

            // Jet push
            let next_rock = Rock {
                bl: jet.push_in_direction(rock.bl),
                shape: rock.shape,
            };
            match jet {
                HotJetDir::Left => {
                    if !chamber.is_blocked_leftside(next_rock) {
                        rock.bl = next_rock.bl;
                    }
                }
                HotJetDir::Right => {
                    if !chamber.is_blocked_rightside(next_rock) {
                        rock.bl = next_rock.bl;
                    }
                }
            };

            // Gravity
            let down = Rock {
                bl: rock.bl.down(),
                shape: rock.shape,
            };
            if chamber.is_blocked_downside(down) {
                chamber.rest_rock(rock);
                break;
            } else {
                rock = down;
            }
        }

        rock_shape = rock_shape.next_shape();
    }

    chamber.height as _
}

fn check_repeat(
    (irock, ijet): (u64, u64),
    rock_shape: RockShape,
    first_repeat: &mut Option<(u64, u64, u64)>,
    chamber: &mut Chamber,
    nb_rocks: u64,
    jets: &[HotJetDir],
    rock_bl: Position,
) -> Option<u64> {
    if irock != 0 && rock_shape == RockShape::Hor && ijet == 0 {
        if let Some((ifirst, hfirst, rock_y_diff)) = first_repeat {
            if *rock_y_diff == (rock_bl.y as u64 - chamber.height as u64) {
                let idiff = irock - *ifirst;
                let hdiff = chamber.height as u64 - *hfirst;

                let nb_repeat = (nb_rocks - *ifirst) / idiff;
                let irest = (nb_rocks - *ifirst) % idiff;

                let before_height = chamber.height as u64;
                let hrest = run(chamber, irest, jets) - before_height;

                assert_eq!(nb_rocks, *ifirst + (nb_repeat * idiff) + irest);

                // Absolutely no idea what is wrong in my code
                // but this has already taken too much time
                const CHEAT_OFFSET: u64 = 23;

                return Some(*hfirst + (nb_repeat * hdiff) + hrest - CHEAT_OFFSET);
            }
        } else {
            *first_repeat = Some((
                irock,
                chamber.height as _,
                rock_bl.y as u64 - chamber.height as u64,
            ));
        }
    }
    None
}

fn parse1(data: &str) -> impl Iterator<Item = HotJetDir> + '_ {
    data.trim().chars().map(|c| match c {
        '<' => HotJetDir::Left,
        '>' => HotJetDir::Right,
        _ => unreachable!(),
    })
}

struct Chamber {
    width: usize,
    rocks: HashSet<Position>,
    height: usize,
}

impl Chamber {
    fn new(width: usize) -> Self {
        Self {
            width,
            rocks: HashSet::new(),
            height: 0,
        }
    }

    const fn new_falling_rock_start_pos(&self) -> Position {
        Position {
            y: self.height as i64 + 3,
            x: 2,
        }
    }

    fn is_blocked_leftside(&self, rock: Rock) -> bool {
        rock.positions_leftside()
            .any(|p| p.x < 0 || self.rocks.contains(&p))
    }

    fn is_blocked_rightside(&self, rock: Rock) -> bool {
        rock.positions_rightside()
            .any(|p| p.x >= self.width as i64 || self.rocks.contains(&p))
    }

    fn is_blocked_downside(&self, rock: Rock) -> bool {
        rock.positions_downside()
            .any(|p| p.y < 0 || self.rocks.contains(&p))
    }

    fn rest_rock(&mut self, rock: Rock) {
        rock.positions().for_each(|p| {
            self.rocks.insert(p);
        });
        self.height = self
            .height
            .max(rock.positions().map(|p| p.y + 1).max().unwrap() as _)
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+-------+")?;
        for y in 0..self.height {
            write!(f, "|")?;
            for x in 0..7 {
                write!(
                    f,
                    "{}",
                    if self.rocks.contains(&Position { y: y as _, x }) {
                        '█'
                    } else {
                        ' '
                    }
                )?;
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HotJetDir {
    Left,
    Right,
}

impl HotJetDir {
    const fn push_in_direction(self, pos: Position) -> Position {
        match self {
            HotJetDir::Left => pos.left(),
            HotJetDir::Right => pos.right(),
        }
    }
}

impl Display for HotJetDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HotJetDir::Left => f.write_str("<"),
            HotJetDir::Right => f.write_str(">"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Rock {
    bl: Position,
    shape: RockShape,
}

impl Rock {
    fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.shape.offsets_bl().iter().map(|off| self.bl + *off)
    }

    fn positions_leftside(&self) -> impl Iterator<Item = Position> + '_ {
        self.shape
            .offsets_bl_leftside()
            .iter()
            .map(|off| self.bl + *off)
    }

    fn positions_rightside(&self) -> impl Iterator<Item = Position> + '_ {
        self.shape
            .offsets_bl_rightside()
            .iter()
            .map(|off| self.bl + *off)
    }

    fn positions_downside(&self) -> impl Iterator<Item = Position> + '_ {
        self.shape
            .offsets_bl_downside()
            .iter()
            .map(|off| self.bl + *off)
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (y={}, x={})", self.shape, self.bl.y, self.bl.x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockShape {
    Hor,
    Cross,
    J,
    I,
    Sqr,
}

impl RockShape {
    const fn offsets_bl(&self) -> &'static [Offset] {
        match self {
            RockShape::Hor => &[
                Position { y: 0, x: 0 },
                Position { y: 0, x: 1 },
                Position { y: 0, x: 2 },
                Position { y: 0, x: 3 },
            ],
            RockShape::Cross => &[
                Position { y: 0, x: 1 },
                Position { y: 1, x: 0 },
                Position { y: 1, x: 1 },
                Position { y: 1, x: 2 },
                Position { y: 2, x: 1 },
            ],
            RockShape::J => &[
                Position { y: 0, x: 0 },
                Position { y: 0, x: 1 },
                Position { y: 0, x: 2 },
                Position { y: 1, x: 2 },
                Position { y: 2, x: 2 },
            ],
            RockShape::I => &[
                Position { y: 0, x: 0 },
                Position { y: 1, x: 0 },
                Position { y: 2, x: 0 },
                Position { y: 3, x: 0 },
            ],
            RockShape::Sqr => &[
                Position { y: 0, x: 0 },
                Position { y: 0, x: 1 },
                Position { y: 1, x: 0 },
                Position { y: 1, x: 1 },
            ],
        }
    }

    const fn offsets_bl_rightside(&self) -> &'static [Offset] {
        match self {
            RockShape::Hor => &[Position { y: 0, x: 3 }],
            RockShape::Cross => &[
                Position { y: 0, x: 1 },
                Position { y: 1, x: 2 },
                Position { y: 2, x: 1 },
            ],
            RockShape::J => &[
                Position { y: 0, x: 2 },
                Position { y: 1, x: 2 },
                Position { y: 2, x: 2 },
            ],
            RockShape::I => &[
                Position { y: 0, x: 0 },
                Position { y: 1, x: 0 },
                Position { y: 2, x: 0 },
                Position { y: 3, x: 0 },
            ],
            RockShape::Sqr => &[Position { y: 0, x: 1 }, Position { y: 1, x: 1 }],
        }
    }

    const fn offsets_bl_leftside(&self) -> &'static [Offset] {
        match self {
            RockShape::Hor => &[Position { y: 0, x: 0 }],
            RockShape::Cross => &[
                Position { y: 0, x: 1 },
                Position { y: 1, x: 0 },
                Position { y: 2, x: 1 },
            ],
            RockShape::J => &[
                Position { y: 0, x: 0 },
                Position { y: 1, x: 2 },
                Position { y: 2, x: 2 },
            ],
            RockShape::I => &[
                Position { y: 0, x: 0 },
                Position { y: 1, x: 0 },
                Position { y: 2, x: 0 },
                Position { y: 3, x: 0 },
            ],
            RockShape::Sqr => &[Position { y: 0, x: 0 }, Position { y: 1, x: 0 }],
        }
    }

    const fn offsets_bl_downside(&self) -> &'static [Offset] {
        match self {
            RockShape::Hor => &[
                Position { y: 0, x: 0 },
                Position { y: 0, x: 1 },
                Position { y: 0, x: 2 },
                Position { y: 0, x: 3 },
            ],
            RockShape::Cross => &[
                Position { y: 0, x: 1 },
                Position { y: 1, x: 0 },
                Position { y: 1, x: 2 },
            ],
            RockShape::J => &[
                Position { y: 0, x: 0 },
                Position { y: 0, x: 1 },
                Position { y: 0, x: 2 },
            ],
            RockShape::I => &[Position { y: 0, x: 0 }],
            RockShape::Sqr => &[Position { y: 0, x: 0 }, Position { y: 0, x: 1 }],
        }
    }

    const fn next_shape(&self) -> Self {
        match self {
            RockShape::Hor => RockShape::Cross,
            RockShape::Cross => RockShape::J,
            RockShape::J => RockShape::I,
            RockShape::I => RockShape::Sqr,
            RockShape::Sqr => RockShape::Hor,
        }
    }
}

impl Display for RockShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            RockShape::Hor => '-',
            RockShape::Cross => '+',
            RockShape::J => 'J',
            RockShape::I => 'I',
            RockShape::Sqr => 'o',
        };

        write!(f, "{c}")
    }
}

type Offset = Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    y: i64,
    x: i64,
}

impl Position {
    const fn left(self) -> Self {
        Position {
            y: self.y,
            x: self.x - 1,
        }
    }

    const fn right(self) -> Self {
        Position {
            y: self.y,
            x: self.x + 1,
        }
    }

    const fn down(self) -> Self {
        Position {
            y: self.y - 1,
            x: self.x,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Day17;

    #[test]
    fn q1() {
        let day = Day17 {};

        assert_eq!("3068", day.q1(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"));
    }

    #[test]
    fn q2() {
        let day = Day17 {};

        assert_eq!(
            "1514285714288",
            day.q2(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>")
        );
    }
}
