/*
 * Okay, side-stepping the issue with building an entire graph:
 * We only care about the size of any given directory, so we can just walk over
 * the instructions and keep a hashmap of directories and their sizes.
 * When we get the contents of a directory, we store the size for that
 * directory, as well as all directories above it.
 *
 * It feels _way_ less satisfying, though!
 *
 * If we were going the tree approach: We need to keep a cursor/context around,
 * which should be a bunch of mutable references. Unless we just clone the fuck
 * out of the tree while folding over the instructions...
 */

use std::{collections::BTreeMap, fs::read_to_string};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, not_line_ending, space1, u32},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

/*
 * Data structures
 */
enum Node<'a> {
    Directory { name: &'a str },
    File { name: &'a str, size: u32 },
}

enum Command<'a> {
    Ls(Vec<Node<'a>>),
    Up,
    To(&'a str),
}

fn main() {
    let input = read_to_string("./src/input.txt").unwrap();
    println!("The solution to part 1 is: {}", part1(&input));
    println!("The solution to part 2 is: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let (_, commands) = parse_input(input).unwrap();
    let directory_sizes = chart_filesystem(commands);

    directory_sizes
        .into_iter()
        .map(|(_, size)| size)
        .filter(|&size| size <= 100_000)
        .sum()
}

fn part2(input: &str) -> u32 {
    let (_, commands) = parse_input(input).unwrap();
    let directory_sizes = chart_filesystem(commands);
    let available_space = 70_000_000 - used_space(&directory_sizes);

    directory_sizes
        .into_iter()
        .map(|(_, size)| size)
        .filter(|size| available_space + size >= 30_000_000)
        .min()
        .unwrap()
}

fn used_space(directory_sizes: &BTreeMap<String, u32>) -> u32 {
    *directory_sizes.get("/").unwrap()
}

fn chart_filesystem(commands: Vec<Command>) -> BTreeMap<String, u32> {
    let mut path_stack: Vec<String> = vec!["/".to_string()];
    let mut directory_sizes: BTreeMap<String, u32> = BTreeMap::new();

    for command in commands {
        match command {
            Command::To("/") => {
                path_stack = vec!["/".to_string()];
            }
            Command::Up => {
                path_stack.pop();
            }
            Command::To(dir) => {
                let current_path = path_stack.last().unwrap().clone();

                path_stack.push([current_path, dir.to_string()].join("/"));
            }
            Command::Ls(files) => {
                let additional_size = files
                    .iter()
                    .map(|node| {
                        if let Node::File { name: _, size } = node {
                            *size
                        } else {
                            0
                        }
                    })
                    .sum::<u32>();

                for path in &path_stack {
                    let current_size = directory_sizes.get(path).unwrap_or(&0);

                    directory_sizes.insert(path.to_owned(), current_size + additional_size);
                }
            }
        }
    }

    directory_sizes
}

fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(newline, command)(input)
}

fn command(input: &str) -> IResult<&str, Command> {
    alt((cd, ls))(input)
}

fn cd(input: &str) -> IResult<&str, Command> {
    map(preceded(tag("$ cd "), not_line_ending), |directory| {
        if directory == ".." {
            Command::Up
        } else {
            Command::To(directory)
        }
    })(input)
}

fn ls(input: &str) -> IResult<&str, Command> {
    map(
        preceded(tag("$ ls\n"), separated_list1(newline, fs_node)),
        |files| Command::Ls(files),
    )(input)
}

fn fs_node(input: &str) -> IResult<&str, Node> {
    alt((file, dir))(input)
}

fn file(input: &str) -> IResult<&str, Node> {
    map(
        separated_pair(u32, space1, not_line_ending),
        |(size, name)| Node::File { size, name },
    )(input)
}

fn dir(input: &str) -> IResult<&str, Node> {
    map(preceded(tag("dir "), not_line_ending), |name| {
        Node::Directory { name }
    })(input)
}

/*
 * Tests
 */
#[test]
fn part1_works() {
    let input = read_to_string("./src/test-input.txt").unwrap();
    assert_eq!(part1(&input), 95437);
}

#[test]
fn part2_works() {
    let input = read_to_string("./src/test-input.txt").unwrap();
    assert_eq!(part2(&input), 24933642);
}
