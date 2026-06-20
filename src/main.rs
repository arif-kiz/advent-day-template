mod parse;
mod solve1;
mod solve2;

use solve1::solve_1;
use solve2::solve_2;
use crate::parse::{parse, Question, Answer};

fn main() {
    let input = include_str!("input.txt");
    let parsed = parse(input);
    println!("Answer 1: {}", solve_1(parsed));
    println!("Answer 2: {}", solve_2(parsed));
}

#[cfg(test)]
mod tests;