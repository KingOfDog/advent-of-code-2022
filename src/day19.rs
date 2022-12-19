use std::{
    cmp::Ordering,
    collections::{HashSet, LinkedList},
    ops::{AddAssign, SubAssign},
};

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use regex::Regex;

type Parsed = Vec<Blueprint>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.ore == other.ore
            && self.clay == other.clay
            && self.obsidian == other.obsidian
            && self.geodes == other.geodes
        {
            Some(Ordering::Equal)
        } else if self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geodes >= other.geodes
        {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geodes += rhs.geodes;
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geodes -= rhs.geodes;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Bot {
    requirements: Resources,
    returns: Resources,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Blueprint {
    id: usize,
    ore_bot: Bot,
    clay_bot: Bot,
    obsidian_bot: Bot,
    geode_bot: Bot,
}

impl Blueprint {
    fn bots(&self) -> [&Bot; 4] {
        [
            &self.ore_bot,
            &self.clay_bot,
            &self.obsidian_bot,
            &self.geode_bot,
        ]
    }

    fn max_resources_cost(&self) -> Resources {
        let bots = self.bots();
        let max_ore = bots.iter().map(|bot| bot.requirements.ore).max().unwrap();
        let max_clay = bots.iter().map(|bot| bot.requirements.clay).max().unwrap();
        let max_obsidian = bots
            .iter()
            .map(|bot| bot.requirements.obsidian)
            .max()
            .unwrap();
        Resources {
            ore: max_ore,
            clay: max_clay,
            obsidian: max_obsidian,
            geodes: i32::MAX,
        }
    }
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Parsed {
    let pattern = Regex::new("^Blueprint (\\d+): Each ore robot costs (\\d+) ore. Each clay robot costs (\\d+) ore. Each obsidian robot costs (\\d+) ore and (\\d+) clay. Each geode robot costs (\\d+) ore and (\\d+) obsidian.$").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = pattern
                .captures(line)
                .expect("blueprint line does not match pattern");
            let blueprint_id = captures[1].parse().unwrap();
            let ore_bot = Bot {
                requirements: Resources {
                    ore: captures[2].parse().unwrap(),
                    clay: 0,
                    obsidian: 0,
                    geodes: 0,
                },
                returns: Resources {
                    ore: 1,
                    clay: 0,
                    obsidian: 0,
                    geodes: 0,
                },
            };
            let clay_bot = Bot {
                requirements: Resources {
                    ore: captures[3].parse().unwrap(),
                    clay: 0,
                    obsidian: 0,
                    geodes: 0,
                },
                returns: Resources {
                    ore: 0,
                    clay: 1,
                    obsidian: 0,
                    geodes: 0,
                },
            };
            let obsidian_bot = Bot {
                requirements: Resources {
                    ore: captures[4].parse().unwrap(),
                    clay: captures[5].parse().unwrap(),
                    obsidian: 0,
                    geodes: 0,
                },
                returns: Resources {
                    ore: 0,
                    clay: 0,
                    obsidian: 1,
                    geodes: 0,
                },
            };
            let geode_bot = Bot {
                requirements: Resources {
                    ore: captures[6].parse().unwrap(),
                    clay: 0,
                    obsidian: captures[7].parse().unwrap(),
                    geodes: 0,
                },
                returns: Resources {
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geodes: 1,
                },
            };

            Blueprint {
                id: blueprint_id,
                ore_bot,
                clay_bot,
                obsidian_bot,
                geode_bot,
            }
        })
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
struct State {
    current_resources: Resources,
    resources_per_minute: Resources,
    time_remaining: i32,
}

fn number_of_geodes(blueprint: &Blueprint, time_remaining: i32) -> i32 {
    let mut seen_states: HashSet<State> = HashSet::new();
    let start_state = State {
        current_resources: Resources::default(),
        resources_per_minute: Resources {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        },
        time_remaining: time_remaining,
    };

    let mut queue = LinkedList::new();
    queue.push_back(start_state);

    let mut max_geodes = 0;

    while let Some(state) = queue.pop_front() {
        let State {
            current_resources,
            resources_per_minute,
            time_remaining,
        } = state;

        max_geodes = max_geodes.max(current_resources.geodes);

        if time_remaining <= 0 {
            continue;
        }

        let max_resources = blueprint.max_resources_cost();
        let ore_bots = resources_per_minute.ore.min(max_resources.ore);
        let clay_bots = resources_per_minute.clay.min(max_resources.clay);
        let obsidian_bots = resources_per_minute.obsidian.min(max_resources.obsidian);

        let ore = current_resources
            .ore
            .min((time_remaining * max_resources.ore) - (ore_bots * (time_remaining - 1)));
        let clay = current_resources
            .clay
            .min((time_remaining * max_resources.clay) - (clay_bots * (time_remaining - 1)));
        let obsidian = current_resources.obsidian.min(
            (time_remaining * max_resources.obsidian) - (obsidian_bots * (time_remaining - 1)),
        );

        let state = State {
            current_resources: Resources {
                ore,
                clay,
                obsidian,
                geodes: current_resources.geodes,
            },
            resources_per_minute: Resources {
                ore: ore_bots,
                clay: clay_bots,
                obsidian: obsidian_bots,
                geodes: resources_per_minute.geodes,
            },
            time_remaining,
        };
        if seen_states.contains(&state) {
            continue;
        }

        seen_states.insert(state);

        let available_resources = state.current_resources;

        let mut state = state;
        state.current_resources += state.resources_per_minute;
        state.time_remaining -= 1;

        queue.push_back(state);

        blueprint
            .bots()
            .iter()
            .filter(|bot| available_resources >= bot.requirements)
            .for_each(|bot| {
                let mut state = state;
                state.current_resources -= bot.requirements;
                state.resources_per_minute += bot.returns;
                queue.push_back(state);
            });
    }

    max_geodes
}

#[aoc(day19, part1)]
fn part1(input: &Parsed) -> i32 {
    input
        .par_iter()
        .map(|blueprint| number_of_geodes(blueprint, 24) * blueprint.id as i32)
        .sum()
}

#[aoc(day19, part2)]
fn part2(input: &Parsed) -> i32 {
    input
        .par_iter()
        .take(3)
        .map(|blueprint| number_of_geodes(blueprint, 32))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 33);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 56 * 62);
    }
}
