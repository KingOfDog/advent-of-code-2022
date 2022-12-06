use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Parsed = String;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Parsed {
    String::from(input)
}

fn only_unique_characters(input: &[char]) -> bool {
    let set: HashSet<_> = input.into_iter().collect();
    set.len() == input.len()
}

#[aoc(day6, part1)]
fn part1(input: &Parsed) -> usize {
    input
        .chars()
        .tuple_windows::<(char, char, char, char)>()
        .enumerate()
        .find(|(_, last_sequence)| {
            only_unique_characters(&[
                last_sequence.0,
                last_sequence.1,
                last_sequence.2,
                last_sequence.3,
            ])
        })
        .unwrap()
        .0
        + 4
}

#[aoc(day6, part2)]
fn part2(input: &Parsed) -> usize {
    let chars = input.chars().collect_vec();
    chars
        .windows(14)
        .enumerate()
        .find(|(_, last_sequence)| only_unique_characters(&last_sequence))
        .unwrap()
        .0
        + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_a<'a>() -> &'a str {
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
    }

    fn input_b<'a>() -> &'a str {
        "bvwbjplbgvbhsrlpgdmjqwftvncz"
    }

    fn input_c<'a>() -> &'a str {
        "nppdvjthqldpwncqszvftbrmjlhg"
    }

    fn input_d<'a>() -> &'a str {
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
    }

    fn input_e<'a>() -> &'a str {
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input_a())), 7);
        assert_eq!(part1(&parse_input(input_b())), 5);
        assert_eq!(part1(&parse_input(input_c())), 6);
        assert_eq!(part1(&parse_input(input_d())), 10);
        assert_eq!(part1(&parse_input(input_e())), 11);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input_a())), 19);
        assert_eq!(part2(&parse_input(input_b())), 23);
        assert_eq!(part2(&parse_input(input_c())), 23);
        assert_eq!(part2(&parse_input(input_d())), 29);
        assert_eq!(part2(&parse_input(input_e())), 26);
    }
}
