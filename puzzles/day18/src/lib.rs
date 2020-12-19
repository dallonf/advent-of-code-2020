// Day 18: Operation Order

use std::fmt::{Debug, Display};

use shared::prelude::*;

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
                .map(|x| Token::Number(i64::from(x))),
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

    fn evaluate_mk2(&self) -> i64 {
        match self {
            Expression::Number(num) => *num,
            Expression::Operation(operation) => operation.evaluate_mk2(),
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
        let left = self.left.evaluate();
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

    fn evaluate_mk2(&self) -> i64 {
        if self.right.is_empty() {
            return self.left.evaluate_mk2();
        }

        let continuation_to_eval = self
            .right
            .iter()
            .enumerate()
            // first try to find the first add operator
            .find(|(_, OperationContinuation(op, _))| *op == Operator::Add)
            // then settle for the first operator available
            .or_else(|| self.right.first().map(|first| (0, first)))
            .unwrap();
        let (index, OperationContinuation(operator, expression)) = continuation_to_eval;

        let rhs = expression.evaluate_mk2();
        let lhs = if index == 0 {
            self.left.evaluate_mk2()
        } else {
            self.right[index - 1].1.evaluate_mk2()
        };

        let new_value = match operator {
            Operator::Add => lhs + rhs,
            Operator::Multiply => lhs * rhs,
        };

        // Collapse the new value into the previous container
        let new_operation = {
            if index == 0 {
                let (_, new_right) = self.right.split_first().unwrap();
                Operation {
                    left: Box::new(Expression::Number(new_value)),
                    right: new_right.to_vec(),
                }
            } else {
                let new_right = self
                    .right
                    .iter()
                    .enumerate()
                    // Remove the evaluated operation
                    .filter(|(i, _)| *i != index)
                    .map(|(i, continuation)| {
                        if i == index - 1 {
                            OperationContinuation(
                                continuation.0,
                                Box::new(Expression::Number(new_value)),
                            )
                        } else {
                            continuation.to_owned()
                        }
                    })
                    .collect();

                Operation {
                    left: self.left.to_owned(),
                    right: new_right,
                }
            }
        };

        new_operation.evaluate_mk2()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Number(num) => f.write_fmt(format_args!("{}", num)),
            Expression::Operation(operation) => f.write_fmt(format_args!("({})", operation)),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tokens = self
            .right
            .iter()
            .flat_map(|OperationContinuation(operator, expression)| {
                vec![
                    match operator {
                        Operator::Add => "+".to_string(),
                        Operator::Multiply => "*".to_string(),
                    },
                    expression.to_string(),
                ]
            })
            .collect::<Vec<String>>()
            .join(" ");

        f.write_str(&format!("{} {}", self.left.to_string(), tokens))
    }
}

pub fn eval(s: &str) -> anyhow::Result<i64> {
    let tokens = parse(s)?;
    let expression: Expression = Expression::parse(&mut tokens.into_iter())?;

    Ok(expression.evaluate())
}

pub fn eval_mk2(s: &str) -> anyhow::Result<i64> {
    let tokens = parse(s)?;
    let expression: Expression = Expression::parse(&mut tokens.into_iter())?;

    Ok(expression.evaluate_mk2())
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

#[cfg(test)]
mod part_two {
    use super::*;
    #[test]
    fn test_case_1() {
        assert_eq!(eval_mk2("1 + 2 * 3 + 4 * 5 + 6").unwrap(), 231);
    }

    #[test]
    fn test_cases() {
        assert_eq!(eval_mk2("1 + (2 * 3) + (4 * (5 + 6))").unwrap(), 51);
        assert_eq!(eval_mk2("2 * 3 + (4 * 5)").unwrap(), 46);
        assert_eq!(eval_mk2("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap(), 1445);
        assert_eq!(
            eval_mk2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap(),
            669060
        );
        assert_eq!(
            eval_mk2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap(),
            23340
        );
    }

    #[test]
    fn answer() {
        let result = PUZZLE_INPUT
            .iter()
            .map(|&x| eval_mk2(x))
            .try_fold(0, |a, b| match b {
                Ok(b) => Ok(a + b),
                err @ Err(_) => err,
            })
            .unwrap();

        assert_eq!(result, 290726428573651);
    }
}
