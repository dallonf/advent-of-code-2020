// Day 18: Operation Order

use shared::prelude::*;
use std::convert::TryFrom;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Number(i64),
    Operation(Operation),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operation {
    left: Box<Expression>,
    right: Vec<OperationContinuation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationContinuation(Operator, Box<Expression>);

lazy_static! {
    static ref PUZZLE_INPUT: Vec<&'static str> =
        puzzle_input::lines(include_str!("puzzle_input.txt"));
}

pub fn parse(s: &str) -> anyhow::Result<Vec<Token>> {
    s.chars()
        .filter(|&x| x != ' ')
        .map(|x| match x {
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
        .collect()
}

impl Expression {
    pub fn parse(input: &mut impl Iterator<Item = Token>) -> anyhow::Result<Expression> {
        Ok(Expression::Operation(Operation::try_collect(input)?))
    }

    fn parse_numeric(input: &mut impl Iterator<Item = Token>) -> anyhow::Result<Expression> {
        match input.next() {
            Some(Token::Number(number)) => Ok(Expression::Number(number)),
            Some(Token::ParenOpen) => Expression::parse(input),
            Some(_) | None => Err(anyhow!("Expected number")),
        }
    }

    fn evaluate(&self) -> i64 {
        match self {
            Expression::Number(num) => *num,
            Expression::Operation(operation) => operation.evaluate(),
        }
    }
}

impl Operation {
    pub fn try_collect(input: &mut impl Iterator<Item = Token>) -> anyhow::Result<Operation> {
        let left = Box::new(Expression::parse_numeric(input)?);

        fn next(
            iter: &mut impl Iterator<Item = Token>,
            mut list_so_far: Vec<OperationContinuation>,
        ) -> anyhow::Result<Vec<OperationContinuation>> {
            let operator = iter.next();
            match operator {
                Some(Token::Operator(operator)) => {
                    let expression = Expression::parse_numeric(iter)?;
                    let continuation = OperationContinuation(operator, Box::new(expression));
                    list_so_far.push(continuation);
                    next(iter, list_so_far)
                }
                None | Some(Token::ParenClose) => Ok(list_so_far), // end of operation
                Some(_) => Err(anyhow!("Expected an operator")),
            }
        }

        let right = next(input, vec![])?;

        Ok(Operation { left, right })
    }

    fn evaluate(&self) -> i64 {
        let left = self.left.deref().evaluate();
        if self.right.is_empty() {
            return left;
        }

        // Solve first continuation
        let (OperationContinuation(operation, expression), new_right) =
            self.right.split_first().unwrap();
        let expression = expression.evaluate();

        let new_left = match operation {
            Operator::Add => left + expression,
            Operator::Multiply => left * expression,
        };

        let new_operation = Operation {
            left: Box::new(Expression::Number(new_left)),
            right: new_right.to_vec(),
        };

        new_operation.evaluate()
    }
}

pub fn eval(s: &str) -> anyhow::Result<i64> {
    let tokens = parse(s)?;
    let expression: Expression = Expression::parse(&mut tokens.into_iter())?;

    Ok(expression.evaluate())
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
