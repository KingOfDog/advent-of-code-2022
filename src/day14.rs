use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Parsed = Vec<Line>;

type Point = (usize, usize);

#[derive(Debug, Copy, Clone)]
struct Line {
    from: Point,
    to: Point,
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Air,
    Rock,
    Sand,
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|point| {
                    point
                        .split(',')
                        .map(|part| part.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .tuple_windows()
                .map(|(from, to)| Line { from, to })
        })
        .collect()
}

fn bounds_x(lines: &Parsed) -> (usize, usize) {
    let min_x = lines.iter().min_by_key(|line| line.to.0).unwrap();
    let max_x = lines.iter().max_by_key(|line| line.from.0).unwrap();
    (min_x.to.0, max_x.from.0)
}

fn bounds_y(lines: &Parsed) -> (usize, usize) {
    let min_y = lines.iter().min_by_key(|line| line.from.1).unwrap();
    let max_y = lines.iter().max_by_key(|line| line.to.1).unwrap();
    (min_y.from.1, max_y.to.1)
}

fn build_tiles(
    lines: &Parsed,
    bounds_x: (usize, usize),
    bounds_y: (usize, usize),
) -> Vec<Vec<Tile>> {
    let width = bounds_x.1 - bounds_x.0 + 1;
    let height = bounds_y.1 + 1;
    let mut grid = vec![vec![Tile::Air; width]; height];

    lines.iter().for_each(|line| {
        if line.from.0 < line.to.0 {
            let y = line.from.1;
            for x in line.from.0..=line.to.0 {
                grid[y][x - bounds_x.0] = Tile::Rock
            }
        } else if line.from.0 > line.to.0 {
            let y = line.from.1;
            for x in line.to.0..=line.from.0 {
                grid[y][x - bounds_x.0] = Tile::Rock
            }
        } else if line.from.1 < line.to.1 {
            let x = line.from.0;
            for y in line.from.1..=line.to.1 {
                grid[y][x - bounds_x.0] = Tile::Rock
            }
        } else if line.to.1 < line.from.1 {
            let x = line.from.0;
            for y in line.to.1..=line.from.1 {
                grid[y][x - bounds_x.0] = Tile::Rock
            }
        }
    });

    grid
}

fn print_grid(grid: &Vec<Vec<Tile>>) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|cell| match cell {
            Tile::Air => print!(" "),
            Tile::Rock => print!("#"),
            Tile::Sand => print!("O"),
        });
        println!("");
    });
}

fn find_next_sand_spot(grid: &Vec<Vec<Tile>>, x: usize) -> Option<(usize, usize)> {
    let mut x = x as i32;
    let mut y = 0 as i32;
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    while x >= 0 && x < width && y < height {
        if y + 1 >= height || matches!(grid[y as usize + 1][x as usize], Tile::Air) {
            y += 1;
            continue;
        }
        if x - 1 < 0 || matches!(grid[y as usize + 1][x as usize - 1], Tile::Air) {
            x -= 1;
            y += 1;
            continue;
        }
        if x + 1 >= width || matches!(grid[y as usize + 1][x as usize + 1], Tile::Air) {
            x += 1;
            y += 1;
            continue;
        }
        return Some((x as usize, y as usize));
    }

    None
}

fn build_tiles_b(lines: &Parsed) -> HashMap<Point, Tile> {
    let mut grid = HashMap::new();

    lines.iter().for_each(|line| {
        if line.from.0 < line.to.0 {
            let y = line.from.1;
            for x in line.from.0..=line.to.0 {
                grid.insert((x, y), Tile::Rock);
            }
        } else if line.from.0 > line.to.0 {
            let y = line.from.1;
            for x in line.to.0..=line.from.0 {
                grid.insert((x, y), Tile::Rock);
            }
        } else if line.from.1 < line.to.1 {
            let x = line.from.0;
            for y in line.from.1..=line.to.1 {
                grid.insert((x, y), Tile::Rock);
            }
        } else if line.to.1 < line.from.1 {
            let x = line.from.0;
            for y in line.to.1..=line.from.1 {
                grid.insert((x, y), Tile::Rock);
            }
        }
    });

    grid
}

fn print_grid_b(grid: &HashMap<Point, Tile>) {
    let min_x = grid.keys().min_by_key(|key| key.0).unwrap().0;
    let max_x = grid.keys().max_by_key(|key| key.0).unwrap().0;
    let max_y = grid.keys().max_by_key(|key| key.1).unwrap().1;

    for y in 0..=max_y {
        for x in min_x..=max_x {
            if let Some(tile) = grid.get(&(x, y)) {
                match tile {
                    Tile::Air => print!(" "),
                    Tile::Rock => print!("#"),
                    Tile::Sand => print!("O"),
                }
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("");
}

fn find_next_sand_spot_b(grid: &HashMap<Point, Tile>, x: usize, height: usize) -> (usize, usize) {
    let mut x = x;
    let mut y = 0;
    loop {
        if !grid.contains_key(&(x, y + 1)) && y + 1 < height {
            y += 1;
            continue;
        }
        if !grid.contains_key(&(x - 1, y + 1)) && y + 1 < height {
            x -= 1;
            y += 1;
            continue;
        }
        if !grid.contains_key(&(x + 1, y + 1)) && y + 1 < height {
            x += 1;
            y += 1;
            continue;
        }
        return (x as usize, y as usize);
    }
}

#[aoc(day14, part1)]
fn part1(input: &Parsed) -> usize {
    let bounds_x = bounds_x(input);
    let bounds_y = bounds_y(input);
    let mut grid = build_tiles(input, bounds_x, bounds_y);

    print_grid(&grid);

    let mut count = 0;
    while let Some(pos) = find_next_sand_spot(&grid, 500 - bounds_x.0) {
        grid[pos.1][pos.0] = Tile::Sand;
        count += 1;
    }

    println!("");
    print_grid(&grid);

    count
}

#[aoc(day14, part2)]
fn part2(input: &Parsed) -> usize {
    let bounds_y = bounds_y(input);
    let mut grid = build_tiles_b(input);

    let height = bounds_y.1 + 2;

    print_grid_b(&grid);

    let mut count = 0;
    loop {
        let pos = find_next_sand_spot_b(&grid, 500, height);
        if pos == (500, 0) {
            println!("");
            print_grid_b(&grid);
            return count + 1;
        }

        grid.insert(pos, Tile::Sand);
        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 24);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 93);
    }
}
