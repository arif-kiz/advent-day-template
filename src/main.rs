mod parse;
mod solve1;
mod solve2;

#[allow(unused)]

use solve1::solve_1;
use solve2::solve_2;
use crate::parse::parse;

fn main() {
    let input = include_str!("input.txt");
    let parsed = parse(input);
    cfg_if::cfg_if! {
        if #[cfg(feature="a1")] {
            dbg!(solve_1(parsed));
        } else {
            dbg!(solve_2(parsed));
        }
    }
}

#[cfg(test)]
mod tests;