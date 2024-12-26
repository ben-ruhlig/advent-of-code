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
pub struct Position {
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
}

#[derive(Debug, PartialEq)]
enum DirectedCycle {
    Yes,
    No,
}

#[derive(Debug, PartialEq)]
enum TraverseStatus {
    Incomplete,
    Complete,
}

#[derive(Debug)]
pub struct Map {
    boundary: Boundary,                 // map boundary
    start_guard: GuardPosition,         // guard position and direction
    obstacles: HashSet<Position>,       // existing obstacles
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

        let mut start_guard: Option<GuardPosition> = None;
        let mut obstacles: HashSet<Position> = HashSet::new();
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
                        start_guard = Some(GuardPosition::new(row, col, dir_extract(c)));
                    }
                    col += 1;
                });
            row += 1;
        };

        if start_guard.is_none() {
            panic!("Guard not found.");
        }

        Self {
            boundary: Boundary {row, col},
            start_guard: start_guard.unwrap(),  // Will panic before this line if guard is None
            obstacles,
        }
    }

    fn next_position_valid(&self, position: &Position, direction: &Direction) -> bool {
        if self.in_lower_bound(position, direction) {
            let next_position = match direction {
                Direction::Right => Position::new(position.row, position.col + 1),
                Direction::Down => Position::new(position.row + 1, position.col),
                Direction::Left => Position::new(position.row, position.col - 1),
                Direction::Up => Position::new(position.row - 1, position.col),
            };
            self.in_upper_bound(&next_position)
        } else {
            false
        }
    }

    fn in_upper_bound(&self, position: &Position) -> bool {
        position.row < self.boundary.row && position.col < self.boundary.col
    }

    fn in_lower_bound(&self, position: &Position, direction: &Direction) -> bool {
        !(position.row == 0 && direction == &Direction::Up
        || position.col == 0 && direction == &Direction::Left)
    }
}

#[derive(Debug)]
pub struct TraversedMap {
    visited: HashSet<Position>,
    directed_cycle: DirectedCycle,
}

enum NextPosition {
    DirectedCycle(DirectedCycle),
    Position,
}

impl TraversedMap {
    pub fn from_map(map: &Map, new_obstacle: &Option<Position>) -> Self {
        let mut guard: GuardPosition = map.start_guard.clone();
        let mut visited: HashSet<Position> = HashSet::new();
        let mut directed_cycle = DirectedCycle::No;
        let mut directed_edges: HashSet<(Position, Position)> = HashSet::new();
        let mut status = TraverseStatus::Incomplete;

        let contains_obstacle = |position: &Position| -> bool {
            map.obstacles.contains(position)
            || if let Some(new_obstacle) = new_obstacle {
                new_obstacle == position
            } else {
                false
            }
        };

        let get_next_position = |curr_p: &Position, d: &mut Direction| -> NextPosition {
            let mut next_p: Option<Position> = None;

            let calc_next_position = |curr_p: &Position, d: &Direction| -> Position {
                match d {
                    Direction::Right => Position::new(curr_p.row, curr_p.col + 1),
                    Direction::Down => Position::new(curr_p.row + 1, curr_p.col),
                    Direction::Left => Position::new(curr_p.row, curr_p.col - 1),
                    Direction::Up => Position::new(curr_p.row - 1, curr_p.col),
                }
            };

            if !map.next_position_valid(&curr_p, &d) {
                println!("Guard is out of bounds. Complete");
                return NextPosition::DirectedCycle(DirectedCycle::No);
            }

            let mut obs_cnt: usize = 0;
            while next_p.is_none() && obs_cnt < 3 {
                if map.in_lower_bound(curr_p, &d) {
                    let potential_next_p = calc_next_position(curr_p, &d);
                    if map.in_upper_bound(&potential_next_p) {
                        // test for obstacle, if obstacle, turn right and update cnt
                        if contains_obstacle(&potential_next_p) {
                            obs_cnt += 1;
                            d.turn_right();
                        } else {
                            next_p = Some(potential_next_p);
                        }
                    }
                }
            }

            if obs_cnt == 3 && !next_p.is_none() {
                println!("Directed Cycle Detected as guard is in a standstill.");
                *dc = DirectedCycle::Yes;
            }
            next_p
        };

        while status == TraverseStatus::Incomplete {
            dbg!("Current Guard: {:#?}", guard);
            let next_p = get_next_position(&guard.position, &mut guard.direction, &mut directed_cycle);
            if let Some(next_p) = next_p {
                if directed_edges.contains(&(guard.position, next_p)) {
                    println!("Directed Cycle Detected.");
                    directed_cycle = DirectedCycle::Yes;
                    status = TraverseStatus::Complete;
                } else {
                    directed_edges.insert((guard.position, next_p));
                    visited.insert(next_p);
                    guard.position = next_p;
                }
            } else {
                dbg!("No Next Position. Complete.");
                status = TraverseStatus::Complete;
            }
        }

        Self {
            visited,
            directed_cycle,
        }
    }

    fn unique_visits(&self) -> usize {
        self.visited.len() as usize
    }
}

pub fn get_answer_p1(filepath: &path::Path) -> usize {
    let input = fs::read_to_string(filepath).expect("Failed to read file to string.");
    let map = Map::create_from_string(&input);
    let traversed_map = TraversedMap::from_map(&map, &None);
    traversed_map.unique_visits()
}

pub fn get_answer_p2(filepath: &path::Path) -> usize {
    let input = fs::read_to_string(filepath).expect("Failed to read file to string.");
    let map = Map::create_from_string(&input);
    let traversed_map = TraversedMap::from_map(&map, &None);
    let mut cycle_cnt: usize = 0;
    traversed_map.visited.into_iter().for_each(|p| {
        if TraversedMap::from_map(&map, &Some(p)).directed_cycle == DirectedCycle::Yes {
            cycle_cnt += 1;
        }
    });
    cycle_cnt
}

fn main() {
    let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day06.txt");
    println!("Day06 P1: {}", get_answer_p1(&filepath));
    println!("Day06 P2: {}", get_answer_p2(&filepath));
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
        assert_eq!(6, get_answer_p2(&filepath));
    }
}
