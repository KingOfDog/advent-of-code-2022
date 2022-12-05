use std::collections::LinkedList;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

type Stack = LinkedList<char>;
type Parsed = (Vec<Stack>, Vec<Operation>);

struct Operation {
    from: usize,
    to: usize,
    count: usize,
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Parsed {
    let (stacks, ops) = input.split("\n\n").collect_tuple::<(&str, &str)>().unwrap();
    let mut lines = stacks.lines().collect_vec();
    let stack_count = lines
        .pop()
        .unwrap()
        .split("   ")
        .last()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut stacks: Vec<Stack> = vec![LinkedList::new(); stack_count];
    lines.iter().rev().for_each(|line| {
        line.chars()
            .chunks(4)
            .into_iter()
            .enumerate()
            .for_each(|(index, item)| {
                let item = item.collect::<Vec<char>>()[1];
                if item != ' ' {
                    stacks[index].push_back(item);
                }
            });
    });

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
    let operations = ops
        .lines()
        .map(|line| {
            let matches = re.captures(line).unwrap();
            Operation {
                count: matches[1].parse().unwrap(),
                from: matches[2].parse::<usize>().unwrap() - 1,
                to: matches[3].parse::<usize>().unwrap() - 1,
            }
        })
        .collect_vec();
    (stacks, operations)
}

#[aoc(day5, part1)]
fn part1(input: &Parsed) -> String {
    let mut stacks = input.0.to_owned();

    input.1.iter().for_each(|op| {
        for _ in 0..op.count {
            let item = stacks[op.from].pop_back().unwrap();
            stacks[op.to].push_back(item);
        }
    });

    stacks
        .iter()
        .map(|stack| stack.back().unwrap())
        .collect::<String>()
}

#[aoc(day5, part2)]
fn part2(input: &Parsed) -> String {
    let mut stacks = input.0.to_owned();

    input.1.iter().for_each(|op| {
        let index = stacks[op.from].len() - op.count;
        let mut split_off = stacks[op.from].split_off(index);
        stacks[op.to].append(&mut split_off);
    });

    stacks
        .iter()
        .map(|stack| stack.back().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), "CMZ");
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), "MCD");
    }
}
