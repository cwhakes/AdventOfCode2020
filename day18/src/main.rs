use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{all_consuming, cut, map, value},
    error::Error,
    IResult,
    multi::{many1},
    sequence::{preceded, terminated},
};

use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let expressions = process_input(&buf);
    
    let answer = get_answer(&expressions);
    let answer2 = get_answer2(&expressions);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> Vec<Expression> {
    input.lines().map(Expression::from_str).collect()
}

fn get_answer(expressions: &[Expression]) -> i64 {
    expressions.iter().map(Expression::evaluate).sum()
}

fn get_answer2(expressions: &[Expression]) -> i64 {
    expressions.iter().map(Expression::evaluate2).sum()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Expression(Vec<Token>);

impl Expression {
    fn from_str(s: &str) -> Self {
        let(_, out) = all_consuming(parse_expression)(s).unwrap();
        out
    }

    fn evaluate(&self) -> i64 {
        let mut total = 0;
        let mut operator = Operator::Add;
        for token in self.0.iter() {
            if let Token::Operator(op) = token {
                operator = op.clone();
            } else if let Some(num) = token.evaluate(Expression::evaluate) {
                match operator {
                    Operator::Add => total += num,
                    Operator::Multiply => total *= num,
                }
            }
        }
        total
    }

    fn evaluate2(&self) -> i64 {
        self.0.split(|o| *o == Token::Operator(Operator::Multiply)).map(|ops| {
            let mut total = 0;
            let mut operator = Operator::Add;
            for token in ops.iter() {
                if let Token::Operator(op) = token {
                    operator = op.clone();
                } else if let Some(num) = token.evaluate(Expression::evaluate2) {
                    match operator {
                        Operator::Add => total += num,
                        Operator::Multiply => total *= num,
                    }
                }
            }
            total
        }).product()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Token {
    Number(i64),
    Operator(Operator),
    Parenthetical(Expression)
}

impl Token {
    fn evaluate<F: FnOnce(&Expression) -> i64>(&self, evaluator: F) -> Option<i64> {
        match self {
            Token::Number(num) => Some(*num),
            Token::Parenthetical(exp) => Some(evaluator(exp)),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

fn parse_number(i: &str) -> IResult<&str, Token, Error<&str>> {
    map(
        digit1,
        |s: &str| Token::Number(s.parse::<i64>().unwrap_or(-1))
    )(i)
}

fn parse_operator(i: &str) -> IResult<&str, Token, Error<&str>> {
    let add = value(Operator::Add, tag("+"));
    let multiply = value(Operator::Multiply, tag("*"));

    map(
        alt((add, multiply)),
        |o| Token::Operator(o),
    )(i)
}

fn parse_parenthetical(i: &str) -> IResult<&str, Token, Error<&str>> {
    map(
        preceded(
            char('('),
            cut(
                terminated(
                    parse_expression,
                    preceded(multispace0, char(')'))
                )
            )
        ),
        |e| Token::Parenthetical(e)
    )(i)
}

fn parse_operation(i: &str) -> IResult<&str, Token, Error<&str>> {
    preceded(
        multispace0,
        alt((
            parse_number,
            parse_operator,
            parse_parenthetical,
        ))
    )(i)
}

fn parse_expression(i: &str) -> IResult<&str, Expression, Error<&str>> {
    map(
        many1(parse_operation),
        |e| Expression(e),
    )(i)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "2 * 3 + (4 * 5)";
    const INPUT2: &'static str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    const INPUT3: &'static str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    const INPUT4: &'static str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn test_answer() {
        let expressions = process_input(INPUT);
        assert_eq!(26, get_answer(&expressions));
    }

    #[test]
    fn test_answer2() {
        assert_eq!(46, get_answer2(&process_input(INPUT)));
        assert_eq!(1445, get_answer2(&process_input(INPUT2)));
        assert_eq!(669060, get_answer2(&process_input(INPUT3)));
        assert_eq!(23340, get_answer2(&process_input(INPUT4)));
    }
}
