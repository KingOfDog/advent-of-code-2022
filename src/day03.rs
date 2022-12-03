use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<String>;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Parsed {
    input.lines().map(|line| String::from(line)).collect()
}

// Did only work for part 1 :(
// fn common_item((left, right): &(String, String)) -> char {
//     let left = left.chars().collect::<HashSet<char>>();
//     let right = right.chars().collect::<HashSet<char>>();
//     return left.intersection(&right).next().unwrap().to_owned();
// }

fn common_item(item_lists: Vec<String>) -> char {
    item_lists
        .iter()
        .map(|list| list.chars().collect::<HashSet<char>>())
        .reduce(|acc, list| {
            acc.intersection(&list)
                .map(|char| char.clone())
                .collect::<HashSet<char>>()
        })
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .to_owned()
}

fn item_priority(item: char) -> usize {
    let ascii = item as u8;
    if ascii >= 97 {
        (ascii - 96) as usize
    } else {
        (ascii - 64 + 26) as usize
    }
}

#[aoc(day3, part1)]
fn part1(input: &Parsed) -> usize {
    input
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|str| (str.0.to_string(), str.1.to_string()))
        .map(|rucksack| item_priority(common_item(vec![rucksack.0.clone(), rucksack.1.clone()])))
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Parsed) -> usize {
    input
        .chunks(3)
        .map(|chunk| item_priority(common_item(chunk.to_vec())))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 157);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 70);
    }
}
