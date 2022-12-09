/*
 * Data structures
 */

use std::fs::read_to_string;

#[derive(Debug)]
struct TreeVisibility {
    height: u32,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl TreeVisibility {
    fn new(height: u32) -> TreeVisibility {
        TreeVisibility {
            height,
            up: true,
            down: true,
            left: true,
            right: true,
        }
    }
}

#[derive(Debug)]
struct TreeDistances {
    up: u32,
    down: u32,
    left: u32,
    right: u32,
}

impl TreeDistances {
    fn new() -> TreeDistances {
        TreeDistances {
            up: 0,
            down: 0,
            left: 0,
            right: 0,
        }
    }

    fn score(&self) -> u32 {
        self.up * self.down * self.left * self.right
    }
}

/*
 * Parsing
 */
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn main() {
    let input = read_to_string("./src/input.txt").unwrap();
    println!("The solution to part 1 is: {}", part1(&input));
    println!("The solution to part 2 is: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let trees = parse_input(input);

    get_visibilities(trees)
        .iter()
        .flat_map(|trees| trees.iter())
        .filter(|tree| tree.up || tree.down || tree.left || tree.right)
        .count()
}

fn part2(input: &str) -> u32 {
    let trees = parse_input(input);
    let distances = get_distances(trees);

    distances
        .iter()
        .flat_map(|row| row.iter())
        .map(|distances| distances.score())
        .max()
        .unwrap()
}

fn get_visibilities(trees: Vec<Vec<u32>>) -> Vec<Vec<TreeVisibility>> {
    let mut visibilities: Vec<Vec<TreeVisibility>> = trees
        .into_iter()
        .map(|trees| trees.into_iter().map(TreeVisibility::new).collect())
        .collect();

    // Set top-down and left-to-right visibilities
    let mut left_right_max: Vec<u32> = vec![0; visibilities.len()];
    let mut top_bottom_max: Vec<u32> = vec![0; visibilities[0].len()];

    for (i, row) in visibilities.iter_mut().enumerate() {
        for (j, tree) in row.iter_mut().enumerate() {
            tree.left = j == 0 || tree.height > left_right_max[i];
            tree.up = i == 0 || tree.height > top_bottom_max[j];

            if tree.height > left_right_max[i] {
                left_right_max[i] = tree.height;
            }

            if tree.height > top_bottom_max[j] {
                top_bottom_max[j] = tree.height;
            }
        }
    }

    // Set right-to-left and bottom-to-top visibilities
    let mut right_left_max: Vec<u32> = vec![0u32; visibilities.len()];
    let mut bottom_top_max: Vec<u32> = vec![0u32; visibilities[0].len()];
    for (i, row) in visibilities.iter_mut().rev().enumerate() {
        for (j, tree) in row.iter_mut().rev().enumerate() {
            tree.right = i == 0 || tree.height > right_left_max[i];
            tree.down = j == 0 || tree.height > bottom_top_max[j];

            if tree.height > right_left_max[i] {
                right_left_max[i] = tree.height;
            }

            if tree.height > bottom_top_max[j] {
                bottom_top_max[j] = tree.height;
            }
        }
    }

    visibilities
}

fn get_distances(trees: Vec<Vec<u32>>) -> Vec<Vec<TreeDistances>> {
    let mut distances: Vec<Vec<TreeDistances>> = trees
        .iter()
        .map(|trees| trees.into_iter().map(|_| TreeDistances::new()).collect())
        .collect();

    for (i, row) in trees.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            // find distance to equal-height tree above by walking up the grid
            for i2 in (0..i).rev() {
                distances[i][j].up += 1;

                if trees[i2][j] >= *tree {
                    break;
                }
            }

            // Walking down
            for i2 in i + 1..trees.len() {
                distances[i][j].down += 1;

                if trees[i2][j] >= *tree {
                    break;
                }
            }

            // Walking left
            for j2 in (0..j).rev() {
                distances[i][j].left += 1;

                if trees[i][j2] >= *tree {
                    break;
                }
            }

            // Walking right
            for j2 in j + 1..row.len() {
                distances[i][j].right += 1;

                if trees[i][j2] >= *tree {
                    break;
                }
            }
        }
    }

    dbg!(distances)
}

#[test]
fn part1_works() {
    let input = read_to_string("./src/test-input.txt").unwrap();
    assert_eq!(part1(&input), 21);
}

#[test]
fn part2_works() {
    let input = read_to_string("./src/test-input.txt").unwrap();
    assert_eq!(part2(&input), 8);
}
