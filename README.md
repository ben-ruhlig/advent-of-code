# advent-of-code
all code for https://adventofcode.com/

Good Rust Coder to Compare: https://github.com/AhmedYassineMaalej/AoC-2024/

2024 Challenge - What did I learn each day:
- Day01
    - [Part1](https://adventofcode.com/2024/day/1#part1): Sort & Compare two vectors with vector methods `iter`, `zip`, `map`
    - [Part2](https://adventofcode.com/2024/day/1#part2): Compare two vectors, result in third. Same as above.
- Day02
    - [Part1](https://adventofcode.com/2024/day/2#part1): Iterate through contiguous slices of vector with method `windows`
    - [Part2](https://adventofcode.com/2024/day/2#part2): Dealing with `Option` types, and `enumerate` with vectors. Complex Logic. **Could this be done in a more rust friendly?**
- Day03
    - [Part1](https://adventofcode.com/2024/day/3#part1): Using `regex`lib from crate to match groups in strings, then process.
    - [Part2](https://adventofcode.com/2024/day/3#part2): Not Complete. **Come back to this**
- Day04
    - [Part1](https://adventofcode.com/2024/day/4#part1): Focus on control flow logic
    - [Part2](https://adventofcode.com/2024/day/4#part2): Not Complete. **Come back to this**
- Day05:
    - [Part1](https://adventofcode.com/2024/day/5#part1): Learn the brilliance of `include_str!` macro in std lib. Joys of `.split_once()` for a string.
- Day06:
    - [Part1](https://adventofcode.com/2024/day/6#part1): Should have used [`check-sub()`](https://doc.rust-lang.org/std/primitive.usize.html#method.checked_sub) method on primitive types to check for overflow when using the usize grid rather than do a manual check. 