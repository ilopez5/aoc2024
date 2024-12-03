use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("inputs/day01.txt");
    let lines: Vec<&str> = input.lines().collect();

    let parsed: Vec<(u32, u32)> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32)>()
                .unwrap()
        })
        .collect();

    let part1 = part1(parsed.clone());
    println!("Part 1: {}", part1);

    let part2 = part2(parsed);
    println!("Part 2: {}", part2);
}

fn part1(parsed: Vec<(u32, u32)>) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = parsed.into_iter().unzip();
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter_mut())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn part2(parsed: Vec<(u32, u32)>) -> u32 {
    let mut map = HashMap::<u32, u32>::new();
    let (left, right): (Vec<u32>, Vec<u32>) = parsed.into_iter().unzip();
    right.iter().for_each(|e| *map.entry(*e).or_insert(0) += 1);
    left.iter().map(|n| *n * map.get(n).unwrap_or(&0)).sum()
}
