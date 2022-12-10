#![feature(iter_array_chunks)]
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Instruction {
        match value {
            "noop" => Instruction::Noop,
            _ => {
                let instr = value.split(" ").collect::<Vec<_>>();
                let amount: i32 = instr[1].parse().unwrap(); // Error handling is for wimps
                return Instruction::Addx(amount);
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| Instruction::from(line)).collect()
}

fn part1(input: &str) -> i32 {
    let instructions = parse_input(&input);
    let register_values = get_register_values(instructions);

    register_values
        .iter()
        .enumerate()
        .map(|(i, val)| (i + 1, val)) // change indices to be 1-based
        .filter(|(i, _)| i % 40 == 20) // Only consider cycle 20, 60, 100, etc...
        .map(|(i, val)| (i as i32) * val) // Calculate score
        .sum()
}

fn get_register_values(instructions: Vec<Instruction>) -> Vec<i32> {
    let cycle_updates = instructions
        .into_iter()
        .flat_map(|instruction| match instruction {
            Instruction::Noop => vec![0].into_iter(),
            Instruction::Addx(val) => vec![0, val].into_iter(),
        });

    cycle_updates
        .scan(1, |acc, val| {
            let current = *acc;
            *acc = *acc + val;

            Some(current)
        })
        .collect()
}

fn part2(input: &str) -> String {
    let instructions = parse_input(input);
    let register_values = get_register_values(instructions);
    let clock_cycles = 0..279;
    let pixels = clock_cycles
        .zip(register_values.iter())
        // Convert to pixel values
        .map(|(cycle, register)| {
            if (register - 1..=register + 1).contains(&(cycle % 40)) {
                '#'
            } else {
                '.'
            }
        })
        .array_chunks::<40>() // Chunk into 40 character lines
        .map(|line_chars| line_chars.iter().collect::<String>()) // Collect 40 chars into string
        .collect::<Vec<_>>()
        .join("\n");

    pixels
}

fn main() {
    let input = read_to_string("./src/input.txt").unwrap();
    println!("Solution to part 1 is: {}", part1(&input));
    println!("Solution to part 2 is:\n{}", part2(&input));
}

#[test]
fn part1_works() {
    let input = read_to_string("./src/test-input.txt").unwrap();
    assert_eq!(part1(&input), 13140);
}
