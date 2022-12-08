use std::{collections::HashSet, fs::read_to_string};

/*
 * Parsing
 */
fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

/*
* Solutions
*/
fn main() {
    let input = read_to_string("./src/input.txt").unwrap();
    println!("The solution to part 1 is {}", part1(&input));
    println!("The solution to part 2 is {}", part2(&input));
}

fn part1(input: &str) -> usize {
    find_unique_sequence(input, 4)
}

fn part2(input: &str) -> usize {
    find_unique_sequence(input, 14)
}

fn find_unique_sequence(input: &str, n: usize) -> usize {
    let chars = parse_input(input);

    &chars
        .windows(n)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == n)
        .unwrap()
        + n
}

/*
 * Tests
 */
#[test]
fn part1_works() {
    assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
}
