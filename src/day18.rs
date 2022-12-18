use std::{collections::HashSet, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::connected_components;

type Parsed = Vec<Position>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn neighbors(&self) -> Vec<Position> {
        [
            (-1, 0, 0),
            (0, 1, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .iter()
        .map(|(ox, oy, oz)| Position {
            x: self.x + ox,
            y: self.y + oy,
            z: self.z + oz,
        })
        .collect()
    }
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Position { x, y, z })
    }
}

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Parsed {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day18, part1)]
fn part1(input: &Parsed) -> usize {
    input
        .iter()
        .map(|cube| {
            let neighbors = cube.neighbors();
            neighbors
                .iter()
                .filter(|cube| !input.contains(*cube))
                .count()
        })
        .sum()
}

#[aoc(day18, part2)]
fn part2(input: &Parsed) -> usize {
    let bubbles = find_bubbles(input);

    input
        .iter()
        .map(|cube| {
            let neighbors = cube.neighbors();
            neighbors
                .iter()
                .filter(|cube| !input.contains(*cube))
                .filter(|cube| !bubbles.iter().any(|b| b.contains(*cube)))
                .count()
        })
        .sum()
}

fn find_bubbles(input: &Parsed) -> Vec<HashSet<Position>> {
    let max_x = input.iter().max_by_key(|p| p.x).unwrap().x + 1;
    let max_y = input.iter().max_by_key(|p| p.y).unwrap().y + 1;
    let max_z = input.iter().max_by_key(|p| p.z).unwrap().z + 1;

    let steam_cubes = (0..=max_x)
        .flat_map(|x| (0..=max_y).flat_map(move |y| (0..=max_z).map(move |z| Position { x, y, z })))
        .filter(|cube| !input.contains(cube))
        .collect_vec();
    let mut bubbles = connected_components(&steam_cubes, |cube| {
        cube.neighbors()
            .into_iter()
            .filter(|neighbor| !input.contains(neighbor))
            .collect_vec()
    });
    let outer_steam = bubbles
        .iter()
        .position_max_by_key(|bubble| bubble.len())
        .unwrap();
    bubbles.remove(outer_steam);

    bubbles
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 64);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 58);
    }
}
