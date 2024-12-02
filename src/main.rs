use std::fs;

fn is_safe(line: &str) -> bool {
    let nums = line
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    nums.windows(2)
        .all(|ele| ele[0] < ele[1] && ele[1] - ele[0] <= 3)
        || nums
            .windows(2)
            .all(|ele| ele[0] > ele[1] && ele[0] - ele[1] <= 3)
}

fn is_safe2(line: &str) -> bool {
    let nums = line
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let increasing = nums
        .windows(2)
        .map(|ele| ele[0] - ele[1])
        .all(|ele| ele < 0 && ele >= -3);

    let decreasing = nums
        .windows(2)
        .map(|ele| ele[0] - ele[1])
        .all(|ele| ele > 0 && ele <= 3);

    let one_positive_or_zero = nums
        .windows(2)
        .map(|ele| ele[0] - ele[1])
        .filter(|&ele| ele < 0)
        .count()
        == nums.len() - 2;

    let one_negative_or_zero = nums
        .windows(2)
        .map(|ele| ele[0] - ele[1])
        .filter(|&ele| ele > 0)
        .count()
        == nums.len() - 2;

    increasing || decreasing || one_positive_or_zero || one_negative_or_zero
    // nums.windows(2)
    //     .all(|ele| ele[0] < ele[1] && ele[1] - ele[0] <= 3)
    //     || nums
    //         .windows(2)
    //         .all(|ele| ele[0] > ele[1] && ele[0] - ele[1] <= 3)
    //     || nums
    //         .windows(3)
    //         .filter(|ele| ele[0] < ele[1] && ele[1] > ele[2])
    //         .count()
    //         == 1
    //     || nums
    //         .windows(3)
    //         .filter(|ele| ele[0] > ele[1] && ele[1] < ele[2])
    //         .count()
    //         == 1
    //     || nums
    //         .windows(3)
    //         .filter(|ele| ele[0] == ele[1] && ele[1] > ele[2])
    //         .count()
    //         == 1
    //     || nums
    //         .windows(3)
    //         .filter(|ele| ele[0] == ele[1] && ele[1] < ele[2])
    //         .count()
    //         == 1
    //     || nums
    //         .windows(3)
    //         .filter(|ele| ele[0] < ele[1] && ele[1] == ele[2])
    //         .count()
    //         == 1
    //     || nums
    //         .windows(3)
    //         .filter(|ele| ele[0] > ele[1] && ele[1] == ele[2])
    //         .count()
    //         == 1
}

fn main() {
    let Ok(content) = fs::read_to_string("C:\\repos\\adventofcode2024\\input.day2.txt")
    else {
        return;
    };

    // Part 1
    let count = content.lines().filter(|line| is_safe(line)).count();
    println!("count = {}", count); // 314

    // Part 2
    let count = content.lines().filter(|line| is_safe2(line)).count();
    println!("count2 = {}", count); // 314
}
