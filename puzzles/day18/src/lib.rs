// Day 18: Operation Order

use shared::prelude::*;
use std::{convert::TryFrom, iter::Peekable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Number(i64),
    ParenOpen,
    ParenClose,
    Operator(Operator),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Multiply,
}

lazy_static! {
    static ref PUZZLE_INPUT: Vec<&'static str> =
        puzzle_input::lines(include_str!("puzzle_input.txt"));
}

pub fn parse(s: &str) -> impl Iterator<Item = anyhow::Result<Token>> + '_ {
    s.chars().filter(|&x| x != ' ').map(|x| match x {
        '(' => Ok(Token::ParenOpen),
        ')' => Ok(Token::ParenClose),
        '+' => Ok(Token::Operator(Operator::Add)),
        '*' => Ok(Token::Operator(Operator::Multiply)),
        x => x
            .to_digit(10)
            .ok_or(anyhow!("Unsupported character"))
            .and_then(|x| i64::try_from(x).map_err(|e| anyhow::Error::from(e)))
            .map(|x| Token::Number(x)),
    })
}

pub fn evaluate_expression(
    input: &mut Peekable<impl Iterator<Item = anyhow::Result<Token>>>,
) -> anyhow::Result<i64> {
    let first_number = evaluate_numeric_value(input)?;
    continue_operation(first_number, input)
}

fn evaluate_numeric_value(
    input: &mut Peekable<impl Iterator<Item = anyhow::Result<Token>>>,
) -> anyhow::Result<i64> {
    match input.next() {
        Some(Ok(Token::Number(number))) => Ok(number),
        Some(Ok(Token::ParenOpen)) => evaluate_expression(input),
        Some(Err(err)) => Err(err),
        Some(_) | None => Err(anyhow!("Expected number")),
    }
}

fn continue_operation(
    lhs: i64,
    input: &mut Peekable<impl Iterator<Item = anyhow::Result<Token>>>,
) -> anyhow::Result<i64> {
    let operator = match input.next() {
        Some(Ok(Token::Operator(x))) => Ok(x),
        None | Some(Ok(Token::ParenClose)) => return Ok(lhs),
        Some(Ok(_)) => Err(anyhow!("Expected an operator")),
        Some(Err(err)) => Err(err),
    }?;

    let rhs = evaluate_numeric_value(input)?;

    let result = match operator {
        Operator::Add => lhs + rhs,
        Operator::Multiply => lhs * rhs,
    };

    continue_operation(result, input)
}

pub fn eval(s: &str) -> anyhow::Result<i64> {
    evaluate_expression(&mut parse(s).peekable())
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(eval("1 + 2 * 3 + 4 * 5 + 6").unwrap(), 71);
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(eval("1 + (2 * 3) + (4 * (5 + 6))").unwrap(), 51);
    }

    #[test]
    fn more_test_cases() {
        assert_eq!(eval("2 * 3 + (4 * 5)").unwrap(), 26);
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap(), 437);
        assert_eq!(
            eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap(),
            12240
        );
        assert_eq!(
            eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap(),
            13632
        );
    }

    #[test]
    fn answer() {
        let result = PUZZLE_INPUT
            .iter()
            .map(|&x| eval(x))
            .try_fold(0, |a, b| match b {
                Ok(b) => Ok(a + b),
                err @ Err(_) => err,
            })
            .unwrap();

        assert_eq!(result, 11004703763391);
    }
}

// #[cfg(test)]
// mod part_two {
//     use super::*;
//     #[test]
//     fn test_cases() {}
//     #[test]
//     fn answer() {}
// }
