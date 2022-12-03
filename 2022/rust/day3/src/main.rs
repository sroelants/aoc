use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

////////////////////////////////////////////////////////////////////////////////
// Data definitions
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Item {
    item: char,
}

impl Item {
    fn priority(&self) -> u32 {
        if self.item.is_ascii_lowercase() {
            u32::from(self.item) - 96
        } else {
            u32::from(self.item) - 38
        }
    }
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        Item { item: c }
    }
}

#[derive(Debug, Clone)]
struct Backpack {
    left: HashSet<Item>,
    right: HashSet<Item>,
}

impl Backpack {
    fn all(&self) -> HashSet<Item> {
        &self.left | &self.right
    }

    fn common_object(&self) -> Option<Item> {
        let common = &self.left & &self.right;

        common.into_iter().next()
    }
}

impl From<&str> for Backpack {
    fn from(input: &str) -> Backpack {
        let compartment_size: usize = input.len() / 2;

        let left: HashSet<Item> = input[0..compartment_size]
            .chars()
            .map(|c| Item::from(c))
            .collect();

        let right: HashSet<Item> = input[compartment_size..(compartment_size * 2)]
            .chars()
            .map(|c| Item::from(c))
            .collect();

        Backpack { left, right }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Parsing
////////////////////////////////////////////////////////////////////////////////
fn parse_input(input: &str) -> Vec<Backpack> {
    input.lines().map(|line| Backpack::from(line)).collect()
}

////////////////////////////////////////////////////////////////////////////////
// Solutions
////////////////////////////////////////////////////////////////////////////////

fn main() {
    let input = read_to_string("./src/input.txt").expect("Failed to read input");
    let input = parse_input(&input);

    println!("Solution to part1: {}", part1(&input));
    println!("Solution to part2: {}", part2(&input));
}

fn part1(input: &Vec<Backpack>) -> u32 {
    input
        .into_iter()
        .map(|backpack| backpack.common_object().expect("No item in common"))
        // Convert into numerical value
        .map(|item| item.priority())
        .sum()
}

fn part2(input: &Vec<Backpack>) -> u32 {
    let groups = input.into_iter().tuples::<(_, _, _)>();
    groups
        // Find common intersection using bitwise and on &HashSet
        .map(|(one, two, three)| &(&one.all() & &two.all()) & &three.all())
        // Unwrap the item from the HashSet
        .map(|intersection| intersection.into_iter().next().expect("No item in common"))
        // Convert to a numerical value
        .map(|item| item.priority())
        .sum()
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[test]
fn priority() {
    assert_eq!(Item::from('a').priority(), 1, "'a' has priority 1");
    assert_eq!(Item::from('z').priority(), 26, "'z' has priority 26");
    assert_eq!(Item::from('A').priority(), 27, "'A' has priority 27");
    assert_eq!(Item::from('E').priority(), 31, "'E' has priority 31");
}

#[test]
fn backpack_from_str() {
    let backpack: Backpack = Backpack::from("abCD");

    assert!(backpack.left.contains(&Item { item: 'a' }));
    assert!(backpack.left.contains(&Item { item: 'b' }));
    assert!(backpack.right.contains(&Item { item: 'C' }));
    assert!(backpack.right.contains(&Item { item: 'D' }));

    assert!(!backpack.right.contains(&Item { item: 'a' }));
}

#[test]
fn part1_works() {
    let input = read_to_string("./src/test-input.txt").expect("Failed to read input");
    let input = parse_input(&input);
    assert_eq!(part1(&input), 157);
}

#[test]
fn part2_works() {
    let input = read_to_string("./src/test-input.txt").expect("Failed to read input");
    let input = parse_input(&input);
    assert_eq!(part2(&input), 70);
}
