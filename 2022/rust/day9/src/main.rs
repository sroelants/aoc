use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn adjacent(&self, other: &Self) -> bool {
        i32::max((self.x - other.x).abs(), (self.y - other.y).abs()) <= 1
    }

    fn approach(&mut self, other: &Self) {
        if !self.adjacent(other) {
            self.x -= (self.x - other.x).signum();
            self.y -= (self.y - other.y).signum();
        }
    }

    fn go(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Up => self.y += 1,
            Instruction::Down => self.y -= 1,
            Instruction::Left => self.x -= 1,
            Instruction::Right => self.x += 1,
        };
    }
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Instruction, Self::Error> {
        match value {
            "U" => Ok(Instruction::Up),
            "D" => Ok(Instruction::Down),
            "L" => Ok(Instruction::Left),
            "R" => Ok(Instruction::Right),
            _ => Err("Could not parse instruction!"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .flat_map(|line| {
            let mut iter = line.split(" ");
            let direction = iter.next().map(Instruction::try_from).unwrap().unwrap();
            let count = iter
                .next()
                .map(|count| count.parse::<usize>())
                .unwrap()
                .unwrap();

            std::iter::repeat(direction).take(count)
        })
        .collect()
}

fn main() {
    let input = read_to_string("./src/input.txt").unwrap();
    println!("Solution to part 1 is: {}", part1(&input));
    println!("Solution to part 2 is: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let instructions = parse_input(&input);
    let mut head = Position::default();
    let mut tail = Position::default();
    let mut positions: HashSet<Position> = HashSet::from([tail]);

    for instruction in instructions {
        head.go(&instruction);
        tail.approach(&head);
        positions.insert(tail);
    }

    positions.len() as u32
}

fn part2(input: &str) -> u32 {
    let instructions = parse_input(&input);
    let mut knots = (0..10).map(|_| Position::default()).collect::<Vec<_>>();
    // Instantiate the positions set with the tail position (knot 9)
    let mut positions: HashSet<Position> = HashSet::from([knots[9]]);

    for instruction in instructions {
        knots[0].go(&instruction);

        for i in 1..10 {
            let prev = knots[i - 1].clone();
            knots[i].approach(&prev);
        }

        positions.insert(knots[9]);
    }

    positions.len() as u32
}

#[test]
fn part1_works() {
    let input = read_to_string("./src/test-input.txt").unwrap();
    assert_eq!(part1(&input), 13);
}

#[test]
fn part2_works() {
    let input = read_to_string("./src/test-input2.txt").unwrap();
    assert_eq!(part2(&input), 36);
}
