use std::{fs::read_to_string, num::ParseIntError};

#[derive(Debug, Eq, PartialEq)]
struct Assignment {
    upper: u32,
    lower: u32,
}

impl Assignment {
    fn contains(&self, point: u32) -> bool {
        self.lower <= point && self.upper >= point
    }

    fn partially_overlaps(&self, other: &Self) -> bool {
        self.contains(other.upper)
            || self.contains(other.lower)
            || other.contains(self.upper)
            || other.contains(self.lower)
    }

    fn fully_overlaps(&self, other: &Self) -> bool {
        self.contains(other.lower) && self.contains(other.upper)
            || other.contains(self.lower) && other.contains(self.upper)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Pair(Assignment, Assignment);

impl TryFrom<&str> for Pair {
    type Error = ParseIntError;

    fn try_from(input: &str) -> Result<Pair, ParseIntError> {
        let mut ranges = input.split(",");
        let first: Assignment = ranges.next().unwrap().try_into()?;
        let second: Assignment = ranges.next().unwrap().try_into()?;

        Ok(Pair(first, second))
    }
}

impl TryFrom<&str> for Assignment {
    type Error = ParseIntError;

    fn try_from(input: &str) -> Result<Assignment, ParseIntError> {
        let mut boundaries = input.split('-');
        let lower = boundaries.next().unwrap().parse::<u32>()?;
        let upper = boundaries.next().unwrap().parse::<u32>()?;
        Ok(Assignment { upper, lower })
    }
}

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(Pair::try_from)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn main() {
    let input = read_to_string("./src/input.txt").expect("Failed to read file");
    let input = parse_input(&input);
    println!("Solution to part1 is {}", part1(&input));
    println!("Solution to part2 is {}", part2(&input));
}

fn part1(input: &Vec<Pair>) -> usize {
    input
        .into_iter()
        .filter(|Pair(ass1, ass2)| ass1.fully_overlaps(ass2))
        .count()
}

fn part2(input: &Vec<Pair>) -> usize {
    input
        .into_iter()
        .filter(|Pair(ass1, ass2)| ass1.partially_overlaps(ass2))
        .count()
}

#[test]
fn parse_valid_assignment() {
    assert_eq!(
        Assignment::try_from("1-3").unwrap(),
        Assignment { lower: 1, upper: 3 }
    );
}

#[test]
#[should_panic]
fn parse_invalid_assignment() {
    Assignment::try_from("1-lol").unwrap();
}

#[test]
fn parse_pair() {
    assert_eq!(
        Pair::try_from("1-2,2-3").unwrap(),
        Pair(
            Assignment { lower: 1, upper: 2 },
            Assignment { lower: 2, upper: 3 }
        )
    );
}

#[test]
fn part1_works() {
    let input = read_to_string("./src/test-input.txt").expect("Failed to read file");
    let input = parse_input(&input);
    assert_eq!(part1(&input), 2);
}

#[test]
fn part2_works() {
    let input = read_to_string("./src/test-input.txt").expect("Failed to read file");
    let input = parse_input(&input);
    assert_eq!(part2(&input), 4);
}
