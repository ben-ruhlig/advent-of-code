mod problems;
use clap::Parser;

#[derive(Parser)]
#[command(name = "aoc2024")]
#[command(about = "Advent of Code 2024 Solutions")]
struct Cli {
    /// Day number to run (1-25)
    #[arg(short, long)]
    day: Option<u8>,

    /// Part number to run (1-2)
    #[arg(short, long)]
    part: Option<u8>,
}

fn main() {
    let cli = Cli::parse();
    let mut selected_files: Vec<(u8, u8)> = vec![];

    match (cli.day, cli.part) {
        (Some(day), Some(part)) => {
            selected_files.push((day, part));
        }
        (Some(day), None) => {
            // If only day is specified, run both parts
            selected_files.push((day, 1));
            selected_files.push((day, 2));
        }
        (None, _) => {
            // Default behavior - run all implemented solutions
            selected_files.extend([
                (1, 1),
                (1, 2),
                (2, 1),
                (2, 2),
                (3, 1),
                (4, 1),
                (5, 1),
                (6, 1),
            ]);
        }
    }

    fn run_day(day: &u8, part: &u8) {
        let input_path = format!("./input/day{day}.txt");
        match (day, part) {
            (1u8, 1u8) => println!("Day1 P1: {}", problems::day1p1::solution(&input_path)),
            (1u8, 2u8) => println!("Day1 P2: {}", problems::day1p2::solution(&input_path)),
            (2u8, 1u8) => println!("Day2 P1: {}", problems::day2p1::solution(&input_path)),
            (2u8, 2u8) => println!("Day2 P2: {}", problems::day2p2::solution(&input_path)),
            (3u8, 1u8) => println!("Day3 P1: {}", problems::day3p1::solution(&input_path)),
            // (3u8, 2u8) => println!("Day3 P2: {}", problems::day3p2::solution(&input_path)),
            (4u8, 1u8) => println!("Day4 P1: {}", problems::day4p1::solution(&input_path)),
            // (4u8, 2u8) => println!("Day4 P2: {}", problems::day4p2::solution(&input_path)),
            (5u8, 1u8) => println!("Day5 P1: {}", problems::day5p1::solution(&input_path)),
            (5u8, 2u8) => println!("Day5 P2: {}", problems::day5p2::solution(&input_path)),
            (6u8, 1u8) => println!("Day6 P1: {}", problems::day6p1::solution(&input_path)),
            // (6u8, 2u8) => println!("Day6 P2: {}", problems::day6p2::solution(&input_path)),
            // (7u8, 1u8) => println!("Day7 P1: {}", problems::day7p1::solution(&input_path)),
            // (7u8, 2u8) => println!("Day7 P2: {}", problems::day7p2::solution(&input_path)),
            // (8u8, 1u8) => println!("Day8 P1: {}", problems::day8p1::solution(&input_path)),
            // (8u8, 2u8) => println!("Day8 P2: {}", problems::day8p2::solution(&input_path)),
            // (9u8, 1u8) => println!("Day9 P1: {}", problems::day9p1::solution(&input_path)),
            // (9u8, 2u8) => println!("Day9 P2: {}", problems::day9p2::solution(&input_path)),
            // (10u8, 1u8) => println!("Day10 P1: {}", problems::day10p1::solution(&input_path)),
            // (10u8, 2u8) => println!("Day10 P2: {}", problems::day10p2::solution(&input_path)),
            // (11u8, 1u8) => println!("Day11 P1: {}", problems::day11p1::solution(&input_path)),
            // (11u8, 2u8) => println!("Day11 P2: {}", problems::day11p2::solution(&input_path)),
            // (12u8, 1u8) => println!("Day12 P1: {}", problems::day12p1::solution(&input_path)),
            // (12u8, 2u8) => println!("Day12 P2: {}", problems::day12p2::solution(&input_path)),
            // (13u8, 1u8) => println!("Day13 P1: {}", problems::day13p1::solution(&input_path)),
            // (13u8, 2u8) => println!("Day13 P2: {}", problems::day13p2::solution(&input_path)),
            // (14u8, 1u8) => println!("Day14 P1: {}", problems::day14p1::solution(&input_path)),
            // (14u8, 2u8) => println!("Day14 P2: {}", problems::day14p2::solution(&input_path)),
            // (15u8, 1u8) => println!("Day15 P1: {}", problems::day15p1::solution(&input_path)),
            // (15u8, 2u8) => println!("Day15 P2: {}", problems::day15p2::solution(&input_path)),
            // (16u8, 1u8) => println!("Day16 P1: {}", problems::day16p1::solution(&input_path)),
            // (16u8, 2u8) => println!("Day16 P2: {}", problems::day16p2::solution(&input_path)),
            // (17u8, 1u8) => println!("Day17 P1: {}", problems::day17p1::solution(&input_path)),
            // (17u8, 2u8) => println!("Day17 P2: {}", problems::day17p2::solution(&input_path)),
            // (18u8, 1u8) => println!("Day18 P1: {}", problems::day18p1::solution(&input_path)),
            // (18u8, 2u8) => println!("Day18 P2: {}", problems::day18p2::solution(&input_path)),
            // (19u8, 1u8) => println!("Day19 P1: {}", problems::day19p1::solution(&input_path)),
            // (19u8, 2u8) => println!("Day19 P2: {}", problems::day19p2::solution(&input_path)),
            // (20u8, 1u8) => println!("Day20 P1: {}", problems::day20p1::solution(&input_path)),
            // (20u8, 2u8) => println!("Day20 P2: {}", problems::day20p2::solution(&input_path)),
            // (21u8, 1u8) => println!("Day21 P1: {}", problems::day21p1::solution(&input_path)),
            // (21u8, 2u8) => println!("Day21 P2: {}", problems::day21p2::solution(&input_path)),
            // (22u8, 1u8) => println!("Day22 P1: {}", problems::day22p1::solution(&input_path)),
            // (22u8, 2u8) => println!("Day22 P2: {}", problems::day22p2::solution(&input_path)),
            // (23u8, 1u8) => println!("Day23 P1: {}", problems::day23p1::solution(&input_path)),
            // (23u8, 2u8) => println!("Day23 P2: {}", problems::day23p2::solution(&input_path)),
            // (24u8, 1u8) => println!("Day24 P1: {}", problems::day24p1::solution(&input_path)),
            // (24u8, 2u8) => println!("Day24 P2: {}", problems::day24p2::solution(&input_path)),
            // (25u8, 1u8) => println!("Day25 P1: {}", problems::day25p1::solution(&input_path)),
            // (25u8, 2u8) => println!("Day25 P2: {}", problems::day25p2::solution(&input_path)),
            _ => println!("Day{day} P{part} NOT FOUND"),
        }
    }

    fn run_selected_days(selected_files: &[(u8, u8)]) {
        selected_files.iter().for_each(|(day, part)| {
            run_day(day, part);
        });
    }

    run_selected_days(&selected_files);
}
