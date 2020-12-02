use nom::{
    IResult,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, anychar, digit1},
    combinator::map_res,
    error::{ParseError, FromExternalError},
};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let entries: Vec<Password> = reader.lines().filter_map(|s| s.unwrap().parse().ok()).collect();
    let answer = entries.into_iter().filter(Password::validate2).count();
    println!("{}", answer);
}

#[derive(Debug)]
struct Password {
    pub letter: char,
    pub min: usize,
    pub max: usize,
    pub password: String,
}

impl Password {
    pub fn validate(&self) -> bool{
        let count = self.password.chars().filter(|c| c == &self.letter).count();
        (self.min <= count) && (count <= self.max)
    }

    pub fn validate2(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        (self.letter == chars[self.min - 1]) ^ (self.letter == chars[self.max - 1])
    }
}

fn space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn parse_number<'a, E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>>(i: &'a str) -> IResult<&'a str, usize, E> {
    map_res(
        digit1,
        |number: &str| number.parse::<usize>()
    )(i)
}

impl FromStr for Password {
    type Err = nom::Err<()>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, min) = parse_number(s)?;
        let (s, _) = space(s)?;
        let (s, _) = tag("-")(s)?;
        let (s, _) = space(s)?;
        let (s, max) = parse_number(s)?;
        let (s, _) = space(s)?;
        let (s, letter) = anychar(s)?;
        let (s, _) = space(s)?;
        let (s, _) = tag(":")(s)?;
        let (s, _) = space(s)?;
        let (_, password) = alpha1(s)?;

        Ok(Password{
            min,
            max,
            letter,
            password: password.to_string(),
        })
    }
}
