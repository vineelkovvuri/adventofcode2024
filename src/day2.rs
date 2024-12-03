use std::fs;

fn is_safe(nums: &Vec<u32>) -> bool {
    nums.windows(2)
        .all(|ele| ele[0] < ele[1] && ele[1] - ele[0] <= 3)
        || nums
            .windows(2)
            .all(|ele| ele[0] > ele[1] && ele[0] - ele[1] <= 3)
}

fn is_safe2(nums: &Vec<u32>) -> bool {
    if is_safe(nums) {
        return true;
    }

    for k in 0..nums.len() {
        let nums2 = nums
            .iter()
            .enumerate()
            .filter(|(index, _)| k != *index)
            .map(|(_, &val)| val)
            .collect::<Vec<u32>>();
        if is_safe(&nums2) {
            return true;
        }
    }
    false
}

fn main() {
    let Ok(content) = fs::read_to_string("C:\\repos\\adventofcode2024\\input.day2.txt") else {
        return;
    };

    // Part 1
    let count = content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|line| is_safe(line))
        .count();
    println!("count = {}", count); // 314

    // Part 2
    let count = content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|line| is_safe2(line))
        .count();
    println!("count2 = {}", count); // 314
}
