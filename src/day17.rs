use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<Direction>;

lazy_static! {
    static ref ROCKS: Vec<Vec<Vec<bool>>> = vec![
        vec![vec![true; 4]],
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        vec![
            vec![false, false, true],
            vec![false, false, true],
            vec![true, true, true],
        ],
        vec![vec![true]; 4],
        vec![vec![true; 2]; 2],
    ];
}

// rocks:
//
// ####
const ROCK_1: [[bool; 4]; 4] = [
    [true, true, true, true],
    [false, false, false, false],
    [false, false, false, false],
    [false, false, false, false],
];
// .#.
// ###
// .#.
const ROCK_2: [[bool; 4]; 4] = [
    [false, true, false, false],
    [true, true, true, false],
    [false, true, false, false],
    [false, false, false, false],
];
// ..#
// ..#
// ###
const ROCK_3: [[bool; 4]; 4] = [
    [true, true, true, false],
    [false, false, true, false],
    [false, false, true, false],
    [false, false, false, false],
];
// #
// #
// #
// #
const ROCK_4: [[bool; 4]; 4] = [
    [true, false, false, false],
    [true, false, false, false],
    [true, false, false, false],
    [true, false, false, false],
];
// ##
// ##
const ROCK_5: [[bool; 4]; 4] = [
    [true, true, false, false],
    [true, true, false, false],
    [false, false, false, false],
    [false, false, false, false],
];
const ROCKS_2: [[[bool; 4]; 4]; 5] = [ROCK_1, ROCK_2, ROCK_3, ROCK_4, ROCK_5];

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Direction::Left),
            ">" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

type Field = Vec<[bool; 7]>;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Rock {
    pos: Pos,
    class: usize,
}

impl Rock {
    fn new(class: usize) -> Self {
        Rock {
            pos: Pos { x: 2, y: 0 },
            class,
        }
    }

    fn layout(&self) -> Vec<Vec<bool>> {
        ROCKS[self.class].clone()
    }

    fn width(&self) -> usize {
        self.layout()[0].len()
    }

    fn height(&self) -> usize {
        self.layout().len()
    }

    fn can_be_pushed(&self, field: &Field, direction: &Direction) -> bool {
        match direction {
            Direction::Left => {
                self.pos.x >= 1 && self.can_move_to((self.pos.x - 1, self.pos.y), field)
            }
            Direction::Right => {
                self.pos.x + self.width() < 7
                    && self.can_move_to((self.pos.x + 1, self.pos.y), field)
            }
        }
    }

    fn push(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.pos.x -= 1,
            Direction::Right => self.pos.x += 1,
        }
    }

    fn can_move_to(&self, (new_x, new_y): (usize, usize), field: &Field) -> bool {
        self.layout().iter().enumerate().all(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, i)| **i)
                .all(|(x, _)| field[new_y - y][new_x + x] != true)
        })
    }

    fn apply_to_field(&self, field: &mut Field) {
        self.layout().iter().enumerate().for_each(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, i)| **i)
                .for_each(|(x, _)| field[self.pos.y - y][self.pos.x + x] = true);
        });
    }
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Parsed {
    input
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
}

fn highest_point(stack: &Field) -> usize {
    stack
        .iter()
        .enumerate()
        .rev()
        .find(|(_, row)| row.iter().any(|i| *i))
        .map(|(i, _)| i + 1)
        .unwrap_or(0)
}

#[aoc(day17, part1)]
fn part1(input: &Parsed) -> usize {
    let rocks = vec![
        vec![vec![true; 4]],
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        vec![
            vec![false, false, true],
            vec![false, false, true],
            vec![true, true, true],
        ],
        vec![vec![true]; 4],
        vec![vec![true; 2]; 2],
    ];

    let mut stack: Vec<[bool; 7]> = vec![];

    let mut next_rock = 0;
    let mut next_push = 0;

    for _ in 0..2022 {
        let mut rock = Rock::new(next_rock);
        next_rock = (next_rock + 1) % rocks.len();

        let highest_point = highest_point(&stack);
        let new_height = highest_point + 3 + rock.height();
        if new_height > stack.len() {
            stack.extend((stack.len()..new_height).map(|_| [false; 7]));
        }
        rock.pos.y = new_height - 1;
        // println!("{}, {}, {}", stack.len(), highest_point, rock.y);

        loop {
            let push = &input[next_push];
            next_push = (next_push + 1) % input.len();

            if rock.can_be_pushed(&stack, push) {
                rock.push(push);
            }

            if rock.pos.y >= 1 && rock.can_move_to((rock.pos.x, rock.pos.y - 1), &stack) {
                rock.pos.y -= 1;
            } else {
                rock.apply_to_field(&mut stack);
                // if stack.len() < 30 {
                //     print_stack(&stack);
                // }
                break;
            }
        }
    }

    highest_point(&stack)
}

// fn print_stack(stack: &Field) {
//     stack.iter().rev().for_each(|row| {
//         row.iter().for_each(|i| {
//             if *i {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         });
//         println!("");
//     });
//     println!("");
// }

const MAP_HEIGHT: usize = 128;
const MAP_WIDTH: usize = 7;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

