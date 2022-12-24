use std::{
    collections::HashMap,
    ops::{Add, Sub},
    str::FromStr,
};

use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = (Field, Vec<Command>);

type Field = Vec<Vec<Tile>>;

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    OffLimits,
    Open,
    Wall,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Open),
            "#" => Ok(Tile::Wall),
            _ => Ok(Tile::OffLimits),
        }
    }
}

enum Command {
    Move(u32),
    TurnCounterClock,
    TurnClockWise,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Add<i32> for Facing {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        let new = self.value() + rhs;
        match (new + 4) % 4 {
            0 => Facing::Right,
            1 => Facing::Down,
            2 => Facing::Left,
            3 => Facing::Up,
            _ => self,
        }
    }
}

impl Sub<i32> for Facing {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        self + (-rhs)
    }
}

impl Facing {
    fn value(&self) -> i32 {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }

    fn position_modifier(&self) -> Position {
        match self {
            Facing::Right => Position { x: 1, y: 0 },
            Facing::Down => Position { x: 0, y: 1 },
            Facing::Left => Position { x: -1, y: 0 },
            Facing::Up => Position { x: 0, y: -1 },
        }
    }

    fn turn_clockwise(self) -> Facing {
        self + 1
    }

    fn turn_counterclockwise(self) -> Facing {
        self - 1
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Side {
    Front,
    Top,
    Left,
    Right,
    Bottom,
    Back,
}

impl Side {
    fn from_position(position: Position) -> Self {
        let cx = position.x - position.x % SIDE_LENGTH;
        let cy = position.y - position.y % SIDE_LENGTH;
        // println!("{cx}, {cy}");
        *SIDES
            .iter()
            .find(|(_, (pos, _))| pos.x == cx && pos.y == cy)
            .unwrap()
            .0
    }
}

#[aoc_generator(day22)]
fn parse_input(input: &str) -> Parsed {
    let (grid, commands) = input.split_once("\n\n").unwrap();

    let field = grid
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    let commands = commands
        .split_inclusive(&['R', 'L'])
        .flat_map(|part| {
            if part.chars().last().unwrap().is_alphabetic() {
                let number = &part[..part.len() - 1].parse().unwrap();
                let direction = match part.chars().last().unwrap() {
                    'L' => Command::TurnCounterClock,
                    'R' | _ => Command::TurnClockWise,
                };
                vec![Command::Move(*number), direction]
            } else {
                let number = part
                    .trim()
                    .parse()
                    .expect(format!("could not parse {part}").as_str());
                vec![Command::Move(number)]
            }
        })
        .collect();

    (field, commands)
}

#[aoc(day22, part1)]
fn part1(input: &Parsed) -> i32 {
    let (field, commands) = input;

    let mut position = Position {
        x: field[0].iter().position(|x| *x == Tile::Open).unwrap() as i32,
        y: 0,
    };
    let mut facing = Facing::Right;

    commands.iter().for_each(|command| match command {
        Command::Move(steps) => {
            for _ in 0..*steps {
                let mut next_position = position + facing.position_modifier();
                if next_position.y < 0 {
                    next_position.y = field
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, row)| row.len() as i32 > next_position.x)
                        .unwrap()
                        .0 as i32;
                } else if next_position.y >= field.len() as i32 {
                    next_position.y = field
                        .iter()
                        .position(|row| row[next_position.x as usize] != Tile::OffLimits)
                        .unwrap() as i32;
                } else if next_position.x < 0 {
                    next_position.x = field[next_position.y as usize].len() as i32 - 1;
                } else if next_position.x >= field[next_position.y as usize].len() as i32 {
                    if facing == Facing::Left || facing == Facing::Right {
                        next_position.x = field[next_position.y as usize]
                            .iter()
                            .position(|tile| *tile != Tile::OffLimits)
                            .unwrap() as i32;
                    } else if facing == Facing::Up || facing == Facing::Down {
                        next_position.y = field
                            .iter()
                            .position(|row| row[next_position.x as usize] != Tile::OffLimits)
                            .unwrap() as i32;
                    }
                } else if matches!(
                    field[next_position.y as usize][next_position.x as usize],
                    Tile::OffLimits
                ) {
                    match facing {
                        Facing::Left => {
                            next_position.x = field[next_position.y as usize].len() as i32 - 1
                        }
                        Facing::Up => {
                            next_position.y = field
                                .iter()
                                .enumerate()
                                .rev()
                                .find(|(_, row)| row.len() as i32 > next_position.x)
                                .unwrap()
                                .0 as i32;
                        }
                        _ => (),
                    }
                }

                if field[next_position.y as usize][next_position.x as usize] == Tile::Wall {
                    break;
                }
                position = next_position;
            }
        }
        Command::TurnClockWise => facing = facing.turn_clockwise(),
        Command::TurnCounterClock => facing = facing.turn_counterclockwise(),
    });

