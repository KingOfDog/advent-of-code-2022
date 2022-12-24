use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

type Parsed = ((usize, usize), Vec<Blizzard>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn wrap(&self, (width, height): (usize, usize)) -> Position {
        let mut x = self.x;
        let mut y = self.y;
        if x < 1 {
            x = width - 2;
        } else if x >= width - 1 {
            x = 1;
        }
        if y < 1 {
            y = height - 2;
        } else if y >= height - 1 {
            y = 1;
        }
        Position { x, y }
    }

    fn is_wall(&self, (width, height): (usize, usize)) -> bool {
        if self.x <= 0 || self.x >= width - 1 {
            return true;
        }
        if self.y <= 0 && self.x != 1 {
            return true;
        }
        if self.y >= height - 1 && self.x != width - 2 {
            return true;
        }
        false
    }

    fn destinations(&self, size: (usize, usize)) -> Vec<Position> {
        [(-1, 0), (0, 1), (1, 0), (0, -1), (0, 0)]
            .iter()
            .map(|(x, y)| (self.x as i32 + x, self.y as i32 + y))
            .map(|(x, y)| Position {
                x: x as usize,
                y: y as usize,
            })
            .filter(|pos| !pos.is_wall(size))
            .collect()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Blizzard {
    position: Position,
    direction: Direction,
}

impl Blizzard {
    fn simulate(&self, size: (usize, usize)) -> Blizzard {
        let position = match self.direction {
            Direction::Up => Position {
                x: self.position.x,
                y: self.position.y - 1,
            },
            Direction::Right => Position {
                x: self.position.x + 1,
                y: self.position.y,
            },
            Direction::Down => Position {
                x: self.position.x,
                y: self.position.y + 1,
            },
            Direction::Left => Position {
                x: self.position.x - 1,
                y: self.position.y,
            },
        }
        .wrap(size);

        Blizzard {
            position,
            direction: self.direction,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Parsed {
    let width = input.lines().collect_vec().first().unwrap().len();
    let height = input.lines().collect_vec().len();

    let blizzards = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, c)| {
                let position = Position { x, y };
                match c {
                    '^' => Some(Blizzard {
                        position,
                        direction: Direction::Up,
                    }),
                    '>' => Some(Blizzard {
                        position,
                        direction: Direction::Right,
                    }),
                    'v' => Some(Blizzard {
                        position,
                        direction: Direction::Down,
                    }),
                    '<' => Some(Blizzard {
                        position,
                        direction: Direction::Left,
                    }),
                    _ => None,
                }
            })
        })
        .collect();

    ((width, height), blizzards)
}

fn simulate_blizzards(blizzards: &Vec<Blizzard>, size: (usize, usize)) -> Vec<Blizzard> {
    blizzards
        .iter()
        .map(|blizzard| blizzard.simulate(size))
        .collect()
}

fn find_quickest_path(
    blizzards: &Vec<Blizzard>,
    from: Position,
    to: Position,
    size: (usize, usize),
) -> (Vec<Blizzard>, usize) {
    let shortest_path = dijkstra(
        &(from, blizzards.clone()),
        |(pos, blizzards)| {
            let next_blizzards = simulate_blizzards(blizzards, size);
            let destinations = pos.destinations(size);
            destinations
                .iter()
                .filter(|dest| {
                    next_blizzards
                        .iter()
                        .find(|p| p.position == **dest)
                        .is_none()
                })
                .map(|dest| ((*dest, next_blizzards.clone()), 1))
                .collect_vec()
        },
        |(pos, _)| *pos == to,
    )
    .unwrap();

    let final_state = shortest_path.0.last().unwrap().1.clone();
    let length = shortest_path.1;

    (final_state, length)
}

#[aoc(day24, part1)]
fn part1(input: &Parsed) -> usize {
    let ((width, height), blizzards) = input;

    let start = Position { x: 1, y: 0 };
    let end = Position {
        x: width - 2,
        y: height - 1,
    };

    let (_, length) = find_quickest_path(blizzards, start, end, (*width, *height));
    length
}

#[aoc(day24, part2)]
fn part2(input: &Parsed) -> usize {
    let ((width, height), blizzards) = input;

    let start = Position { x: 1, y: 0 };
    let end = Position {
        x: width - 2,
        y: height - 1,
    };

    let size = (*width, *height);
    let (blizzards, length) = find_quickest_path(blizzards, start, end, size);
    let (blizzards, length2) = find_quickest_path(&blizzards, end, start, size);
    let (_, length3) = find_quickest_path(&blizzards, start, end, size);

    length + length2 + length3
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 18);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 54);
    }
}
