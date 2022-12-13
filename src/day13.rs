use std::{cmp::Ordering, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Parsed = Vec<(PacketValue, PacketValue)>;

#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketValue {
    List(Vec<PacketValue>),
    Int(u32),
}

impl PacketValue {
    fn is_lower_than(&self, other: &PacketValue) -> bool {
        self.compare(other) == Ordering::Less
    }

    fn compare(&self, other: &PacketValue) -> Ordering {
        match self {
            PacketValue::Int(value) => match other {
                PacketValue::Int(other_value) => value.cmp(other_value),
                PacketValue::List(_) => {
                    PacketValue::List(vec![PacketValue::Int(*value)]).compare(other)
                }
            },
            PacketValue::List(values) => match other {
                PacketValue::Int(other_value) => {
                    self.compare(&PacketValue::List(vec![PacketValue::Int(*other_value)]))
                }
                PacketValue::List(other_values) => {
                    for i in 0..values.len() {
                        if i >= other_values.len() {
                            return Ordering::Greater;
                        }

                        let ordering = values[i].compare(&other_values[i]);
                        if ordering != Ordering::Equal {
                            return ordering;
                        }
                    }

                    return Ordering::Less;
                }
            },
        }
    }
}

impl FromStr for PacketValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            let mut items = Vec::new();
            let mut current_item = Vec::new();
            let mut bracket_level = 0;
            let mut iter = s[1..s.len() - 1].chars();
            while let Some(c) = iter.next() {
                if c == '[' {
                    bracket_level += 1;
                } else if c == ']' {
                    bracket_level -= 1;
                }

                if bracket_level == 0 && c == ',' {
                    items.push(current_item.iter().collect::<String>().parse().unwrap());
                    current_item.clear();
                    continue;
                }

                current_item.push(c);
            }

            if !current_item.is_empty() {
                items.push(current_item.iter().collect::<String>().parse().unwrap());
            }

            Ok(PacketValue::List(items))
        } else {
            Ok(PacketValue::Int(s.parse().unwrap()))
        }
    }
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Parsed {
    input
        .split("\n\n")
        .map(|pair| {
            pair.lines()
                .map(|line| line.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(input: &Parsed) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left.is_lower_than(right))
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &Parsed) -> usize {
    let divider_one = PacketValue::List(vec![PacketValue::List(vec![PacketValue::Int(2)])]);
    let divider_two = PacketValue::List(vec![PacketValue::List(vec![PacketValue::Int(6)])]);
    let mut input = input.clone();
    input.append(&mut vec![(divider_one.clone(), divider_two.clone())]);
    let ordered = input
        .iter()
        .flat_map(|group| [group.0.clone(), group.1.clone()])
        .sorted_by(|a, b| a.compare(b))
        .collect_vec();

    let index_one = ordered.iter().position(|x| x == &divider_one).unwrap() + 1;
    let index_two = ordered.iter().position(|x| x == &divider_two).unwrap() + 1;

    index_one * index_two
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 13);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 140);
    }
}
