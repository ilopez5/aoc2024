use regex::Regex;

fn main() {
    let input: &str = include_str!("inputs/day03.txt");
    let lines: Vec<&str> = input.lines().collect();
    let program: String = lines.join("");

    let part1 = part1(program.clone());
    println!("Part 1: {}", part1);

    let part2 = part2(program);
    println!("Part 2: {}", part2);
}

fn part1(program: String) -> usize {
    multiply(program)
}

fn part2(program: String) -> usize {
    let (left, right) = program.split_once("don't()").unwrap();
    let cleaned = left.to_owned()
        + &right
            .split("don't()")
            .into_iter()
            .map(|s| s.split_once("do()").unwrap_or_default().1)
            .collect::<Vec<&str>>()
            .join("");
    multiply(cleaned)
}

fn multiply(program: String) -> usize {
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(&program)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
        })
        .sum::<usize>()
}
