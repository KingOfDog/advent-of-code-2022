use std::{
    collections::{HashMap, HashSet, LinkedList},
    thread,
    time::Duration,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::dfs;
use rayon::prelude::*;
use regex::Regex;

type Valves = HashMap<String, Valve>;

type Parsed = (String, Valves);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Valve {
    name: String,
    flow_rate: usize,
    leading_to: Vec<String>,
}

impl Valve {
    fn neighbors<'a>(&'a self, valves: &'a Valves) -> Vec<&'a Valve> {
        self.leading_to.iter().map(|id| &valves[id]).collect()
    }

    fn find_best_path(
        &self,
        valves: &Valves,
        connections: &HashMap<(String, String), Vec<String>>,
        available_valves: &HashSet<&String>,
        current_time: i64,
        time_limit: i64,
        current_pressure: i64,
        current_flow: i64,
        indent: usize,
    ) -> usize {
        // if time_remaining <= 0 {
        //     return 0;
        // }

        let n_score = current_pressure + (time_limit - current_time) * current_flow;
        let mut max = n_score as usize;
        let mut target = String::from("");

        if indent == 4 && self.name == "BB" {
            println!("{}, {},{}", current_time, current_pressure, current_flow);
        }

        available_valves
            .iter()
            .map(|valve| {
                let valve = &valves[*valve];
                let distance =
                    connections[&(self.name.clone(), valve.name.clone())].len() as i64 + 1;
                (valve, distance)
            })
            .filter(|(_, distance)| current_time + distance < time_limit)
            .for_each(|(valve, distance)| {
                let mut available_valves = available_valves.clone();
                available_valves.remove(&valve.name);
                let score = valve.find_best_path(
                    valves,
                    connections,
                    &available_valves,
                    current_time + distance,
                    time_limit,
                    current_pressure + distance * current_flow,
                    current_flow + valve.flow_rate as i64,
                    indent + 2,
                );
                if indent == 2 && self.name == "DD" {
                    println!("{}, {}", valve.name, score);
                }
                if score > max {
                    target = valve.name.clone();
                    max = score;
                }
            });

        // println!(
        //     "{}, {}, {}, {}, {}",
        //     self.name, max, current_pressure, current_flow, current_time
        // );
        if indent < 6 {
            println!(
                "{}{} took to {}",
                vec![' '; indent].iter().collect::<String>(),
                self.name,
                target
            );
        }

        max

        // let mut available_valves = available_valves.clone();
        // let open_this_valve = available_valves.remove(&self.name);
        // let time_spent_here = if open_this_valve { 1 } else { 0 };

        // let max = available_valves
        //     .iter()
        //     .map(|valve| {
        //         let valve = &valves[*valve];
        //         let connection = connections
        //             .get(&(self.name.clone(), valve.name.clone()))
        //             .unwrap();
        //         let (path, value) = valve.find_best_path(
        //             valves,
        //             connections,
        //             &available_valves,
        //             time_remaining - connection.len() as i64 - time_spent_here,
        //             indent + 2,
        //         );
        //         let mut path = path;
        //         let mut connection = LinkedList::from_iter(connection.iter().map(|s| s.clone()));
        //         connection.pop_back();
        //         connection.append(&mut path);

        //         (valve, (connection, value))
        //     })
        //     .max_by_key(|p| p.1 .1)
        //     .unwrap_or((self, (LinkedList::new(), 0)));
        // let (following, max) = max.1;

        // if open_this_valve {
        //     (following, max + self.flow_rate * (time_remaining as usize))
        // } else {
        //     (following, max)
        // }
    }

    fn trace_path(
        &self,
        valves: &Valves,
        open_valves: HashSet<&String>,
        time_elapsed: usize,
        path: Vec<&String>,
    ) -> usize {
        // println!("{}, {:?}, time: {}", self.name, path, time_elapsed);
        if time_elapsed >= 28 {
            return 0;
        }
        if path.len() > 3 && path.contains(&&self.name) {
            let position = path
                .iter()
                .enumerate()
                .rev()
                .find(|p| p.1 == &&self.name)
                .unwrap()
                .0;
            if path[position + 1..path.len()].iter().all(|p| {
                // println!("{p}, {}", path.iter().positions(|s| s == p).count());
                path.iter().positions(|s| s == p).count() > 1
            }) {
                return 0;
            }
        }
        let mut path = path.clone();
        path.push(&self.name);
        if self.flow_rate == 0 || open_valves.contains(&self.name) {
            return self
                .neighbors(valves)
                .par_iter()
                .filter(|p| Some(&&p.name) != path.last())
                .map(|neighbor| {
                    neighbor.trace_path(valves, open_valves.clone(), time_elapsed + 1, path.clone())
                })
                .max()
                .unwrap_or(0);
        }
        let max_value_with_opening = self
            .neighbors(valves)
            .par_iter()
            .map(|neighbor| {
                let mut open_valves = open_valves.clone();
                open_valves.insert(&self.name);
                neighbor.trace_path(valves, open_valves, time_elapsed + 2, path.clone())
            })
            .max()
            .unwrap_or(0)
            + self.flow_rate * (30 - time_elapsed - 1);
        let max_value_without_opening = self
            .neighbors(valves)
            .par_iter()
            .filter(|p| Some(&&p.name) != path.last())
            .map(|neighbor| {
                neighbor.trace_path(valves, open_valves.clone(), time_elapsed + 1, path.clone())
            })
            .max()
            .unwrap_or(0);
        // println!(
        //     "{}, {:?}, time: {}, {} vs. {}",
        //     self.name, open_valves, time_elapsed, max_value_without_opening, max_value_with_opening
        // );
        return max_value_with_opening.max(max_value_without_opening);
    }
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Parsed {
    let pattern = Regex::new("^Valve ([A-Z]+) .+=(\\d+); .+ valves? (.+)$").unwrap();
    let valves = input.lines().map(|l| {
        let l = pattern.captures(l).unwrap();
        let name = l[1].to_string();
        let flow_rate = l[2].parse().unwrap();
        let leading_to = l[3].split(", ").map(|s| s.to_string()).collect();
        (
            name.clone(),
            Valve {
                name,
                flow_rate,
                leading_to,
            },
        )
    });

    let start = valves.clone().next().unwrap().0;
    (start, valves.collect())
}

#[aoc(day16, part1)]
fn part1(input: &Parsed) -> usize {
    // println!("{}", input.0);
    // input.1[&input.0].trace_path(&input.1, HashSet::new(), 0, vec![])

    let all_valves = input.1.clone();
    let sensible_valves: HashMap<_, _> = all_valves
        .iter()
        .filter(|(_, valve)| valve.flow_rate > 0)
        .collect();
    let connections: HashMap<(String, String), Vec<String>> = all_valves
        .iter()
        .flat_map(|(_, valve)| {
            sensible_valves.iter().map(|(_, other_valve)| {
                let path = dfs(
                    valve.name.clone(),
                    |v| all_valves[v].leading_to.clone(),
                    |v| v == &other_valve.name,
                )
                .unwrap();
                ((valve.name.clone(), other_valve.name.clone()), path)
            })
        })
        .collect();

    println!("{:?}", sensible_valves);
    println!("{:?}", connections);

    let path = all_valves[&input.0].find_best_path(
        &all_valves,
        &connections,
        &sensible_valves.values().map(|v| &v.name).collect(),
        0,
        30,
        0,
        0,
        0,
    );
    path
}

#[aoc(day16, part2)]
fn part2(input: &Parsed) -> usize {
    todo!()
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
        assert_eq!(part2(&parse_input(input())), 0);
    }
}
