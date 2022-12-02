use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{
    cmp::{Ordering, PartialOrd},
    str::FromStr,
};

type Parsed = Vec<(Shape, String)>;

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Error;

impl FromStr for Shape {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(Error),
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Shape) -> Option<Ordering> {
        match self {
            Shape::Rock => match other {
                Shape::Rock => Some(Ordering::Equal),
                Shape::Paper => Some(Ordering::Less),
                Shape::Scissors => Some(Ordering::Greater),
            },
            Shape::Paper => match other {
                Shape::Rock => Some(Ordering::Greater),
                Shape::Paper => Some(Ordering::Equal),
                Shape::Scissors => Some(Ordering::Less),
            },
            Shape::Scissors => match other {
                Shape::Rock => Some(Ordering::Less),
                Shape::Paper => Some(Ordering::Greater),
                Shape::Scissors => Some(Ordering::Equal),
            },
        }
    }
}

impl Shape {
    fn points(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn winning_shape(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn losing_shape(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            (parts[0].parse().unwrap(), parts[1].to_string())
        })
        .collect()
}

fn round_points(ordering: Ordering) -> usize {
    match ordering {
        Ordering::Less => 0,
        Ordering::Greater => 6,
        _ => 3,
    }
}

#[aoc(day2, part1)]
fn part1(input: &Parsed) -> usize {
    input
        .iter()
        .map(|row| (row.0, row.1.parse::<Shape>().unwrap()))
        .map(|round| round.1.points() + round_points(round.1.partial_cmp(&round.0).unwrap()))
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &Parsed) -> usize {
    input
        .iter()
        .map(|round| points_for_strategy(round.0, &round.1))
        .sum()
}

fn points_for_strategy(enemy_shape: Shape, str: &str) -> usize {
    match str {
        "X" => enemy_shape.losing_shape().points(),
        "Y" => 3 + enemy_shape.points(),
        "Z" => 6 + enemy_shape.winning_shape().points(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "A Y
B X
C Z"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 15);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 12);
    }
}
