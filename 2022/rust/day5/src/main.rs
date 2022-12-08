use std::fs::read_to_string;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{anychar, char, newline, u32};
use nom::combinator::map;
use nom::multi::{count, many1, separated_list1};
use nom::sequence::{delimited, terminated, tuple};

/*
 * Data types
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Container(char);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Instruction {
    from: u32,
    to: u32,
    count: u32,
}

/*
 * Parsing
 */
type ParseResult<'a, T> = nom::IResult<&'a str, T>;

fn container_or_gap(input: &str) -> ParseResult<Option<Container>> {
    let gap = map(count(char(' '), 3), |_| None);
    let container = map(delimited(char('['), anychar, char(']')), |ch| {
        Some(Container(ch))
    });

    alt((container, gap))(input)
}

fn row(input: &str) -> ParseResult<Vec<Option<Container>>> {
    terminated(separated_list1(char(' '), container_or_gap), char('\n'))(input)
}

fn containers(input: &str) -> ParseResult<Vec<Vec<Option<Container>>>> {
    many1(row)(input)
}

fn line(input: &str) -> ParseResult<&str> {
    terminated(take_until("\n"), newline)(input)
}

fn instruction(input: &str) -> ParseResult<Instruction> {
    map(
        tuple((tag("move "), u32, tag(" from "), u32, tag(" to "), u32)),
        |(_, count, _, from, _, to)| Instruction { count, from, to },
    )(input)
}

fn instructions(input: &str) -> ParseResult<Vec<Instruction>> {
    separated_list1(newline, instruction)(input)
}

fn parse_input(input: &str) -> (Vec<Vec<Container>>, Vec<Instruction>) {
    let (input, containers) = containers(input).unwrap();

    // Skip the next two lines
    let (input, _) = line(input).unwrap();
    let (input, _) = line(input).unwrap();

    let (_, instructions) = instructions(input).unwrap();

    (transpose(containers), instructions)
}

/*
* Solutions
*/
fn main() {
    let input = read_to_string("./src/input.txt").unwrap();
    println!("The solution to part 1 is {}", part1(&input));
    println!("The solution to part 2 is {}", part2(&input));
}

fn part1(input: &str) -> String {
    let (mut containers, instructions) = parse_input(input);

    for instruction in instructions {
        let Instruction { from, to, count } = instruction;
        let from_col_len = containers[from as usize - 1].len();

        let taken = containers[from as usize - 1]
            .drain(from_col_len - count as usize..)
            .rev()
            .collect::<Vec<_>>();

        for container in taken {
            containers[to as usize - 1].push(container.to_owned());
        }
    }

    containers
        .into_iter()
        .filter_map(|col| col.to_owned().pop())
        .map(|Container(c)| c)
        .collect()
}

fn part2(input: &str) -> String {
    let (mut containers, instructions) = parse_input(input);

    for instruction in instructions {
        let Instruction { from, to, count } = instruction;
        let from_col = containers[from as usize - 1].clone();

        let taken = containers[from as usize - 1]
            .drain(from_col.len() - count as usize..)
            .collect::<Vec<_>>();

        for container in taken {
            containers[to as usize - 1].push(container.to_owned());
        }
    }

    containers
        .into_iter()
        .filter_map(|col| col.to_owned().pop())
        .map(|Container(c)| c)
        .collect()
}

/*
 * utilities
 */
/// Transpose a Vec of Vecs from describing a collection of rows to a
/// collection of columns
///
/// [[None, 2], -> [[1   ],
///  [1,    3]] ->  [3, 2]]
fn transpose<T>(vecs: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    let columns = vecs[0].len();
    let mut transposed: Vec<Vec<T>> = (0..columns).map(|_| vec![]).collect();

    for row in vecs.into_iter().rev() {
        for (j, col) in row.into_iter().enumerate() {
            if let Some(item) = col {
                transposed[j].push(item);
            }
        }
    }

    transposed
}

/*
 * Tests
 */

#[test]
fn part1_works() {
    let input = read_to_string("./src/test-input.txt").unwrap();
    assert_eq!(part1(&input), "CMZ");
}

#[test]
fn part2_works() {
    let input = read_to_string("./src/test-input.txt").unwrap();
    assert_eq!(part2(&input), "MCD");
}

#[test]
fn transpose_matrix() {
    let matrix = vec![vec![None, Some(2)], vec![Some(1), Some(3)]];
    assert_eq!(transpose(matrix), vec![vec![1], vec![3, 2]]);
}

#[test]
fn parse_container() {
    let (_, container) = container_or_gap("[A]").unwrap();
    assert_eq!(container, Some(Container('A')))
}

#[test]
fn parse_row() {
    let (_, containers) = row("    [A] [B]\n").unwrap();

    assert_eq!(
        containers,
        vec![None, Some(Container('A')), Some(Container('B'))]
    );
}

#[test]
fn parse_instruction() {
    let (_, instruction) = instruction("move 1 from 2 to 3").unwrap();

    assert_eq!(
        instruction,
        Instruction {
            from: 2,
            to: 3,
            count: 1
        }
    );
}
