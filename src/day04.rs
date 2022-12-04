use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Parsed = Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .map(|line| {
            let limits = line
                .split(",")
                .flat_map(|side| side.split("-"))
                .map(|number| number.parse().unwrap())
                .collect_tuple::<(u32, u32, u32, u32)>()
                .unwrap();
            (limits.0..=limits.1, limits.2..=limits.3)
        })
        .collect()
}

trait FullyContains<T> {
    fn fully_contains(&self, other: &T) -> bool;
    fn overlaps(&self, other: &T) -> bool;
}

impl FullyContains<RangeInclusive<u32>> for RangeInclusive<u32> {
    fn fully_contains(&self, other: &RangeInclusive<u32>) -> bool {
        self.contains(&other.start()) && self.contains(&other.end())
    }

    fn overlaps(&self, other: &RangeInclusive<u32>) -> bool {
        self.contains(&other.start())
            || self.contains(&other.end())
            || other.contains(&self.start())
            || other.contains(&self.end())
    }
}

#[aoc(day4, part1)]
fn part1(input: &Parsed) -> usize {
    input
        .iter()
        .filter(|pair| pair.0.fully_contains(&pair.1) || pair.1.fully_contains(&pair.0))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Parsed) -> usize {
    input.iter().filter(|pair| pair.0.overlaps(&pair.1)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 4);
    }
}
