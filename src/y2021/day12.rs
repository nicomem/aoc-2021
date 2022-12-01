use itertools::Itertools;

use crate::Solution;

pub struct Day12;

type Room = usize;

struct CaveMap {
    /// Number of rooms in the cave
    rooms: Room,

    /// Whether each room is big or small
    small_rooms: Vec<bool>,

    /// The rooms one can go from another
    paths: Vec<Vec<Room>>,

    /// The names of the rooms
    names: Vec<String>,

    /// The starting room
    start: Room,

    /// The ending room
    end: Room,
}

impl CaveMap {
    /// Try to find a room index by its name.
    /// If it does not exist, insert it and return its index.
    fn find_or_push_room(&mut self, name: &str) -> Room {
        if let Some(idx) = self
            .names
            .iter()
            .find_position(|&s| s == name)
            .map(|(i, _)| i)
        {
            idx as _
        } else {
            self.rooms += 1;
            self.names.push(name.to_string());
            self.small_rooms
                .push(name.chars().all(|c| c.is_lowercase()));
            self.paths.push(vec![]);

            match name {
                "start" => self.start = self.rooms - 1,
                "end" => self.end = self.rooms - 1,
                _ => {}
            }

            self.rooms - 1
        }
    }

    /// Visit the cave from a room.
    /// At each path, will call `advance` which must return whether to advance to this room or not.
    /// If advancing, do the same for all paths in this room.
    /// After having advanced and processed a room, `go_back` will be called.
    fn visit<FADV, FBACK, BAG>(
        &self,
        from: Room,
        advance: &mut FADV,
        go_back: &mut FBACK,
        travel_bag: &mut BAG,
    ) where
        FADV: FnMut(Room, Room, &mut BAG) -> bool,
        FBACK: FnMut(Room, Room, &mut BAG),
    {
        let paths = &self.paths[from];
        for &to in paths {
            if advance(from, to, travel_bag) {
                self.visit(to, advance, go_back, travel_bag);
                go_back(to, from, travel_bag);
            }
        }
    }
}

impl Solution for Day12 {
    /// Find the number of unique paths from start to end.
    fn q1(&self, data: &str) -> String {
        let map = Self::parse_data(data);

        let mut npaths = 0u64;
        let mut visited = vec![false; map.rooms];
        visited[map.start] = true;

        let mut advance = |_from, to, visited: &mut Vec<bool>| {
            // If advancing goes to the end, mark the path directly without advancing
            if to == map.end {
                npaths += 1;
                return false;
            }

            // Cannot advance twice to the same small room
            if map.small_rooms[to] && visited[to] {
                return false;
            }

            // Advance and update the path
            visited[to] = true;
            true
        };

        let mut go_back = |from, _to, visited: &mut Vec<bool>| {
            visited[from] = false;
        };

        map.visit(map.start, &mut advance, &mut go_back, &mut visited);
        npaths.to_string()
    }

    /// Same as q1 but can visit a single small cave twice (expect start & end)
    fn q2(&self, data: &str) -> String {
        let map = Self::parse_data(data);
        let mut npaths = 0u64;

        let mut advance = |_from, to, (visited, visited_twice): &mut (Vec<bool>, Option<Room>)| {
            // If advancing goes to the end, mark the path directly without advancing
            if to == map.end {
                npaths += 1;
                return false;
            }

            // Cannot advance twice to the same small room
            if map.small_rooms[to] && visited[to] {
                // If can visit twice this small room, do it
                if visited_twice.is_none() && (to != map.start && to != map.end) {
                    *visited_twice = Some(to);
                    return true;
                }
                return false;
            }

            // Advance and update the path
            visited[to] = true;
            true
        };

        let mut go_back = |from, _to, (visited, visited_twice): &mut (Vec<bool>, Option<Room>)| {
            if let Some(room) = visited_twice {
                if *room == from {
                    *visited_twice = None;
                    return;
                }
            }

            visited[from] = false;
        };

        let visited_twice: Option<Room> = None;
        let mut visited = vec![false; map.rooms];
        visited[map.start] = true;

        map.visit(
            map.start,
            &mut advance,
            &mut go_back,
            &mut (visited, visited_twice),
        );
        npaths.to_string()
    }
}

impl Day12 {
    /// Parse the cave map
    fn parse_data(data: &str) -> CaveMap {
        let mut cave_map = CaveMap {
            rooms: 0,
            small_rooms: vec![],
            paths: vec![],
            names: vec![],
            start: 0,
            end: 0,
        };

        for (from, to) in data
            .split_terminator('\n')
            .map(|line| line.split_once('-').unwrap())
        {
            let from = cave_map.find_or_push_room(from);
            let to = cave_map.find_or_push_room(to);

            cave_map.paths[from].push(to);
            cave_map.paths[to].push(from);
        }

        for path in &mut cave_map.paths {
            path.sort_unstable();
        }

        cave_map
    }
}
