/*
 * Data structures
 *
 * This is where I wish I was implementing this as a lisp...
 */

use std::{cmp::Ordering, fs::read_to_string};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    character::complete::u128,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Monkey {
    activity: u128,
    items: Vec<Item>,
    worry_update: Op,
    test: Test,
}

impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.activity.cmp(&other.activity)
    }
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.activity.cmp(&other.activity))
    }
}

impl Monkey {
    fn test(&self, item: &Item) -> usize {
        if item.0 % self.test.modulo == 0 {
            self.test.if_true as usize
        } else {
            self.test.if_false as usize
        }
    }

    fn update_items(&mut self, worry_factor: u128, modulus: u128) {
        self.items.iter_mut().for_each(|item| {
            self.activity += 1;

            let new_value = match &self.worry_update {
                Op::Add(num) => item.0 + num,
                Op::Mult(num) => item.0 * num,
                Op::Square => item.0 * item.0,
            } as u128;

            item.0 = (new_value / worry_factor as u128) % modulus;
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Add(u128),
    Mult(u128),
    Square,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Test {
    modulo: u128,
    if_true: u128,
    if_false: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item(u128);

/*
 * Parsing
 */

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, monkey)(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    preceded(
        delimited(tag("Monkey "), u128, tag(":\n")),
        tuple((items, operation, test)),
    )
    .map(|(items, operation, test)| Monkey {
        activity: 0,
        items,
        test,
        worry_update: operation,
    })
    .parse(input)
}

fn items(input: &str) -> IResult<&str, Vec<Item>> {
    delimited(
        tag("  Starting items: "),
        separated_list1(tag(", "), u128),
        newline,
    )
    .map(|levels| levels.into_iter().rev().map(Item).collect())
    .parse(input)
}

fn operation(input: &str) -> IResult<&str, Op> {
    delimited(
        tag("  Operation: new = "),
        alt((add_operation, mult_operation, square_operation)),
        newline,
    )(input)
}

fn add_operation(input: &str) -> IResult<&str, Op> {
    preceded(tag("old + "), u128).map(Op::Add).parse(input)
}

fn mult_operation(input: &str) -> IResult<&str, Op> {
    preceded(tag("old * "), u128).map(Op::Mult).parse(input)
}

fn square_operation(input: &str) -> IResult<&str, Op> {
    tag("old * old").map(|_| Op::Square).parse(input)
}

fn test(input: &str) -> IResult<&str, Test> {
    tuple((modulo, if_true, if_false))
        .map(|(modulo, if_true, if_false)| Test {
            modulo,
            if_true,
            if_false,
        })
        .parse(input)
}

fn modulo(input: &str) -> IResult<&str, u128> {
    delimited(tag("  Test: divisible by "), u128, newline)(input)
}

fn if_true(input: &str) -> IResult<&str, u128> {
    delimited(tag("    If true: throw to monkey "), u128, newline)(input)
}

fn if_false(input: &str) -> IResult<&str, u128> {
    delimited(tag("    If false: throw to monkey "), u128, newline)(input)
}

fn part1(input: &str) -> u128 {
    let (_, mut monkeys) = parse_input(&input).unwrap();

    for _ in 0..20 {
        play_round(&mut monkeys, 3);
    }

    monkeys.sort();

    monkeys
        .into_iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.activity)
        .product()
}

fn part2(input: &str) -> u128 {
    let (_, mut monkeys) = parse_input(&input).unwrap();

    for _ in 0..10000 {
        play_round(&mut monkeys, 1);
    }

    monkeys.sort();

    monkeys
        .into_iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.activity)
        .product()
}

fn play_round(monkeys: &mut Vec<Monkey>, denom: u128) {
    for idx in 0..monkeys.len() {
        play_turn(monkeys, idx, denom);
    }
}

fn play_turn(monkeys: &mut Vec<Monkey>, n: usize, denom: u128) {
    // SECRET SAUNCE: We want to prevent overflows in the products. Notice that
    // the only thing we really care about is the worry factor modulo the
    // test modulus. Also notice they are all prime (this is the lame
    // observation that I missed). Hence, we can store the values modulo the
    // product of all of these, and (by the Chinese Remainder Theorem, remember
    // that one?), everyone will still get the correct remainders when doing
    // their modulus checks.
    let crt_modulus = monkeys.iter().map(|monkey| monkey.test.modulo).product();
    monkeys[n].update_items(denom, crt_modulus);

    while &monkeys[n].items.len() > &0 {
        let item = monkeys[n].items.pop().unwrap();
        let recipient = monkeys[n].test(&item);
        monkeys[recipient].items.push(item);
    }
}

fn main() {
    let input = read_to_string("./src/input.txt").unwrap();
    println!("The solution to part 1 is: {}", part1(&input));
    println!("The solution to part 2 is: {}", part2(&input));
}

#[test]
fn part1_works() {
    let input = read_to_string("./src/test-input.txt").unwrap();
    assert_eq!(part1(&input), 10605);
}

#[test]
fn part2_works() {
    let input = read_to_string("./src/test-input.txt").unwrap();
    assert_eq!(part2(&input), 2713310158);
}
