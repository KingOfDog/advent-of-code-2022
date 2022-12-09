use std::{collections::HashSet, fmt::Error, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<Move>;

#[derive(Debug, Clone, Copy)]
struct Move {
    direction: Direction,
    steps: u32,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, steps) = s.split_once(" ").unwrap();
        Ok(Move {
            direction: direction.parse()?,
            steps: steps.parse().unwrap(),
        })
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            _ => Err(Error),
        }
    }
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        }
    }
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Parsed {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug, Clone, Copy)]
struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: (0, 0),
            tail: (0, 0),
        }
    }

    fn apply_move(&mut self, movement: Move) -> HashSet<(i32, i32)> {
        let mut visited = HashSet::new();
        for _ in 0..movement.steps {
            self.move_in_dir(movement.direction);
            visited.insert(self.tail);
        }
        visited
    }

    fn move_in_dir(&mut self, direction: Direction) {
        let offset = direction.offset();
        self.head = (self.head.0 + offset.0, self.head.1 + offset.1);

        self.update_tail();
        // println!("{:?}, {:?}", self.head, self.tail);
    }

    fn update_tail(&mut self) {
        if self.distance() > 1 {
            if self.head.0 < self.tail.0 {
                self.tail.0 -= 1;
            } else if self.head.0 > self.tail.0 {
                self.tail.0 += 1;
            }
            if self.head.1 < self.tail.1 {
                self.tail.1 -= 1;
            } else if self.head.1 > self.tail.1 {
                self.tail.1 += 1;
            }
        }
    }

    fn distance(&self) -> i32 {
        (self.head.0 - self.tail.0)
            .abs()
            .max((self.head.1 - self.tail.1).abs())
    }
}

#[aoc(day9, part1)]
fn part1(input: &Parsed) -> usize {
    let mut visited = HashSet::new();
    let mut rope = Rope::new();
    input.iter().for_each(|movement| {
        let new_visited = rope.apply_move(*movement);
        visited.extend(new_visited);
    });
    visited.len()
}

#[aoc(day9, part2)]
fn part2(input: &Parsed) -> usize {
    let mut visited = HashSet::new();
    let mut ropes = [Rope::new(); 9];
    input.iter().for_each(|movement| {
        for _ in 0..movement.steps {
            ropes[0].move_in_dir(movement.direction);
            for i in 1..ropes.len() {
                ropes[i].head = ropes[i - 1].tail;
                ropes[i].update_tail();
            }
            visited.insert(ropes[8].tail);
        }

        // for y in -10..16 {
        //     for x in -13..13 {
        //         if ropes[0].head == (x, y) {
        //             print!("H");
        //         } else {
        //             let rope = ropes.iter().skip(1).find(|r| r.tail == (x, y));
        //             if let Some(rope) = rope {
        //                 print!("T");
        //             } else {
        //                 print!(".");
        //             }
        //         }
        //     }
        //     println!("");
        // }
        // println!("");
    });
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 13);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 1);
        assert_eq!(
            part2(&parse_input(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            )),
            36
        )
    }
}
