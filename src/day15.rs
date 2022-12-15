use std::ops::Range;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

type Parsed = Vec<Sensor>;

static mut PART1_COORD_Y: i64 = 2000000;
static mut PART2_MAX_COORD_XY: i64 = 4000000;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Sensor {
    sensor_location: Point,
    beacon_location: Point,
    range: i64,
}

impl Sensor {
    fn coverage_at_y(&self, y: i64) -> Option<Range<i64>> {
        let relative_y = (y - self.sensor_location.y).abs();
        let half_width = self.range - relative_y;
        match half_width {
            _ if half_width < 0 => None,
            _ => Some(self.sensor_location.x - half_width..self.sensor_location.x + half_width + 1),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Parsed {
    let re = Regex::new("^.+x=(-?\\d+), y=(-?\\d+).+x=(-?\\d+), y=(-?\\d+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let sensor_location = Point {
                x: captures[1].parse().unwrap(),
                y: captures[2].parse().unwrap(),
            };
            let beacon_location = Point {
                x: captures[3].parse().unwrap(),
                y: captures[4].parse().unwrap(),
            };
            Sensor {
                sensor_location,
                beacon_location,
                range: sensor_location.manhattan_distance(&beacon_location) as i64,
            }
        })
        .collect()
}

#[aoc(day15, part1)]
fn part1(input: &Parsed) -> i64 {
    let coverages = input
        .iter()
        .filter_map(|sensor| sensor.coverage_at_y(unsafe { PART1_COORD_Y }))
        .collect_vec();

    let (covered_area, _) = covered_area(coverages);
    let beacons_in_area = input
        .iter()
        .filter(|sensor| sensor.beacon_location.y == unsafe { PART1_COORD_Y })
        .map(|sensor| sensor.beacon_location)
        .dedup()
        .count() as i64;

    covered_area - beacons_in_area
}

#[aoc(day15, part2)]
fn part2(input: &Parsed) -> i64 {
    let scan_range = 0..(unsafe { PART2_MAX_COORD_XY } + 1);

    let beacon_pos = scan_range
        .clone()
        .rev()
        .map(|y| {
            let coverages = input
                .iter()
                .filter_map(|x| x.coverage_at_y(y))
                .collect_vec();
            let (_, parts) = covered_area(coverages);

            parts
                .iter()
                .filter(|p| p.start <= scan_range.end && p.end >= scan_range.start)
                .sorted_by(|a, b| a.start.cmp(&b.start))
                .tuple_windows()
                .find(|(a, b)| a.end != b.start)
                .map(|(a, _)| Point { x: a.end, y })
        })
        .find_map(|p| p)
        .unwrap();
    let tuning_frequency = beacon_pos.x * 4000000 + beacon_pos.y;
    tuning_frequency
}

fn covered_area(mut ranges: Vec<Range<i64>>) -> (i64, Vec<Range<i64>>) {
    let mut covered_parts = Vec::<Range<i64>>::new();
    'next_range: while let Some(mut range) = ranges.pop() {
        for covered in &covered_parts {
            let overlap = range.start.max(covered.start)..range.end.min(covered.end);
            if overlap.start >= overlap.end {
                continue;
            }

            match overlap {
                _ if covered.start <= range.start && covered.end >= range.end => {
                    // full range already covered
                    continue 'next_range;
                }
                _ if range.start <= covered.start && range.end >= covered.end => {
                    // cut away middle
                    ranges.push(range.start..covered.start);
                    ranges.push(covered.end..range.end);
                    continue 'next_range;
                }
                _ if covered.start <= range.start => range = covered.end..range.end, // cut away start
                _ if covered.end >= range.end => range = range.start..covered.start, // cut away end
                _ => (),
            };
        }

        if range.start < range.end {
            covered_parts.push(range);
        }
    }

    let area = covered_parts.iter().fold(0, |a, x| a + x.end - x.start);
    (area, covered_parts)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    fn input<'a>() -> &'a str {
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
    }

    #[test]
    fn sample1() {
        unsafe {
            PART1_COORD_Y = 10;
        }
        assert_eq!(part1(&parse_input(input())), 26);
    }

    #[test]
    fn input1() {
        assert_eq!(
            part1(&parse_input(
                read_to_string("input/2022/day15.txt").unwrap().as_str()
            )),
            4985193
        );
    }

    #[test]
    fn sample2() {
        unsafe {
            PART2_MAX_COORD_XY = 20;
        };
        assert_eq!(part2(&parse_input(input())), 56000011);
    }

    #[test]
    fn input2() {
        assert_eq!(
            part2(&parse_input(
                read_to_string("input/2022/day15.txt").unwrap().as_str()
            )),
            11583882601918
        );
    }
}
