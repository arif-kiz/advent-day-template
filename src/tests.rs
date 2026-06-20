use super::{solve_1, solve_2, parse, parse::{Answer, Question}};

fn parse_input() -> Question {
    let input = "";
    parse(input)
}

#[test]
fn test_1() {
    assert_eq!(solve_1(parse_input()), Answer(0));
}

#[test]
fn test_2() {
    assert_eq!(solve_2(parse_input()), Answer(0));
}