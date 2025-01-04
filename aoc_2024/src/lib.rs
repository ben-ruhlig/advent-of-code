pub mod problems;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1p1() {
        assert_eq!(11, problems::day1p1::solution("./input/day1_test.txt"));
    }

    #[test]
    fn day1p2() {
        assert_eq!(31, problems::day1p2::solution("./input/day1_test.txt"));
    }

    #[test]
    fn day2p1() {
        assert_eq!(2, problems::day2p1::solution("./input/day2_test.txt"));
    }

    #[test]
    fn day2p2() {
        assert_eq!(4, problems::day2p2::solution("./input/day2_test.txt"));
    }

    #[test]
    fn day3p1() {
        assert_eq!(161, problems::day3p1::solution("./input/day3_test.txt"));
    }

    #[test]
    fn day4p1() {
        assert_eq!(18, problems::day4p1::solution("./input/day4_test.txt"));
    }

    #[test]
    fn day5p1() {
        assert_eq!(143, problems::day5p1::solution("./input/day5_test.txt"));
    }

    #[test]
    fn day5p2() {
        assert_eq!(123, problems::day5p2::solution("./input/day5_test.txt"));
    }

    #[test]
    fn day6p1() {
        assert_eq!(42, problems::day6p1::solution("./input/day6_test.txt"));
    }
}
