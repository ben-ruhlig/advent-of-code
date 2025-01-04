# advent-of-code
all code for https://adventofcode.com/

To run specific problems (can list multiple):
```
cargo run day1p1 day1p2
```

To run tests:
```
cargo test --lib               // all tests
cargo test --lib day1          // all day 1 tests
cargo test --lib day1p1        // day 1 part 1 tests
```

To run benchmarks:
```
cargo bench -- day1p1          // run single benchmark
cargo bench -- day1            // run all day1 benchmarks
cargo bench                    // run all benchmarks
```
