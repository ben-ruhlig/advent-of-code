use std::{fs, path,};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn_right(&mut self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Position{
        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy)]
struct Boundary {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy)]
struct GuardPosition {
    position: Position,
    direction: Direction,
}

impl GuardPosition {
    fn new(row: usize, col: usize, direction: Direction) -> Self {
        Self {
            position: Position { row, col },
            direction,
        }
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }
}

#[derive(Debug, PartialEq)]
enum MapStatus {
    Complete,
    Incomplete,
}

#[derive(Debug)]
pub struct Map {
    boundary: Boundary,                 // map boundary
    guard: GuardPosition,               // guard position and direction
    obstacles: HashSet<Position>,       // existing obstacles
    visited: HashSet<Position>,         // visited positions
    new_obstacles: HashSet<Position>,   // obstacles that would create cycles
    status: MapStatus,                  // map status complete or incomplete
}

impl Map {
    pub fn create_from_string(map_str: &str) -> Self {
        let dir_extract = |c: char| -> Direction {
            match c {
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '^' => Direction::Up,
                _ => panic!("Invalid direction."),
            }
        };

        let mut guard: Option<GuardPosition> = None;
        let mut obstacles: HashSet<Position> = HashSet::new();
        let mut visited: HashSet<Position> = HashSet::new();
        let new_obstacles: HashSet<Position> = HashSet::new();
        let mut row: usize = 0;
        let mut col: usize = 0;
        
        for map_str in map_str.lines() {
            col = 0;
            map_str
                .chars()
                .for_each(|c| {
                    if c == '#' {
                        obstacles.insert(Position::new(row, col));
                    } else if c == '>' || c == 'v' || c == '<' || c == '^' {
                        guard = Some(GuardPosition::new(row, col, dir_extract(c)));
                        visited.insert(Position::new(row, col));
                    }
                    col += 1;
                });
            row += 1;
        };

        // default map status is incomplete
        let mut status = MapStatus::Incomplete;

        // Helper functions
        let inbounds = |(r, c): (usize, usize)| -> bool {
            r < row && c < col
        };
        let contains_obstacle = |(r, c): (usize, usize)| -> bool {
            obstacles.contains(&Position::new(r, c))
        };
        
        // If there's a guard, check if it's surrounded by obstacles. If it is, the map is complete.
        if let Some(g) = guard {
            if
                inbounds((g.position.row, g.position.col + 1)) && contains_obstacle((g.position.row, g.position.col + 1))
                && inbounds((g.position.row + 1, g.position.col)) && contains_obstacle((g.position.row + 1, g.position.col))
                && inbounds((g.position.row, g.position.col - 1)) && contains_obstacle((g.position.row, g.position.col - 1))
                && inbounds((g.position.row - 1, g.position.col)) && contains_obstacle((g.position.row - 1, g.position.col))
            {
                status = MapStatus::Complete;
            }
        } else {
            panic!("Guard not found.");

        }

        Self {
            boundary: Boundary {row, col},
            guard: guard.unwrap(),  // Will panic before this line if guard is None
            obstacles,
            visited,
            new_obstacles,
            status,
        }
    }

    fn unique_visits(&self) -> usize {
        self.visited.len() as usize
    }

    fn unique_cycle_generating_obstacles(&self) -> usize {
        self.new_obstacles.len() as usize
    }

    fn visited(&mut self, position: Position) {
        self.visited.insert(position);
    }

    fn complete(&mut self) {
        self.status = MapStatus::Complete;
    }

    fn get_next_position(&self) -> Option<Position> {
        let mut next_p: Option<Position> = None;
        let curr_p = self.guard.position;
        let d = self.guard.direction;

        let lower_bound_issue = |curr_p: Position, d: Direction| -> bool {
            curr_p.row == 0 && d == Direction::Up || curr_p.col == 0 && d == Direction::Left
        };

        let upper_bound_issue = |next_p: Position, b: Boundary| {
            next_p.row > b.row  || next_p.col > b.col
        };

        let get_next_position = |curr_p: Position, d: Direction| -> Position {
            match d {
                Direction::Right => Position::new(curr_p.row, curr_p.col + 1),
                Direction::Down => Position::new(curr_p.row + 1, curr_p.col),
                Direction::Left => Position::new(curr_p.row, curr_p.col - 1),
                Direction::Up => Position::new(curr_p.row - 1, curr_p.col),
            }
        };

        if !lower_bound_issue(curr_p, d) {
            let potential_next_p = get_next_position(curr_p, d);
            if !upper_bound_issue(potential_next_p, self.boundary) {
                next_p = Some(potential_next_p);
            }
        }
        next_p
    }

    fn next_position(&mut self) {
        let next_p = self.get_next_position();
        if next_p.is_none() {
            self.complete();
        } else {
            let next_p = next_p.unwrap();
            if self.obstacles.contains(&next_p) {
                self.guard.turn_right();
                self.next_position();
            } else {
                self.guard.position = next_p.clone();
                let potential_cycle_obstacle = self.get_next_position();
                if self.visited.contains(&next_p) && !potential_cycle_obstacle.is_none() {
                    self.new_obstacles.insert(potential_cycle_obstacle.unwrap());
                }
                self.visited(next_p);
            }
        }
    }
}

fn get_and_process_map(filepath: &path::Path) -> Map {
    let input = fs::read_to_string(filepath).expect("Failed to read file to string.");
    let mut map = Map::create_from_string(&input);
    while map.status == MapStatus::Incomplete {
        map.next_position();
    };
    map
}

pub fn get_answer_p1(filepath: &path::Path) -> usize {
    let map = get_and_process_map(filepath);
    map.unique_visits()
}

pub fn get_answer_p2(filepath: &path::Path) -> usize {
    let map = get_and_process_map(filepath);
    map.unique_cycle_generating_obstacles()
}

fn main() {
    let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day06.txt");
    println!("Day06 P1: {}", get_answer_p1(&filepath));
    // println!("Day06 P2: {}", get_answer_p2(&filepath));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_p1() {
        let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day06_test.txt");
        assert_eq!(42, get_answer_p1(&filepath));
    }
    #[test]
    fn day06_p2() {
        let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day06_test.txt");
        assert_eq!(2, get_answer_p2(&filepath));
    }
}
