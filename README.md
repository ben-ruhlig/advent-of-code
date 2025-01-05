# advent-of-code
all code for https://adventofcode.com/

Enter 2024 folder:
```
cd aoc_2024
```

Run solutions:
```
cargo run                      // Run all implemented solutions
cargo run -- -d 1           // Run both parts of day 1
cargo run -- -d 1 -p 1  // Run day 1, part 1
```

Run tests:
```
cargo test --lib               // all tests
cargo test --lib day1          // all day 1 tests
cargo test --lib day1p1        // day 1 part 1 tests
```

Run benchmarks:
```
cargo bench                    // run all benchmarks
cargo bench -- day1            // run all day 1 benchmarks
cargo bench -- day1p1          // run single benchmark for day 1 part 1
```

Download input (bash):
```
./get-input 7
```
