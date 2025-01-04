use std::collections::HashSet;
use std::{fs, path};

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
    fn new(row: usize, col: usize) -> Result<Self, &'static str> {
        Ok(Self { row, col })
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
    boundary: Boundary,
    guard: GuardPosition,
    obstacles: HashSet<Position>,
    visited: HashSet<Position>,
    status: MapStatus,
}

impl Map {
    pub fn new(map_str: &str) -> Self {
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
        let mut row: usize = 0;
        let mut col: usize = 0;

        for map_str in map_str.lines() {
            col = 0;
            map_str.chars().for_each(|c| {
                if c == '#' {
                    obstacles.insert(Position::new(row, col).unwrap());
                } else if c == '>' || c == 'v' || c == '<' || c == '^' {
                    guard = Some(GuardPosition::new(row, col, dir_extract(c)));
                    visited.insert(Position::new(row, col).unwrap());
                }
                col += 1;
            });
            row += 1;
        }

        // default map status is incomplete
        let mut status = MapStatus::Incomplete;

        // Helper functions
        let inbounds = |(r, c): (usize, usize)| -> bool { r < row && c < col };
        let contains_obstacle =
            |(r, c): (usize, usize)| -> bool { obstacles.contains(&Position::new(r, c).unwrap()) };

        // If there's no guard, panic
        if guard.is_none() {
            panic!("Guard not found.");
        // If there's a guard, check if it's surrounded by obstacles. If it is, make sure the map status is complete.
        } else {
            let g = guard.unwrap();
            if inbounds((g.position.row, g.position.col + 1))
                && contains_obstacle((g.position.row, g.position.col + 1))
                && inbounds((g.position.row + 1, g.position.col))
                && contains_obstacle((g.position.row + 1, g.position.col))
                && inbounds((g.position.row, g.position.col - 1))
                && contains_obstacle((g.position.row, g.position.col - 1))
                && inbounds((g.position.row - 1, g.position.col))
                && contains_obstacle((g.position.row - 1, g.position.col))
            {
                status = MapStatus::Complete;
            }
        }

        // Return the map
        Self {
            boundary: Boundary { row, col },
            guard: guard.unwrap(),
            obstacles,
            visited,
            status,
        }
    }

    fn unique_visits(&self) -> usize {
        self.visited.len() as usize
    }

    fn exceeds_upper_bounds(&self, position: &Position) -> bool {
        // println!("Is In Bounds Boundary\n{:#?}\nPosition\n{:#?}", self.boundary, position);
        position.row > self.boundary.row || position.col > self.boundary.col
    }

    fn visited(&mut self, position: Position) {
        self.visited.insert(position);
    }

    fn complete(&mut self) {
        self.status = MapStatus::Complete;
    }

    fn next_position(&mut self) {
        let p = self.guard.position;

        // if guard is at lower boundary and heading off map, complete early
        if p.col == 0 && self.guard.direction == Direction::Left {
            self.complete();
            return ();
        } else if { p.row == 0 && self.guard.direction == Direction::Up } {
            self.complete();
            return ();
        }

        let new_p = match self.guard.direction {
            Direction::Right => Position::new(p.row, p.col + 1),
            Direction::Down => Position::new(p.row + 1, p.col),
            Direction::Left => Position::new(p.row, p.col - 1),
            Direction::Up => Position::new(p.row - 1, p.col),
        };

        if let Ok(new_position) = new_p {
            if self.exceeds_upper_bounds(&new_position) {
                self.complete();
            } else if self.obstacles.contains(&new_position) {
                self.guard.turn_right();
                self.next_position();
            } else {
                self.guard.position = new_position.clone();
                self.visited(new_position);
            }
        } else {
            self.complete();
        }
    }

    fn print_map(&self, start_guard: GuardPosition) {
        for row in 0..self.boundary.row {
            for col in 0..self.boundary.col {
                let p = Position::new(row, col).unwrap();
                if self.guard.position == p {
                    if self.guard.direction == Direction::Right {
                        print!(">");
                    } else if self.guard.direction == Direction::Down {
                        print!("v");
                    } else if self.guard.direction == Direction::Left {
                        print!("<");
                    } else if self.guard.direction == Direction::Up {
                        print!("^");
                    }
                } else if start_guard.position == p {
                    if start_guard.direction == Direction::Right {
                        print!(">");
                    } else if start_guard.direction == Direction::Down {
                        print!("v");
                    } else if start_guard.direction == Direction::Left {
                        print!("<");
                    } else if start_guard.direction == Direction::Up {
                        print!("^");
                    }
                } else if self.obstacles.contains(&p) {
                    print!("#");
                } else if self.visited.contains(&p) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub fn get_answer_p1(filepath: &path::Path) -> usize {
    let input = fs::read_to_string(filepath).unwrap();
    let mut map = Map::new(&input);
    while map.status == MapStatus::Incomplete {
        map.next_position();
        // println!("Next Position\n{:#?}", map.guard);
    }
    // println!("Visited\n{:#?}", map.visited);
    // map.print_map(start_guard);
    map.unique_visits()
}

#[allow(unused_variables)]
pub fn get_answer_p2(filepath: &path::Path) -> usize {
    let input = fs::read_to_string(filepath).expect("Failed to read file to string.");
    1
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
    // #[test]
    // fn day06_p2() {
    //     let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day06_test.txt");
    //     assert_eq!(2, get_answer_p2(&filepath));
    // }
}
