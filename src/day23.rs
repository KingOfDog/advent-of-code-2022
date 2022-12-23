use std::{
    collections::{HashMap, HashSet, VecDeque},
    vec::IntoIter,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Parsed = HashMap<(i32, i32), Tile>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Elf(usize),
    Empty,
}

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(((x as i32, y as i32), Tile::Elf(y * 100 + x))),
                _ => None,
            })
        })
        .collect()
}

fn neighbors(field: &Parsed, position: (i32, i32)) -> [((i32, i32), Tile); 8] {
    [
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ]
    .map(|offset| {
        let tile_pos = (position.0 + offset.0, position.1 + offset.1);
        let tile = *field.get(&tile_pos).unwrap_or(&Tile::Empty);

        (tile_pos, tile)
    })
}

fn calculate_proposed_positions(
    field: &Parsed,
    preferred_directions: &VecDeque<[usize; 3]>,
) -> HashMap<Tile, (i32, i32)> {
    field
        .iter()
        .map(|(pos, elf)| {
            let neighbors = neighbors(&field, *pos);
            if neighbors.iter().all(|(_, n)| n == &Tile::Empty) {
                return (*elf, *pos);
            }

            for dir in preferred_directions {
                if dir
                    .iter()
                    .map(|i| neighbors[*i])
                    .all(|(_, n)| n == Tile::Empty)
                {
                    return (*elf, neighbors[dir[1]].0);
                }
            }

            return (*elf, *pos);
        })
        .collect()
}

#[aoc(day23, part1)]
fn part1(input: &Parsed) -> usize {
    let mut field = input.clone();

    let mut preferred_directions: VecDeque<[usize; 3]> =
        vec![[1, 2, 3], [5, 6, 7], [1, 0, 7], [3, 4, 5]].into();

    for _ in 0..10 {
        let proposed_directions = calculate_proposed_positions(&field, &preferred_directions);

        let all_positions = proposed_directions.values().collect_vec();
        proposed_directions.iter().for_each(|(elf, position)| {
            if all_positions.iter().filter(|pos| **pos == position).count() > 1 {
                return;
            }

            let prev_pos = *field
                .iter()
                .find_map(|(p, e)| if e == elf { Some(p) } else { None })
                .unwrap();
            if *position == prev_pos {
                return;
            }

            field.remove(&prev_pos);
            field.insert(*position, *elf);
        });

        let first = preferred_directions.pop_front().unwrap();
        preferred_directions.push_back(first);

        // print_field(&field);
    }

    // print_field(&field);

    let min_x = field.keys().min_by_key(|k| k.0).unwrap().0;
    let max_x = field.keys().max_by_key(|k| k.0).unwrap().0;
    let min_y = field.keys().min_by_key(|k| k.1).unwrap().1;
    let max_y = field.keys().max_by_key(|k| k.1).unwrap().1;

    let rect_size = (max_x - min_x + 1) * (max_y - min_y + 1);
    let empty_tiles = rect_size as usize - field.len();
    empty_tiles
}

fn print_field(field: &Parsed) {
    let min_x = field.keys().min_by_key(|k| k.0).unwrap().0;
    let max_x = field.keys().max_by_key(|k| k.0).unwrap().0;
    let min_y = field.keys().min_by_key(|k| k.1).unwrap().1;
    let max_y = field.keys().max_by_key(|k| k.1).unwrap().1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(_) = field.get(&(x, y)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!("");
    }
    println!("");
}

#[aoc(day23, part2)]
fn part2(input: &Parsed) -> usize {
    let mut field = input.clone();

    let mut preferred_directions: VecDeque<[usize; 3]> =
        vec![[1, 2, 3], [5, 6, 7], [1, 0, 7], [3, 4, 5]].into();

    let mut round = 1;
    loop {
        let proposed_directions = calculate_proposed_positions(&field, &preferred_directions);

        let all_positions = proposed_directions.values().collect_vec();
        let mut moved = false;
        proposed_directions.iter().for_each(|(elf, position)| {
            if all_positions.iter().filter(|pos| **pos == position).count() > 1 {
                return;
            }

            let prev_pos = *field
                .iter()
                .find_map(|(p, e)| if e == elf { Some(p) } else { None })
                .unwrap();
            if *position == prev_pos {
                return;
            }

            field.remove(&prev_pos);
            field.insert(*position, *elf);
            moved = true;
        });

        if !moved {
            return round;
        }

        let first = preferred_directions.pop_front().unwrap();
        preferred_directions.push_back(first);

        // print_field(&field);
        round += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 110);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 20);
    }
}
