use std::cmp::Ordering;
use std::collections::HashSet;
use std::num::NonZeroU8;

use crate::Solution;

pub struct Day9;

impl Solution for Day9 {
    fn q1(&self, data: &str) -> String {
        let movements = parse1(data);

        let mut visited = HashSet::new();

        let mut head = Position { y: 0, x: 0 };
        let mut tail = head;
        visited.insert(tail);
        for mvmt in movements {
            head = advance_head(head, mvmt);
            tail = advance_tail(tail, head, Some(&mut visited));
        }
        visited.len().to_string()
    }

    fn q2(&self, data: &str) -> String {
        let movements = parse1(data);

        let mut visited = HashSet::new();

        let mut rope = [Position { y: 0, x: 0 }; 10];
        visited.insert(Position { y: 0, x: 0 });
        for mvmt in movements {
            advance_rope(&mut rope, mvmt, &mut visited);
        }
        visited.len().to_string()
    }
}

fn parse1(data: &str) -> impl Iterator<Item = Movement> + '_ {
    data.lines().map(str::trim).map(|line| {
        let (dir, dist) = line.split_once(' ').unwrap();
        let dir = match dir.chars().next().unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!("Invalid position"),
        };
        let dist = dist.parse().unwrap();
        Movement { dir, dist }
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    y: i16,
    x: i16,
}

#[must_use]
fn advance_head(mut head: Position, mvmt: Movement) -> Position {
    match mvmt.dir {
        Direction::Up => head.y -= mvmt.dist.get() as i16,
        Direction::Down => head.y += mvmt.dist.get() as i16,
        Direction::Left => head.x -= mvmt.dist.get() as i16,
        Direction::Right => head.x += mvmt.dist.get() as i16,
    }
    head
}

#[must_use]
fn advance_tail(
    mut tail: Position,
    head: Position,
    mut visited: Option<&mut HashSet<Position>>,
) -> Position {
    loop {
        let dx = match head.x.cmp(&tail.x) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        let dy = match head.y.cmp(&tail.y) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };

        let new_position = Position {
            y: tail.y + dy,
            x: tail.x + dx,
        };
        if new_position == head {
            // Tail is already next to the head, stop here
            return tail;
        }
        tail = new_position;
        if let Some(ref mut visited) = visited {
            visited.insert(tail);
        }
    }
}

fn advance_rope(rope: &mut [Position; 10], mvmt: Movement, visited: &mut HashSet<Position>) {
    let (head, rest) = rope.split_first_mut().unwrap();
    let (tail, middles) = rest.split_last_mut().unwrap();

    const ONE: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(1) };
    for _i in 0..mvmt.dist.get() {
        *head = advance_head(
            *head,
            Movement {
                dir: mvmt.dir,
                dist: ONE,
            },
        );
        let mut prev = *head;
        for middle in &mut *middles {
            *middle = advance_tail(*middle, prev, None);
            prev = *middle;
        }
        *tail = advance_tail(*tail, prev, Some(visited));
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct Movement {
    dir: Direction,
    dist: NonZeroU8,
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Day9;

    #[test]
    fn q1() {
        let day = Day9 {};
        assert_eq!(
            "13",
            day.q1("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2")
        );
    }

    #[test]
    fn q2() {
        let day = Day9 {};

        assert_eq!(
            "1",
            day.q2("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2")
        );

        assert_eq!(
            "36",
            day.q2("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20")
        );
    }
}
