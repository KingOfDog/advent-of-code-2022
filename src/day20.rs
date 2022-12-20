use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Parsed = Vec<i128>;

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Parsed {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn encryption_mix(numbers: Vec<(usize, i128)>) -> Vec<(usize, i128)> {
    let mut numbers = numbers;
    let length = numbers.len();
    let max_index = length as i128 - 1;

    for i in 0..length {
        let index = numbers.iter().position(|(index, _)| *index == i).unwrap();
        let value = numbers.remove(index);
        let number = value.1;

        if number == 0 {
            numbers.insert(index, value);
            continue;
        }

        let mut new_index = index as i128 + number;
        if new_index <= 0 {
            new_index += (new_index.abs() / max_index + 1) * max_index;
        }
        if new_index >= length as i128 {
            new_index -= new_index / max_index * max_index;
            if new_index == 0 {
                new_index = max_index;
            }
        }

        numbers.insert(new_index as usize, value);
    }

    numbers
}

fn calculate_solution(numbers: &Vec<(usize, i128)>) -> i128 {
    let pos_0 = numbers.iter().position(|(_, x)| *x == 0).unwrap();

    let one = numbers[(pos_0 + 1000) % numbers.len()].1;
    let two = numbers[(pos_0 + 2000) % numbers.len()].1;
    let three = numbers[(pos_0 + 3000) % numbers.len()].1;

    one + two + three
}

#[aoc(day20, part1)]
fn part1(input: &Parsed) -> i128 {
    let numbers = input.iter().enumerate().map(|(i, x)| (i, *x)).collect_vec();
    let numbers = encryption_mix(numbers);

    calculate_solution(&numbers)
}

#[aoc(day20, part2)]
fn part2(input: &Parsed) -> i128 {
    let key = 811589153;
    let mut numbers = input
        .iter()
        .enumerate()
        .map(|(i, x)| (i, (x * key)))
        .collect_vec();

    for _ in 0..10 {
        numbers = encryption_mix(numbers);
    }

    calculate_solution(&numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "1
2
-3
3
-2
0
4"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 3);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 1623178306);
    }
}