struct Map {
    contents: [[bool; MAP_WIDTH]; MAP_HEIGHT],
    highest_rock: usize,
    map_height: usize,
}

impl Map {
    pub fn new() -> Self {
        Self {
            contents: [[false; MAP_WIDTH]; MAP_HEIGHT],
            highest_rock: 0,
            map_height: 20,
        }
    }

    #[inline]
    pub fn get_contents(&self, pos: Pos) -> bool {
        self.contents[pos.y % MAP_HEIGHT][pos.x]
    }

    #[inline]
    pub fn set_contents(&mut self, pos: Pos, val: bool) {
        self.contents[pos.y % MAP_HEIGHT][pos.x] = val;
    }

    #[inline]
    pub fn check_bounds(&self, pos: Pos) -> bool {
        pos.x < MAP_WIDTH && pos.y < self.map_height
    }

    pub fn collides_with(&self, rock: Rock) -> bool {
        let pos = rock.pos;
        let rock = ROCKS_2[rock.class];
        (0..4)
            .map(|x| (0..4).map(move |y| (x, y)))
            .flatten()
            .any(|(x, y)| {
                let map_pos: Pos = Pos {
                    x: pos.x.wrapping_add(x),
                    y: pos.y.wrapping_add(y),
                };
                if self.check_bounds(map_pos) {
                    rock[y][x] != false && self.get_contents(map_pos) != false
                } else {
                    rock[y][x] != false
                }
            })
    }

    pub fn add_rock(&mut self, rock: Rock) {
        let pos = rock.pos;
        let rock = ROCKS_2[rock.class];
        for x in 0..4 {
            for y in 0..4 {
                let map_pos: Pos = Pos {
                    x: pos.x.wrapping_add(x),
                    y: pos.y.wrapping_add(y),
                };
                if rock[y][x] != false {
                    self.set_contents(map_pos, rock[y][x]);
                }
            }
        }
        for y in self.highest_rock..self.highest_rock + 4 {
            if (0..MAP_WIDTH).any(|x| self.get_contents(Pos { x, y }) != false) {
                self.highest_rock = y + 1;
            }
        }
        for y in self.map_height..self.highest_rock + 20 {
            for x in 0..MAP_WIDTH {
                self.set_contents(Pos { x, y }, false);
            }
        }
        self.map_height = self.highest_rock + 20;
    }
}

#[aoc(day17, part2)]
fn part2(input: &Parsed) -> usize {
    let jet_len = input.len();
    let mut jet_i = 0;
    let mut map = Map::new();

    let mut heights = Vec::new();
    let mut last_highest = 0;
    let mut last_rock = 1;
    let mut first_rock_delta = 0;
    let mut first_height_delta = 0;
    let mut rock_delta = 0;
    let mut height_delta = 0;
    let mut i_rock = 0;
    let mut num_wraps = 0;
    loop {
        let mut rock = Rock {
            class: i_rock % ROCKS.len(),
            pos: Pos {
                x: 2,
                y: map.highest_rock + 3,
            },
        };
        i_rock += 1;
        let mut last_pos = rock.pos;
        while !map.collides_with(rock) {
            last_pos = rock.pos;
            match input[jet_i] {
                Direction::Left => rock.pos.x = rock.pos.x.wrapping_sub(1),
                Direction::Right => rock.pos.x = rock.pos.x.wrapping_add(1),
            }
            jet_i = (jet_i + 1) % jet_len;
            if jet_i == 0 {
                num_wraps += 1;
                if num_wraps == 1 {
                    first_height_delta = map.highest_rock - last_highest;
                } else if num_wraps > 2 && height_delta != map.highest_rock - last_highest {
                    panic!(
                        "mismatching height deltas {} {}",
                        height_delta,
                        map.highest_rock - last_highest
                    )
                }
                if num_wraps == 1 {
                    first_rock_delta = i_rock - last_rock;
                } else if num_wraps > 2 && rock_delta != i_rock - last_rock {
                    panic!(
                        "mismatching rock deltas {} {}",
                        rock_delta,
                        i_rock - last_rock
                    )
                }
                height_delta = map.highest_rock - last_highest;
                rock_delta = i_rock - last_rock;
                last_highest = map.highest_rock;
                last_rock = i_rock;
            }
            if map.collides_with(rock) {
                rock.pos = last_pos;
            }
            last_pos = rock.pos;
            rock.pos.y = rock.pos.y.wrapping_sub(1);
        }
        rock.pos = last_pos;
        map.add_rock(rock);
        heights.push(map.highest_rock);
        if num_wraps >= 3 && i_rock >= last_rock + rock_delta - 1 {
            break;
        }
    }
    let n2 = 1000000000000;
    let last_rock_delta = (n2 - first_rock_delta - 1) % rock_delta;
    let n2_num_repeats = (n2 - first_rock_delta - last_rock_delta) / rock_delta;
    let n3 = last_rock + last_rock_delta;
    let last_height_delta = heights[n3] - last_highest;

    first_height_delta + n2_num_repeats * height_delta + last_height_delta - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 3068);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 1514285714288);
    }
}
