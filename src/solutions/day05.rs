use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

fn main() {
    let input: &str = include_str!("inputs/day05.txt");
    let parsed: Vec<&str> = input.split("\n\n").collect();

    let mut rules: HashSet<(&str, &str)> = HashSet::new();
    parsed[0].lines().into_iter().for_each(|rule| {
        rules.insert(rule.split("|").collect_tuple().unwrap());
    });
    let updates: Vec<&str> = parsed[1].lines().collect();

    let part1 = part1(&rules, &updates);
    println!("Part 1: {}", part1);

    let part2 = part2(&rules, &updates);
    println!("Part 2: {}", part2);
}

fn part1(rules: &HashSet<(&str, &str)>, updates: &Vec<&str>) -> usize {
    updates
        .iter()
        .map(|update| {
            let pages: Vec<&str> = update.split(",").collect();
            if pages.is_sorted_by(|a, b| !rules.contains(&(*b, *a))) {
                pages[pages.len() / 2].parse().unwrap()
            } else {
                0
            }
        })
        .sum()
}

fn part2(rules: &HashSet<(&str, &str)>, updates: &Vec<&str>) -> usize {
    updates
        .iter()
        .map(|update| {
            let mut pages: Vec<&str> = update.split(",").collect();
            if pages.is_sorted_by(|a, b| !rules.contains(&(*b, *a))) {
                0
            } else {
                pages.sort_by(|a, b| {
                    if rules.contains(&(*b, *a)) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });
                pages[pages.len() / 2].parse().unwrap()
            }
        })
        .sum()
}
