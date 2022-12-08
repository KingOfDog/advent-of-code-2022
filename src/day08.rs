use std::slice::Iter;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct Parsed(Vec<Vec<u8>>);

impl Parsed {
    fn width(&self) -> usize {
        self.0.first().unwrap().len()
    }
    fn height(&self) -> usize {
        self.0.len()
    }

    fn row(&self, y: usize) -> Iter<u8> {
        self.0[y].iter()
    }
    fn col(&self, x: usize) -> ParsedColumnIter {
        ParsedColumnIter {
            parsed: self.to_owned(),
            column: x,
            index: 0,
        }
    }

    fn visible(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x == self.width() - 1 || y == self.height() - 1 {
            return true;
        }

        let value = self.0[y][x];

        self.row(y).take(x).all(|i| *i < value)
            || self.row(y).skip(x + 1).all(|i| *i < value)
            || self.col(x).take(y).all(|i| i < value)
            || self.col(x).skip(y + 1).all(|i| i < value)
    }

    fn num_visible(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(move |(x, _)| self.visible(*x, y))
                    .collect_vec()
            })
            .count()
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let value = self.0[y][x];
        let left = self
            .row(y)
            .take(x)
            .rev()
            .take_while(|i| **i < value)
            .count();
        let right = self
            .row(y)
            .skip(x + 1)
            .position(|i| *i >= value)
            .map_or(self.width() - x - 1, |v| v + 1);
        let top = self
            .col(x)
            .take(y)
            .rev()
            .position(|i| i >= value)
            .map_or(y, |v| v + 1);
        let bottom = self.col(x).skip(y + 1).take_while(|i| *i < value).count();

        left * right * top * bottom
    }
}

struct ParsedColumnIter<'a> {
    parsed: &'a Parsed,
    column: usize,
    index: usize,
}

impl Iterator for ParsedColumnIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.parsed.0.len() {
            return None;
        }

        let result = self.parsed.0[self.index][self.column];
        self.index += 1;

        Some(result)
    }
}

impl ExactSizeIterator for ParsedColumnIter<'_> {
    fn len(&self) -> usize {
        self.parsed.0.len() - self.index
    }
}

impl DoubleEndedIterator for ParsedColumnIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index >= self.parsed.0.len() {
            return None;
        }

        let result = self.parsed.0[self.parsed.0.len() - self.index - 1][self.column];
        self.index += 1;

        Some(result)
    }
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Parsed {
    Parsed(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec()
            })
            .collect_vec(),
    )
}

#[aoc(day8, part1)]
fn part1(input: &Parsed) -> usize {
    input.num_visible()
}

#[aoc(day8, part2)]
fn part2(input: &Parsed) -> usize {
    input
        .0
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| input.scenic_score(x, y))
                .collect_vec()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "30373
25512
65332
33549
35390"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 21);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 8);
    }
}
