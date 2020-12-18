// Day 18: Operation Order

use shared::prelude::*;
use std::{convert::TryFrom, iter::Peekable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Number(i32),
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
    // static ref PUZZLE_INPUT: Vec<&'static str> =
    //     puzzle_input::lines(include_str!("puzzle_input.txt"));
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
            .and_then(|x| i32::try_from(x).map_err(|e| anyhow::Error::from(e)))
            .map(|x| Token::Number(x)),
    })
}

pub fn evaluate_expression(
    mut input: Peekable<impl Iterator<Item = anyhow::Result<Token>>>,
) -> anyhow::Result<i32> {
    let first_number = input.next().ok_or(anyhow!("Empty expression"))??;
    let first_number = if let Token::Number(x) = first_number {
        x
    } else {
        return Err(anyhow!("Expected number"));
    };

    continue_operation(first_number, &mut input)
}

pub fn continue_operation(
    lhs: i32,
    input: &mut Peekable<impl Iterator<Item = anyhow::Result<Token>>>,
) -> anyhow::Result<i32> {
    let operator = match input.next() {
        Some(Ok(Token::Operator(x))) => Ok(x),
        Some(Ok(_)) => Err(anyhow!("Expected an operator")),
        Some(Err(err)) => Err(err),
        None => return Ok(lhs),
    }?;

    let rhs = match input.next() {
        Some(Ok(Token::Number(x))) => Ok(x),
        Some(Ok(_)) | None => Err(anyhow!("Expected a number")),
        Some(Err(err)) => Err(err),
    }?;

    let result = match operator {
        Operator::Add => lhs + rhs,
        Operator::Multiply => lhs * rhs,
    };

    continue_operation(result, input)
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            evaluate_expression(parse("1 + 2 * 3 + 4 * 5 + 6").peekable()).unwrap(),
            71
        );
    }

    // #[test]
    // fn answer() {
    //     assert_eq!(*PUZZLE_INPUT, Vec::<String>::new());
    // }
}

// #[cfg(test)]
// mod part_two {
//     use super::*;
//     #[test]
//     fn test_cases() {}
//     #[test]
//     fn answer() {}
// }
