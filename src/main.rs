use itertools::Itertools;

fn main() {
    let input: &str = include_str!("inputs/day02.txt");
    let lines: Vec<&str> = input.lines().collect();

    let parsed: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let part1 = part1(&parsed);
    println!("Part 1: {}", part1);

    let part2 = part2(&parsed);
    println!("Part 2: {}", part2);
}

fn part1(parsed: &Vec<Vec<u32>>) -> usize {
    parsed.iter().filter(|report| is_fully_safe(report)).count()
}

fn part2(parsed: &Vec<Vec<u32>>) -> usize {
    parsed
        .iter()
        .filter(|report| is_mostly_safe(report))
        .count()
}

fn is_fully_safe(report: &Vec<u32>) -> bool {
    is_monotonic(report) && all_levels_similar(report)
}

fn is_mostly_safe(report: &Vec<u32>) -> bool {
    if is_fully_safe(report) {
        return true;
    }
    for idx in 0..report.len() {
        let mut new = (*report).clone();
        new.remove(idx);
        if is_fully_safe(&new) {
            return true;
        }
    }
    false
}

fn is_monotonic(report: &Vec<u32>) -> bool {
    report.is_sorted() || report.is_sorted_by_key(std::cmp::Reverse)
}

fn all_levels_similar(report: &Vec<u32>) -> bool {
    report
        .into_iter()
        .tuple_windows::<(&u32, &u32)>()
        .all(|(a, b)| (1..=3).contains(&a.abs_diff(*b)))
}
