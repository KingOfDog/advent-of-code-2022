use std::collections::{HashMap, HashSet, LinkedList};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::prelude::*;

type Parsed = (Vec<Vec<Node>>, (usize, usize), (usize, usize));

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
    height: usize,
}

impl Node {
    fn new(x: usize, y: usize, height: usize) -> Node {
        Node { x, y, height }
    }

    fn calculate_heuristic_to(&self, node: &Node) -> usize {
        self.x.abs_diff(node.x) + self.y.abs_diff(node.y)
    }
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Parsed {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        Node::new(x, y, 0)
                    }
                    'E' => {
                        end = (x, y);
                        Node::new(x, y, 25)
                    }
                    _ => Node::new(x, y, (c as usize) - 97),
                })
                .collect()
        })
        .collect();
    (grid, start, end)
}

fn reconstruct_path<'a>(
    came_from: HashMap<&'a Node, &'a Node>,
    current: &'a Node,
) -> LinkedList<&'a Node> {
    let mut path = LinkedList::new();
    path.push_front(current);

    let mut current = current;
    while came_from.contains_key(current) {
        current = came_from[current];
        path.push_front(current);
    }

    path
}

fn neighbors<'a>(grid: &'a Vec<Vec<Node>>, node: &'a Node) -> Vec<&'a Node> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|(off_x, off_y)| (node.x as i32 + off_x, node.y as i32 + off_y))
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *y < grid.len() as i32 && *x < grid[0].len() as i32)
        .map(|(x, y)| &grid[y as usize][x as usize])
        .filter(|neighbor| neighbor.height <= node.height + 1)
        .collect()
}

fn find_path<'a>(
    grid: &'a Vec<Vec<Node>>,
    start: &'a Node,
    end: &'a Node,
) -> Result<LinkedList<&'a Node>, ()> {
    let mut open_set = HashSet::new();
    open_set.insert(start);

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, start.calculate_heuristic_to(end));

    while !open_set.is_empty() {
        let current = *open_set.iter().min_by_key(|x| f_score[**x]).unwrap();
        open_set.remove(current);
        if current == end {
            return Ok(reconstruct_path(came_from, current));
        }

        let neighbors = neighbors(grid, current);
        neighbors.iter().for_each(|neighbor| {
            let candidate_score = g_score[current] + 1;
            if !g_score.contains_key(neighbor) || candidate_score < g_score[neighbor] {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, candidate_score);
                f_score.insert(
                    neighbor,
                    candidate_score + neighbor.calculate_heuristic_to(end),
                );
                open_set.insert(neighbor);
            }
        })
    }

    Err(())
}

#[aoc(day12, part1)]
fn part1(input: &Parsed) -> usize {
    let (grid, start, end) = input;

    let start = &grid[start.1][start.0];
    let end = &grid[end.1][end.0];

    let path = find_path(grid, start, end).unwrap();
    path.len() - 1
}

#[aoc(day12, part2)]
fn part2(input: &Parsed) -> usize {
    let (grid, _, end) = input;
    let end = &grid[end.1][end.0];

    let possible_starts = grid
        .iter()
        .flat_map(|row| row.iter().filter(|node| node.height == 0))
        .collect_vec();
    possible_starts
        .par_iter()
        .filter_map(|start| find_path(grid, start, end).ok().map(|path| path.len() - 1))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 31);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 29);
    }
}
