use std::fs;

use regex::Regex;

fn main() {
    let Ok(content) = fs::read_to_string("C:\\repos\\adventofcode2024\\input.day3.txt") else {
        return;
    };

    // Part 1
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let res = re
        .captures_iter(content.as_str())
        .map(|capture| {
            capture
                .get(1)
                .map(|mat| mat.as_str().parse::<u32>().unwrap())
                .unwrap_or(0)
                * capture
                    .get(2)
                    .map(|mat| mat.as_str().parse::<u32>().unwrap())
                    .unwrap_or(0)
        })
        .sum::<u32>();
    println!("{:?}", res); // 183380722

    // Part 2
    let re = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap(); // remove all don't - do. Make sure include new lines
    let content = re.replace_all(&content, "do()");
    let re = Regex::new(r"(?s)don't\(\).*").unwrap(); // remove ending don't - end. Make sure include new lines
    let content = re.replace_all(&content, "");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum = re
        .captures_iter(&content)
        .map(|capture| {
            capture
                .get(1)
                .map(|mat| mat.as_str().parse::<u32>().unwrap())
                .unwrap_or(0)
                * capture
                    .get(2)
                    .map(|mat| mat.as_str().parse::<u32>().unwrap())
                    .unwrap_or(0)
        })
        .sum::<u32>();

    println!("{:?}", sum); // 82733683
}
