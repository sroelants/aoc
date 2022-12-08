use nom::branch::alt;
use nom::character::complete::{char, newline, space1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
struct ParsedInput {
    data: Vec<Round>,
}

#[derive(Debug, PartialEq, Eq)]
enum GameResult {
    Win,
    Lose,
    Tie,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn value(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn score(&self, other: &Self) -> i32 {
        if self > other {
            6
        } else if self == other {
            3
        } else {
            0
        }
    }

    fn wins_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn loses_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if &self.wins_against() == other {
            Ordering::Greater
        } else if self == &other.wins_against() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
    us: Hand,
    them: Hand,
}

#[derive(Debug, PartialEq, Eq)]
struct Score {
    us: i32,
    them: i32,
}

impl Round {
    fn score(&self) -> Score {
        Score {
            us: &self.us.value() + &self.us.score(&self.them),
            them: &self.them.value() + &self.them.score(&self.us),
        }
    }
}

type ParseResult<'a, T> = nom::IResult<&'a str, T>;

fn main() {
    let input = read_to_string("src/input.txt").expect("Failed to read file.");
    println!("Solution to part 1: {}", part1(&input));
    println!("Solution to part 2: {}", part2(&input));
}

pub fn part1(input: &str) -> i32 {
    let (_, input): (_, ParsedInput) = parse_input_part1(&input).expect("Malformed input");
    input
        .data
        .into_iter()
        .map(|round: Round| round.score().us)
        .sum()
}

pub fn part2(input: &str) -> i32 {
    let (_, input): (_, ParsedInput) = parse_input_part2(&input).expect("Malformed input");
    input
        .data
        .into_iter()
        .map(|round: Round| round.score().us)
        .sum()
}

////////////////////////////////////////////////////////////////////////////////
// Parsing
////////////////////////////////////////////////////////////////////////////////

fn my_hand(input: &str) -> ParseResult<Hand> {
    map(alt((char('X'), char('Y'), char('Z'))), |c| match c {
        'X' => Hand::Rock,
        'Y' => Hand::Paper,
        'Z' => Hand::Scissors,
        _ => unreachable!(),
    })(input)
}

fn their_hand(input: &str) -> ParseResult<Hand> {
    map(alt((char('A'), char('B'), char('C'))), |c| match c {
        'A' => Hand::Rock,
        'B' => Hand::Paper,
        'C' => Hand::Scissors,
        _ => unreachable!(),
    })(input)
}

fn result(input: &str) -> ParseResult<GameResult> {
    map(alt((char('X'), char('Y'), char('Z'))), |c| match c {
        'X' => GameResult::Lose,
        'Y' => GameResult::Tie,
        'Z' => GameResult::Win,
        _ => unreachable!(),
    })(input)
}

fn round(input: &str) -> ParseResult<Round> {
    map(separated_pair(their_hand, space1, my_hand), |(them, us)| {
        Round { us, them }
    })(input)
}

fn complicated_round(input: &str) -> ParseResult<Round> {
    map(
        separated_pair(their_hand, space1, result),
        |(them, result): (Hand, GameResult)| match result {
            GameResult::Tie => Round { us: them, them },

            GameResult::Win => Round {
                us: them.loses_against(),
                them,
            },

            GameResult::Lose => Round {
                us: them.wins_against(),
                them,
            },
        },
    )(input)
}

fn parse_input_part1(input: &str) -> ParseResult<ParsedInput> {
    map(separated_list1(newline, round), |rounds| ParsedInput {
        data: rounds,
    })(input)
}

fn parse_input_part2(input: &str) -> ParseResult<ParsedInput> {
    map(separated_list1(newline, complicated_round), |rounds| {
        ParsedInput { data: rounds }
    })(input)
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_round() {
        let (_, result) = super::round(&"B Z").unwrap();
        assert_eq!(
            result,
            Round {
                us: Hand::Scissors,
                them: Hand::Paper
            }
        );
    }

    #[test]
    fn hand_cmp() {
        assert!(Hand::Rock > Hand::Scissors, "Rock is greater than Scissors");
        assert!(Hand::Rock == Hand::Rock, "Rock is equal to Rock");
        assert!(Hand::Rock < Hand::Paper, "Rock is less than Paper");
        assert!(Hand::Paper < Hand::Scissors, "Paper is less than Scissors");
    }

    #[test]
    fn score() {
        assert_eq!(
            (Round {
                us: Hand::Rock,
                them: Hand::Scissors
            })
            .score(),
            Score { us: 7, them: 3 }
        );
    }

    #[test]
    fn part1_test_input() {
        let input = read_to_string("src/test-input.txt").unwrap();
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn part2_test_input() {
        let input = read_to_string("src/test-input.txt").unwrap();
        assert_eq!(part2(&input), 12);
    }
}
