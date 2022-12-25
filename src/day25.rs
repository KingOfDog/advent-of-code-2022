use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<String>;

fn snafu_digit_value(c: char) -> Result<i64, ()> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '-' => Ok(-1),
        '=' => Ok(-2),
        _ => Err(()),
    }
}

fn snafu_to_decimal(s: &str) -> i64 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| snafu_digit_value(c).unwrap() * (5 as i64).pow(i as u32))
        .sum()
}

fn decimal_to_snafu_digit(d: i64) -> Result<char, ()> {
    match d {
        -2 => Ok('='),
        -1 => Ok('-'),
        0 => Ok('0'),
        1 => Ok('1'),
        2 => Ok('2'),
        _ => Err(()),
    }
}

fn decimal_to_snafu(d: i64) -> String {
    let mut digits = vec![];

    let mut remaining = d;
    let mut carry_over = 0;
    while remaining > 0 {
        let mut next_digit = carry_over + remaining % 5;
        if next_digit > 2 {
            next_digit -= 5;
            carry_over = 1;
        } else {
            carry_over = 0;
        }
        digits.push(decimal_to_snafu_digit(next_digit).unwrap());
        remaining /= 5;
    }
    if carry_over > 0 || digits.len() == 0 {
        digits.push(decimal_to_snafu_digit(carry_over).unwrap());
    }

    digits.iter().rev().collect()
}

#[aoc_generator(day25)]
fn parse_input(input: &str) -> Parsed {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day25, part1)]
fn part1(input: &Parsed) -> String {
    decimal_to_snafu(input.iter().map(|line| snafu_to_decimal(line)).sum())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn input<'a>() -> &'a str {
        "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"
    }

    #[rstest]
    #[case("0", 0)]
    #[case("1", 1)]
    #[case("2", 2)]
    #[case("1=", 3)]
    #[case("1-", 4)]
    #[case("10", 5)]
    #[case("11", 6)]
    #[case("12", 7)]
    #[case("2=", 8)]
    #[case("2-", 9)]
    #[case("20", 10)]
    #[case("1=0", 15)]
    #[case("1-0", 20)]
    #[case("1=11-2", 2022)]
    #[case("1-0---0", 12345)]
    #[case("1121-1110-1=0", 314159265)]
    fn test_snafu_to_decimal(#[case] input: &str, #[case] output: i64) {
        assert_eq!(snafu_to_decimal(input), output);
        assert_eq!(decimal_to_snafu(output), input);
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), "2=-1=0");
    }
}
