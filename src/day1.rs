use std::fs;

fn main() {
    let Ok(content) = fs::read_to_string("C:\\repos\\adventofcode2024\\input.day1.txt") else {
        return;
    };

    // Part 1
    let mut list1 = vec![];
    let mut list2 = vec![];
    content.lines().for_each(|line| {
        let mut splits = line.split("   ");
        list1.push(splits.next().unwrap().parse::<i32>().unwrap());
        list2.push(splits.next().unwrap().parse::<i32>().unwrap());
    });

    list1.sort();
    list2.sort();

    let distance = list1
        .iter()
        .zip(&list2)
        .map(|ele| (ele.0 - ele.1).abs())
        .sum::<i32>();

    println!("Total distance: {}", distance); // 2580760

    // Part 2
    let similarity_score = list1
        .iter()
        .map(|&ele| list2.iter().filter(|&&key| key == ele).count() as i32 * ele)
        .sum::<i32>();
    println!("similarity_score = {}", similarity_score); // 25358365
}
