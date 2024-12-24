use std::{fs, path,};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
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
        if row < 0 || col < 0 {
            return Err("Row and column must be greater than 0.");
        }
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

#[derive(Debug)]
pub struct Map {
    boundary: Boundary,
    guard: GuardPosition,
    obstacles: HashSet<Position>,
    visited: HashSet<Position>,
    complete: bool,
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
            map_str
                .chars()
                .for_each(|c| {
                    if c == '#' {
                        obstacles.insert(Position::new(row, col).unwrap());
                    } else if c == '>' || c == 'v' || c == '<' || c == '^' {
                        guard = Some(GuardPosition::new(row, col, dir_extract(c)));
                        visited.insert(Position::new(row, col).unwrap());
                    }
                    col += 1;
                });
            row += 1;
            col = 0;
        };

        Self {
            boundary: Boundary {row, col},
            guard: guard.unwrap(),
            obstacles,
            visited,
            complete: false,
        }
    }

    fn is_in_bounds(&self, position: Position) -> bool {
        self.boundary.row > position.row || self.boundary.col > position.col
    }

    fn visited(&mut self, position: Position) {
        self.visited.insert(position);
    }

    fn complete(&mut self) {
        self.complete = true;
    }

    fn advance_guard(&self) -> Result<(), &'static str> {
        let g: &GuardPosition = &self.guard;

        let test = |map: Map, test_position: Position| -> bool {
            map.is_in_bounds(test_position) && !map.obstacles.contains(&test_position)
        };

        let new_position = match g.direction {
            Direction::Right => Position::new(g.position.row, g.position.col + 1),
            Direction::Down => Position::new(g.position.row + 1, g.position.col),
            Direction::Left => Position::new(g.position.row, g.position.col - 1),
            Direction::Up => Position::new(g.position.row - 1, g.position.col),
        };

        // TODO: Implement remainder of logic

        Ok(())
    }

    pub fn move(&mut self) {
        let mut new_position = self.guard.position;
        match self.guard.direction {
            Direction::Right => new_position.col += 1,
            Direction::Down => new_position.row += 1,
            Direction::Left => new_position.col -= 1,
            Direction::Up => new_position.row -= 1,
        }

        if self.obstacles.contains(&new_position) {
            self.guard.turn_right();
        } else {
            self.guard.position = new_position;
            self.visited(new_position);
        }

        if self.visited.len() == self.boundary.row * self.boundary.col {
            self.complete();
        }

        Ok(())
    }
}


#[allow(unused_variables)]
pub fn get_answer_p1(filepath: &path::Path) -> usize{
    let input = fs::read_to_string(filepath).unwrap();
    let map = Map::new(&input);
    1
}

#[allow(unused_variables)]
pub fn get_answer_p2(filepath: &path::Path) -> usize {
    let input = fs::read_to_string(filepath).expect("Failed to read file to string.");
    1
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
        assert_eq!(41, get_answer_p1(&filepath));
    }
    #[test]
    fn day06_p2() {
        let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day06_test.txt");
        assert_eq!(2, get_answer_p2(&filepath));
    }
}
