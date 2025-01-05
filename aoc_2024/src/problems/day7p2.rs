use std::fs;

fn find_target(target: &u64, nums: &[u64]) -> bool {
    fn recurse(index: usize, curr: u64, target: &u64, nums: &[u64]) -> bool {
        // at the end of the list, check if the current value is the target
        if index == nums.len() {
            return curr == *target;
        }
        // addition case
        let next = curr.checked_add(nums[index]);
        if let Some(next) = next {
            if next <= *target && recurse(index + 1, next, target, nums) {
                return true;
            }
        }
        // multiplication case
        let next = curr.checked_mul(nums[index]);
        if let Some(next) = next {
            if next <= *target && recurse(index + 1, next, target, nums) {
                return true;
            }
        }
        // concatenation case
        let next_str = curr.to_string() + &nums[index].to_string();
        let next = next_str.parse::<u64>().ok();
        if let Some(next) = next {
            if next <= *target && recurse(index + 1, next, target, nums) {
                return true;
            }
        }
        false
    }
    if nums.is_empty() {
        return false;
    }
    recurse(1, nums[0], target, nums)
}

pub fn solution(file_path: &str) -> u64 {
    let mut accum = 0;
    let input = fs::read_to_string(file_path).unwrap();
    input.lines().for_each(|line| {
        let (target, nums) = line.split_once(':').unwrap();
        let target = target.parse::<u64>().unwrap();
        let nums = nums.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        
        if find_target(&target, &nums) {
            accum += target;
        }
    });
    accum
}
