use std::{collections::LinkedList, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Parsed = Vec<Monkey>;

#[derive(Debug, Clone)]
struct Monkey {
    items: LinkedList<u64>,
    operation: Operation,
    test_divisible: u64,
    test_success_target: usize,
    test_failed_target: usize,

    inspected_items: u64,
}

impl Monkey {
    fn throw_item(
        &mut self,
        divide_worry_before_test: bool,
        item_modulo: u64,
    ) -> Option<(u64, usize)> {
        let item = self.items.pop_front()?;
        let mut item = self.operation.apply(item);
        if divide_worry_before_test {
            item /= 3;
        }

        self.inspected_items += 1;

        if item % self.test_divisible == 0 {
            Some((item % item_modulo, self.test_success_target))
        } else {
            Some((item % item_modulo, self.test_failed_target))
        }
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .skip(1)
            .map(|l| {
                l.split_once(": ")
                    .expect(format!("{l} could not be parsed").as_str())
                    .1
            })
            .collect_vec();
        assert_eq!(lines.len(), 5);

        let items = lines[0].split(", ").map(|x| x.parse().unwrap()).collect();
        let operation = lines[1].parse()?;

        let test_divisible = lines[2].split(' ').last().unwrap().parse().unwrap();
        let test_success_target = lines[3].split(' ').last().unwrap().parse().unwrap();
        let test_failed_target = lines[4].split(' ').last().unwrap().parse().unwrap();

        Ok(Monkey {
            items,
            operation,
            test_divisible,
            test_success_target,
            test_failed_target,

            inspected_items: 0,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn apply(&self, x: u64) -> u64 {
        match self {
            Operation::Add(y) => x + y,
            Operation::Multiply(y) => x * y,
            Operation::Square => x * x,
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect_vec();
        assert_eq!(parts.len(), 5);
        match parts[3] {
            "+" => Ok(Operation::Add(parts[4].parse().unwrap())),
            "*" => match parts[4] {
                "old" => Ok(Operation::Square),
                _ => Ok(Operation::Multiply(parts[4].parse().unwrap())),
            },
            _ => Err(()),
        }
    }
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Parsed {
    input.split("\n\n").map(|b| b.parse().unwrap()).collect()
}

fn run(input: &Parsed, divide_worry_before_test: bool, rounds: usize) -> u64 {
    let mut monkeys = input.to_owned();
    let divisor_product = monkeys.iter().map(|monkey| monkey.test_divisible).product();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some((item, target)) =
                monkeys[i].throw_item(divide_worry_before_test, divisor_product)
            {
                monkeys[target].items.push_back(item);
            }
        }
    }

    monkeys
        .iter()
        .map(|monkey| monkey.inspected_items)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[aoc(day11, part1)]
fn part1(input: &Parsed) -> u64 {
    run(input, true, 20)
}

#[aoc(day11, part2)]
fn part2(input: &Parsed) -> u64 {
    run(input, false, 10000)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 10605);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 2713310158);
    }
}
