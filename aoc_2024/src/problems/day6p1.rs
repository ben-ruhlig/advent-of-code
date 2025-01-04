use std::collections::HashSet;
use std::fs;

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
        } else if p.row == 0 && self.guard.direction == Direction::Up {
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
}

pub fn solution(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).unwrap();
    let mut map = Map::new(&input);
    while map.status == MapStatus::Incomplete {
        map.next_position();
    }
    map.unique_visits()
}