    1000 * (position.y + 1) + 4 * (position.x + 1) + facing.value()
}

/* const SIDE_LENGTH: i32 = 4;
lazy_static! {
    static ref SIDES: HashMap<Side, (Position, i32)> = {
        let mut sides = HashMap::new();
        sides.insert(
            Side::Top,
            (
                Position {
                    x: SIDE_LENGTH * 2,
                    y: 0,
                },
                0,
            ),
        );
        sides.insert(
            Side::Front,
            (
                Position {
                    x: SIDE_LENGTH * 2,
                    y: SIDE_LENGTH,
                },
                0,
            ),
        );
        sides.insert(
            Side::Bottom,
            (
                Position {
                    x: SIDE_LENGTH * 2,
                    y: SIDE_LENGTH * 2,
                },
                0,
            ),
        );
        sides.insert(
            Side::Back,
            (
                Position {
                    x: 0,
                    y: SIDE_LENGTH,
                },
                0,
            ),
        );
        sides.insert(
            Side::Left,
            (
                Position {
                    x: SIDE_LENGTH,
                    y: SIDE_LENGTH,
                },
                0,
            ),
        );
        sides.insert(
            Side::Right,
            (
                Position {
                    x: SIDE_LENGTH * 3,
                    y: SIDE_LENGTH * 2,
                },
                0,
            ),
        );

        sides
    };
} */
const SIDE_LENGTH: i32 = 50;
lazy_static! {
    static ref SIDES: HashMap<Side, (Position, i32)> = {
        let mut sides = HashMap::new();
        sides.insert(
            Side::Top,
            (
                Position {
                    x: SIDE_LENGTH,
                    y: 0,
                },
                0,
            ),
        );
        sides.insert(
            Side::Front,
            (
                Position {
                    x: SIDE_LENGTH,
                    y: SIDE_LENGTH,
                },
                0,
            ),
        );
        sides.insert(
            Side::Bottom,
            (
                Position {
                    x: SIDE_LENGTH,
                    y: SIDE_LENGTH * 2,
                },
                0,
            ),
        );
        sides.insert(
            Side::Back,
            (
                Position {
                    x: 0,
                    y: SIDE_LENGTH * 3,
                },
                0,
            ),
        );
        sides.insert(
            Side::Left,
            (
                Position {
                    x: 0,
                    y: SIDE_LENGTH * 2,
                },
                0,
            ),
        );
        sides.insert(
            Side::Right,
            (
                Position {
                    x: SIDE_LENGTH * 2,
                    y: 0,
                },
                0,
            ),
        );

        sides
    };
}

