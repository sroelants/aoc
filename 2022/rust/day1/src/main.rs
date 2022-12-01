use nom::character::complete::{i32, newline};
use nom::combinator::map;
use nom::multi::{count, separated_list1};
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedInput {
    data: Vec<Vec<i32>>,
}

type ParseResult<'a, T> = nom::IResult<&'a str, T>;

#[allow(dead_code)]
const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

fn main() {
    let input = read_to_string("./src/input.txt").unwrap();
    let (_, input) = parse_input(&input).expect("Malformed input format");

    println!("Solution to part 1: {}", part1(&input));
    println!("Solution to part 2: {}", part2(&input));
}

fn part1(input: &ParsedInput) -> i32 {
    input
        .data
        .iter()
        .map(|v| v.into_iter().sum())
        .max()
        .unwrap()
}

fn part2(input: &ParsedInput) -> i32 {
    use itertools::Itertools;

    input
        .data
        .iter()
        .map(|v| v.into_iter().sum())
        .sorted_by(|a: &i32, b: &i32| b.cmp(a))
        .take(3)
        .sum()
}

fn parse_input(input: &str) -> ParseResult<ParsedInput> {
    /// Parse a single number (i32)
    fn number(input: &str) -> ParseResult<i32> {
        i32(input)
    }

    /// Parse a single list of newline-separated numbers
    fn list(input: &str) -> ParseResult<Vec<i32>> {
        separated_list1(newline, number)(input)
    }

    /// Parse a list of lists of numbers, separated by a blank line ("\n\n")
    fn lists(input: &str) -> ParseResult<Vec<Vec<i32>>> {
        separated_list1(count(newline, 2), list)(input)
    }

    map(lists, |parsed| ParsedInput { data: parsed })(input)
}

//
// Tests
//

#[test]
fn parse_input_10() {
    let (_, result) = parse_input("10").unwrap();
    assert_eq!(result.data, vec![vec![10]]);
}

#[test]
fn parse_input_1_2_3() {
    let (_, result) = parse_input("1\n2\n3").unwrap();
    assert_eq!(result.data, vec![vec![1, 2, 3]]);
}

#[test]
fn parse_input_1_2_and_3() {
    let (_, result) = parse_input("1\n2\n\n3").unwrap();
    assert_eq!(result.data, vec![vec![1, 2], vec![3]]);
}

#[test]
fn part1_test_input() {
    let (_, test_input) = parse_input(TEST_INPUT).unwrap();
    let result = part1(&test_input);

    assert_eq!(result, 24000);
}

#[test]
fn part2_test_input() {
    let (_, test_input) = parse_input(TEST_INPUT).unwrap();
    let result = part2(&test_input);

    assert_eq!(result, 45000);
}
