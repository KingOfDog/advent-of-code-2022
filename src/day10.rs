use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<Instruction>;

#[derive(Debug, PartialEq)]
enum Instruction {
    NoOp,
    Add(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("addx") {
            Ok(Instruction::Add(
                s.split_once(" ").unwrap().1.parse().unwrap(),
            ))
        } else {
            Ok(Instruction::NoOp)
        }
    }
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Parsed {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &Parsed) -> i32 {
    let mut iter = input
        .into_iter()
        .scan((0, 1), |(cycle, x), instruction| {
            match instruction {
                Instruction::NoOp => *cycle += 1,
                Instruction::Add(value) => {
                    *cycle += 2;
                    *x += value;
                }
            };
            Some((*cycle, *x))
        })
        .peekable();

    let mut last_x = 1;
    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|tap| -> i32 {
            while let Some((_, x)) = iter.peek().filter(|(cycle, _)| cycle < tap) {
                last_x = *x;
                iter.next();
            }
            tap * last_x
        })
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &Parsed) -> String {
    let mut iter = input
        .into_iter()
        .scan((0, 1), |(cycle, x), instruction| {
            match instruction {
                Instruction::NoOp => *cycle += 1,
                Instruction::Add(value) => {
                    *cycle += 2;
                    *x += value;
                }
            };
            Some((*cycle, *x))
        })
        .peekable();

    let mut last_x = 1;
    (0..240)
        .step_by(40)
        .map(|row| {
            (0..40)
                .map(|col| {
                    while let Some((_, x)) = iter.peek().filter(|(cycle, _)| cycle <= &(row + col))
                    {
                        last_x = *x;
                        iter.next();
                    }
                    if (-1..=1).contains(&(last_x - col)) {
                        '\u{2593}'
                    } else {
                        '\u{2591}'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(INPUT)), 13140);
    }

    #[test]
    fn real_input1() {
        let input = read_to_string("input/2022/day10.txt").unwrap();
        assert_eq!(part1(&parse_input(&input[..])), 14720);
    }

    #[test]
    fn sample2() {
        assert_eq!(
            part2(&parse_input(INPUT)),
            "▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░
▓▓▓░░░▓▓▓░░░▓▓▓░░░▓▓▓░░░▓▓▓░░░▓▓▓░░░▓▓▓░
▓▓▓▓░░░░▓▓▓▓░░░░▓▓▓▓░░░░▓▓▓▓░░░░▓▓▓▓░░░░
▓▓▓▓▓░░░░░▓▓▓▓▓░░░░░▓▓▓▓▓░░░░░▓▓▓▓▓░░░░░
▓▓▓▓▓▓░░░░░░▓▓▓▓▓▓░░░░░░▓▓▓▓▓▓░░░░░░▓▓▓▓
▓▓▓▓▓▓▓░░░░░░░▓▓▓▓▓▓▓░░░░░░░▓▓▓▓▓▓▓░░░░░"
        );
    }

    #[test]
    fn real_input2() {
        let input = read_to_string("input/2022/day10.txt").unwrap();
        assert_eq!(
            part2(&parse_input(&input[..])),
            "▓▓▓▓░▓▓▓▓░▓▓▓░░▓▓▓░░▓▓▓░░▓▓▓▓░▓▓▓▓░▓▓▓▓░
▓░░░░░░░▓░▓░░▓░▓░░▓░▓░░▓░▓░░░░░░░▓░▓░░░░
▓▓▓░░░░▓░░▓▓▓░░▓░░▓░▓▓▓░░▓▓▓░░░░▓░░▓▓▓░░
▓░░░░░▓░░░▓░░▓░▓▓▓░░▓░░▓░▓░░░░░▓░░░▓░░░░
▓░░░░▓░░░░▓░░▓░▓░░░░▓░░▓░▓░░░░▓░░░░▓░░░░
▓░░░░▓▓▓▓░▓▓▓░░▓░░░░▓▓▓░░▓░░░░▓▓▓▓░▓░░░░"
        );
    }
}
