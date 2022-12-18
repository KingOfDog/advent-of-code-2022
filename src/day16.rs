use std::{cmp, collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type Parsed = HashMap<String, Valve>;

struct Valve {
    flow: i64,
    mask: i64,
    tunnels: Vec<String>,
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Parsed {
    let pattern = Regex::new("^Valve ([A-Z]+) .+=(\\d+); .+ valves? (.+)$").unwrap();
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let l = pattern.captures(l).unwrap();
            let name = l[1].to_string();
            let flow_rate = l[2].parse().unwrap();
            let leading_to = l[3].split(", ").map(|s| s.to_string()).collect();
            (
                name,
                Valve {
                    flow: flow_rate,
                    mask: i64::pow(2, i as u32),
                    tunnels: leading_to,
                },
            )
        })
        .collect()
}

fn calc_distances(cave: &Parsed) -> HashMap<(String, String), usize> {
    let mut distances: HashMap<(String, String), usize> = HashMap::new();
    cave.keys().for_each(|x| {
        cave.keys().for_each(|y| {
            if cave.get(x).unwrap().tunnels.contains(y) {
                distances.entry((x.clone(), y.clone())).or_insert(1);
            } else {
                distances
                    .entry((x.clone(), y.clone()))
                    .or_insert(usize::MAX);
            }
        });
    });
    cave.keys().for_each(|k| {
        cave.keys().for_each(|i| {
            cave.keys().for_each(|j| {
                let ij: usize;
                let ik: usize;
                let kj: usize;
                let tmp: usize;
                {
                    ij = *distances.get(&(i.clone(), j.clone())).unwrap();
                }
                {
                    ik = *distances.get(&(i.clone(), k.clone())).unwrap();
                }
                {
                    kj = *distances.get(&(k.clone(), j.clone())).unwrap();
                }
                if ik == usize::MAX || kj == usize::MAX {
                    // workaround to avoid overflow on addition
                    tmp = cmp::min(ij, usize::MAX);
                } else {
                    tmp = cmp::min(ij, ik + kj);
                }
                {
                    distances.insert((i.clone(), j.clone()), tmp);
                }
            });
        });
    });
    return distances;
}

fn visit<'a>(
    valve: String,
    budget: i64,
    state: i64,
    cave: &Parsed,
    distances: &HashMap<(String, String), usize>,
    flow: i64,
    answer: &'a mut HashMap<i64, i64>,
) -> &'a mut HashMap<i64, i64> {
    let n: i64;
    if !answer.contains_key(&state) {
        n = 0
    } else {
        n = *answer.get(&state).unwrap();
    }
    answer.insert(state, cmp::max(n, flow));
    for k in cave.iter().filter(|(_, cv)| cv.flow > 0).map(|(ck, _)| ck) {
        let dist: usize;
        {
            dist = *distances.get(&(valve.clone(), k.clone())).unwrap();
        }
        let new_budget = budget - dist as i64 - 1;
        let mask = cave.get(k).unwrap().mask;
        if (state & mask) != 0 || new_budget < 0 {
            continue;
        } else {
            let flow_here = cave.get(k).unwrap().flow;
            let _ = visit(
                k.clone(),
                new_budget,
                state | mask,
                &cave,
                &distances,
                flow + (new_budget * flow_here),
                answer,
            );
        }
    }
    return answer;
}

#[aoc(day16, part1)]
fn part1(input: &Parsed) -> i64 {
    let distances = calc_distances(input);
    let mut answer: HashMap<i64, i64> = HashMap::new();
    let answer = visit(String::from("AA"), 30, 0, input, &distances, 0, &mut answer);
    *answer.values().max().unwrap()
}

#[aoc(day16, part2)]
fn part2(input: &Parsed) -> i64 {
    let distances = calc_distances(input);
    let mut answer: HashMap<i64, i64> = HashMap::new();
    let answer = visit(String::from("AA"), 26, 0, input, &distances, 0, &mut answer);
    let mut total = 0;
    for (k1, v1) in answer.iter() {
        for (k2, v2) in answer.iter() {
            if (k1 & k2) == 0 {
                if v1 + v2 > total {
                    total = v1 + v2;
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 1651);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 1707);
    }
}