#[aoc(day22, part2)]
fn part2(input: &Parsed) -> i32 {
    let (field, commands) = input;

    let mut position = Position {
        x: field[0].iter().position(|x| *x == Tile::Open).unwrap() as i32,
        y: 0,
    };
    let mut facing = Facing::Right;

    commands.iter().for_each(|command| match command {
        Command::Move(steps) => {
            for _ in 0..*steps {
                let mut next_position = position + facing.position_modifier();
                let mut next_facing = facing;
                if *field
                    .get(next_position.y as usize)
                    .and_then(|row| row.get(next_position.x as usize))
                    .unwrap_or(&Tile::OffLimits)
                    == Tile::OffLimits
                {
                    let sx = position.x % SIDE_LENGTH;
                    let sy = position.y % SIDE_LENGTH;

                    let side = Side::from_position(position);

                    // println!("{:?}, {sx}, {sy}", side);

                    if side == Side::Top && facing == Facing::Left {
                        let next_side = SIDES[&Side::Left].0;
                        next_position = Position {
                            x: next_side.x,
                            y: next_side.y + SIDE_LENGTH - 1 - sy,
                        };
                        next_facing = Facing::Right;
                    } else if side == Side::Top && facing == Facing::Up {
                        let next_side = SIDES[&Side::Back].0;
                        next_position = Position {
                            x: next_side.x,
                            y: next_side.y + sx,
                        };
                        next_facing = Facing::Right;
                    } else if side == Side::Right && facing == Facing::Up {
                        let next_side = SIDES[&Side::Back].0;
                        next_position = Position {
                            x: next_side.x + sx,
                            y: next_side.y + SIDE_LENGTH - 1,
                        };
                        next_facing = Facing::Up;
                    } else if side == Side::Right && facing == Facing::Right {
                        let next_side = SIDES[&Side::Bottom].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1,
                            y: next_side.y + SIDE_LENGTH - 1 - sy,
                        };
                        next_facing = Facing::Left;
                    } else if side == Side::Right && facing == Facing::Down {
                        let next_side = SIDES[&Side::Front].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1,
                            y: next_side.y + sx,
                        };
                        next_facing = Facing::Left;
                    } else if side == Side::Front && facing == Facing::Left {
                        let next_side = SIDES[&Side::Left].0;
                        next_position = Position {
                            x: next_side.x + sy,
                            y: next_side.y,
                        };
                        next_facing = Facing::Down;
                    } else if side == Side::Front && facing == Facing::Right {
                        let next_side = SIDES[&Side::Right].0;
                        next_position = Position {
                            x: next_side.x + sy,
                            y: next_side.y + SIDE_LENGTH - 1,
                        };
                        next_facing = Facing::Up;
                    } else if side == Side::Left && facing == Facing::Left {
                        let next_side = SIDES[&Side::Top].0;
                        next_position = Position {
                            x: next_side.x,
                            y: next_side.y + SIDE_LENGTH - 1 - sy,
                        };
                        next_facing = Facing::Right;
                    } else if side == Side::Left && facing == Facing::Up {
                        let next_side = SIDES[&Side::Front].0;
                        next_position = Position {
                            x: next_side.x,
                            y: next_side.y + sx,
                        };
                        next_facing = Facing::Right;
                    } else if side == Side::Bottom && facing == Facing::Right {
                        let next_side = SIDES[&Side::Right].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1,
                            y: next_side.y + SIDE_LENGTH - 1 - sy,
                        };
                        next_facing = Facing::Left;
                    } else if side == Side::Bottom && facing == Facing::Down {
                        let next_side = SIDES[&Side::Back].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1,
                            y: next_side.y + sx,
                        };
                        next_facing = Facing::Left;
                    } else if side == Side::Back && facing == Facing::Left {
                        let next_side = SIDES[&Side::Top].0;
                        next_position = Position {
                            x: next_side.x + sy,
                            y: next_side.y,
                        };
                        next_facing = Facing::Down;
                    } else if side == Side::Back && facing == Facing::Right {
                        let next_side = SIDES[&Side::Bottom].0;
                        next_position = Position {
                            x: next_side.x + sy,
                            y: next_side.y + SIDE_LENGTH - 1,
                        };
                        next_facing = Facing::Up;
                    } else if side == Side::Back && facing == Facing::Down {
                        let next_side = SIDES[&Side::Right].0;
                        next_position = Position {
                            x: next_side.x + sx,
                            y: next_side.y,
                        };
                        next_facing = Facing::Down;
                    }

                    /*if side == Side::Back && facing == Facing::Left {
                        let next_side = SIDES[&Side::Right].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1 - sy,
                            y: next_side.y + SIDE_LENGTH - 1,
                        };
                        next_facing = Facing::Up;
                    } else if side == Side::Back && facing == Facing::Up {
                        let next_side = SIDES[&Side::Top].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1 - sx,
                            y: next_side.y,
                        };
                        next_facing = Facing::Down;
                    } else if side == Side::Back && facing == Facing::Down {
                        let next_side = SIDES[&Side::Bottom].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1 - sx,
                            y: next_side.y + SIDE_LENGTH - 1,
                        };
                        next_facing = Facing::Up;
                    } else if side == Side::Left && facing == Facing::Up {
                        let next_side = SIDES[&Side::Top].0;
                        next_position = Position {
                            x: next_side.x,
                            y: next_side.y + sx,
                        };
                        next_facing = Facing::Down;
                    } else if side == Side::Left && facing == Facing::Down {
                        let next_side = SIDES[&Side::Bottom].0;
                        next_position = Position {
                            x: next_side.x,
                            y: next_side.y + SIDE_LENGTH - 1 - sx,
                        };
                        next_facing = Facing::Down;
                    } else if side == Side::Bottom && facing == Facing::Down {
                        let next_side = SIDES[&Side::Back].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1 - sx,
                            y: next_side.y + SIDE_LENGTH - 1,
                        };
                        next_facing = Facing::Up;
                    } else if side == Side::Bottom && facing == Facing::Left {
                        let next_side = SIDES[&Side::Left].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1 - sy,
                            y: next_side.y + SIDE_LENGTH - 1,
                        };
                        next_facing = Facing::Up;
                    } else if side == Side::Right && facing == Facing::Down {
                        let next_side = SIDES[&Side::Back].0;
                        next_position = Position {
                            x: next_side.x,
                            y: next_side.y + SIDE_LENGTH - 1 - sx,
                        };
                        next_facing = Facing::Right;
                    } else if side == Side::Right && facing == Facing::Up {
                        let next_side = SIDES[&Side::Front].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1,
                            y: next_side.y + SIDE_LENGTH - 1 - sx,
                        };
                        next_facing = Facing::Left;
                    } else if side == Side::Right && facing == Facing::Right {
                        let next_side = SIDES[&Side::Top].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1,
                            y: next_side.y + SIDE_LENGTH - 1 - sy,
                        };
                        next_facing = Facing::Left;
                    } else if side == Side::Front && facing == Facing::Right {
                        let next_side = SIDES[&Side::Right].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1 - sy,
                            y: next_side.y,
                        };
                        println!("{:?}, {:?}", next_side, next_position);
                        next_facing = Facing::Down;
                    } else if side == Side::Top && facing == Facing::Left {
                        let next_side = SIDES[&Side::Left].0;
                        next_position = Position {
                            x: next_side.x + sy,
                            y: next_side.y,
                        };
                        next_facing = Facing::Down;
                    } else if side == Side::Top && facing == Facing::Up {
                        let next_side = SIDES[&Side::Back].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1 - sx,
                            y: next_side.y,
                        };
                        next_facing = Facing::Down;
                    } else if side == Side::Top && facing == Facing::Right {
                        let next_side = SIDES[&Side::Right].0;
                        next_position = Position {
                            x: next_side.x + SIDE_LENGTH - 1,
                            y: next_side.y + SIDE_LENGTH - 1 - sy,
                        };
                        next_facing = Facing::Left;
                    }*/
                }

                if field[next_position.y as usize][next_position.x as usize] == Tile::Wall {
                    break;
                }
                position = next_position;
                facing = next_facing;

                // println!("{:?}, {:?}", position, facing);
            }
        }
        Command::TurnClockWise => facing = facing.turn_clockwise(),
        Command::TurnCounterClock => facing = facing.turn_counterclockwise(),
    });

    1000 * (position.y + 1) + 4 * (position.x + 1) + facing.value()
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::*;

    fn input<'a>() -> &'a str {
        "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 6032);
    }

    #[test]
    fn input1() {
        assert_eq!(
            part1(&parse_input(
                &read_to_string("input/2022/day22.txt").unwrap()
            )),
            196134
        );
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 5031);
    }
}
