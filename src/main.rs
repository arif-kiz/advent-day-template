mod parse;
mod solve;

use solve::{solve_1, solve_2};
use crate::parse::{parse, Question, Answer};

fn main() {
    let input = include_str!("input.txt");
    let parsed = parse(input);
    println!("Answer 1: {}", solve_1(parsed));
    println!("Answer 2: {}", solve_2(parsed));
}

#[cfg(test)]
mod tests;