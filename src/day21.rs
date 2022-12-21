use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = HashMap<String, Monkey>;

enum Monkey {
    Number(u64),
    Operation {
        left: String,
        right: String,
        operation: Operation,
    },
}

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        }
    }

    fn apply_backwards_left(&self, left: u64, result: u64) -> u64 {
        match self {
            Operation::Add => result - left,
            Operation::Sub => left - result,
            Operation::Mul => result / left,
            Operation::Div => left / result,
        }
    }

    fn apply_backwards_right(&self, right: u64, result: u64) -> u64 {
        match self {
            Operation::Add => result - right,
            Operation::Sub => result + right,
            Operation::Mul => result / right,
            Operation::Div => result * right,
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Sub),
            "*" => Ok(Operation::Mul),
            "/" => Ok(Operation::Div),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day21)]
fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .map(|line| {
            let (name, action) = line.split_once(": ").unwrap();
            let action_parts = action.split(" ").collect::<Vec<_>>();

            let action = if action_parts.len() == 1 {
                Monkey::Number(action_parts[0].parse().unwrap())
            } else {
                Monkey::Operation {
                    left: action_parts[0].to_string(),
                    right: action_parts[2].to_string(),
                    operation: action_parts[1].parse().unwrap(),
                }
            };

            (name.to_string(), action)
        })
        .collect()
}

fn solve(monkeys: &Parsed, current_monkey: &String) -> u64 {
    let monkey = &monkeys[current_monkey];
    match monkey {
        Monkey::Number(x) => *x,
        Monkey::Operation {
            left,
            right,
            operation,
        } => {
            let left = solve(monkeys, &left);
            let right = solve(monkeys, &right);
            operation.apply(left, right)
        }
    }
}

fn try_solve(monkeys: &Parsed, current_monkey: &String) -> Result<u64, ()> {
    if current_monkey == "humn" {
        return Err(());
    }
    let monkey = &monkeys[current_monkey];
    match monkey {
        Monkey::Number(x) => Ok(*x),
        Monkey::Operation {
            left,
            right,
            operation,
        } => {
            let left = try_solve(monkeys, &left)?;
            let right = try_solve(monkeys, &right)?;
            Ok(operation.apply(left, right))
        }
    }
}

fn back_propagate(monkeys: &Parsed, current_monkey: &String, required_result: u64) -> Option<u64> {
    if current_monkey == "humn" {
        return Some(required_result);
    }

    let monkey = &monkeys[current_monkey];
    match monkey {
        Monkey::Number(_) => None,
        Monkey::Operation {
            left: left_monkey,
            right: right_monkey,
            operation,
        } => {
            let left = try_solve(monkeys, &left_monkey);
            let right = try_solve(monkeys, &right_monkey);

            if let Ok(left) = left {
                let required_result = if current_monkey == "root" {
                    left
                } else {
                    operation.apply_backwards_left(left, required_result)
                };
                back_propagate(monkeys, right_monkey, required_result)
            } else if let Ok(right) = right {
                let required_result = if current_monkey == "root" {
                    right
                } else {
                    operation.apply_backwards_right(right, required_result)
                };
                back_propagate(monkeys, left_monkey, required_result)
            } else {
                None
            }
        }
    }
}

#[aoc(day21, part1)]
fn part1(input: &Parsed) -> u64 {
    solve(input, &String::from("root"))
}

#[aoc(day21, part2)]
fn part2(input: &Parsed) -> u64 {
    back_propagate(input, &String::from("root"), 0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 152);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 301);
    }
}
