use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::mem;

use nom::{
    IResult,
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{digit1, hex_digit1, multispace0, newline, space0},
    combinator::{all_consuming, value},
    error::Error,
    sequence::{tuple, pair, preceded},
};

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let answer = get_answer2(&buf);

    println!("{}", answer);
}

fn get_answer(input: &str) -> usize {
    PassportRaw::from_str(input)
        .into_iter()
        .filter_map(|p| p.check_fields().ok())
        .count()
}

fn get_answer2(input: &str) -> usize {
    PassportRaw::from_str(input)
        .into_iter()
        .filter_map(|p| p.check_fields().ok())
        .filter_map(|p| if p.validate() {Some(())} else {None})
        .count()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Field {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    fn validate(&self) -> bool {
        validate_byr(&self.byr) &&
        validate_iyr(&self.iyr) &&
        validate_eyr(&self.eyr) &&
        validate_hgt(&self.hgt) &&
        validate_hcl(&self.hcl) &&
        validate_ecl(&self.ecl) &&
        validate_pid(&self.pid)
    }
}

#[derive(Debug)]
struct PassportRaw (HashMap<Field, String>);

impl PassportRaw {
    fn new() -> Self {
        PassportRaw(HashMap::new())
    }

    fn check_fields(mut self) -> Result<Passport, ()> {
        let byr = self.0.remove(&Field::BirthYear).ok_or(())?;
        let iyr = self.0.remove(&Field::IssueYear).ok_or(())?;
        let eyr = self.0.remove(&Field::ExpirationYear).ok_or(())?;
        let hgt = self.0.remove(&Field::Height).ok_or(())?;
        let hcl = self.0.remove(&Field::HairColor).ok_or(())?;
        let ecl = self.0.remove(&Field::EyeColor).ok_or(())?;
        let pid = self.0.remove(&Field::PassportId).ok_or(())?;
        let cid = self.0.remove(&Field::CountryId);

        Ok(Passport{
            byr, iyr, eyr, hgt, hcl, ecl, pid, cid,
        })
    }

    fn from_str(string: &str) -> Vec<PassportRaw> {
        let mut passports = Vec::new();
        let mut stream = &*string;
        let mut passport_working = PassportRaw::new();

        while let Ok((new_stream, (field_name, field_value))) = parse_entry(stream) {
            passport_working.0.insert(field_name, field_value.to_string());
            stream = new_stream;

            if let Ok((new_stream, _)) = parse_separator(stream) {
                passports.push(mem::replace(&mut passport_working, PassportRaw::new()));
                stream = new_stream;
            }
        };
        passports.push(passport_working);

        passports
    }
}

fn parse_separator(i: &str) -> IResult<&str, (char, &str, char), Error<&str>> {
    tuple((newline, space0, newline))(i)
}

fn parse_field_name(i: &str) -> IResult<&str, Field, Error<&str>> {
    let byr = value(Field::BirthYear, tag("byr"));
    let iyr = value(Field::IssueYear, tag("iyr"));
    let eyr = value(Field::ExpirationYear, tag("eyr"));
    let hgt = value(Field::Height, tag("hgt"));
    let hcl = value(Field::HairColor, tag("hcl"));
    let ecl = value(Field::EyeColor, tag("ecl"));
    let pid = value(Field::PassportId, tag("pid"));
    let cid = value(Field::CountryId, tag("cid"));

    preceded(
        multispace0,
        alt((byr,iyr,eyr,hgt,hcl,ecl,pid,cid)),
    )(i)
}

fn parse_entry(i: &str) -> IResult<&str, (Field, &str), Error<&str>> {
    pair(
        parse_field_name,
        preceded(
            tag(":"),
            is_not(" \t\r\n"),
        )
    )(i)
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum HeightUnit { Cm, Inch }

fn parse_height(i: &str) -> IResult<&str, (&str, HeightUnit), Error<&str>> {
    let cm = value(HeightUnit::Cm, tag("cm"));
    let inch = value(HeightUnit::Inch, tag("in"));
    pair(
        digit1,
        alt((cm, inch))
    )(i)
}

fn parse_hair_color(i: &str) -> IResult<&str, &str, Error<&str>> {
    all_consuming(preceded(
        tag("#"),
        hex_digit1,
    ))(i)
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum EyeColor { Amber, Blue, Brown, Grey, Green, Hazel, Other}

fn parse_eye_color(i: &str) -> IResult<&str, EyeColor, Error<&str>> {
    let amb = value(EyeColor::Amber, tag("amb"));
    let blu = value(EyeColor::Blue, tag("blu"));
    let brn = value(EyeColor::Brown, tag("brn"));
    let gry = value(EyeColor::Grey, tag("gry"));
    let grn = value(EyeColor::Green, tag("grn"));
    let hzl = value(EyeColor::Hazel, tag("hzl"));
    let oth = value(EyeColor::Other, tag("oth"));

    alt((amb, blu, brn, gry, grn, hzl, oth))(i)
}

fn parse_passport_id(i: &str) -> IResult<&str, &str, Error<&str>> {
    all_consuming(digit1)(i)
}

fn validate_byr(byr: &str) -> bool {
    if let Ok(byr) = byr.parse::<i32>() {
        1920 <= byr && byr <= 2002
    } else { false }
}

fn validate_iyr(iyr: &str) -> bool {
    if let Ok(iyr) = iyr.parse::<i32>() {
        2010 <= iyr && iyr <= 2020
    } else { false }
}

fn validate_eyr(eyr: &str) -> bool {
    if let Ok(eyr) = eyr.parse::<i32>() {
        2020 <= eyr && eyr <= 2030
    } else { false }
}

fn validate_hgt(hgt: &str) -> bool {
    if let Ok((_, (num, unit))) = parse_height(hgt) {
        if let Ok(height) = num.parse::<i32>() {
            match unit {
                HeightUnit::Cm => 150 <= height && height <= 193,
                HeightUnit::Inch => 59 <= height && height <= 76,
            }
        } else { false }
    } else { false }
}

fn validate_hcl(hcl: &str) -> bool {
    if let Ok((_, hcl)) = parse_hair_color(hcl) {
        6 == hcl.len()
    } else { false }
}

fn validate_ecl (ecl: &str) -> bool {
    if let Ok((_, _ecl)) = parse_eye_color(ecl) {
        true
    } else { false }
}

fn validate_pid (pid: &str) -> bool {
    if let Ok((_, pid)) = parse_passport_id(pid) {
        9 == pid.len()
    } else { false }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let answer = get_answer(input);
        assert_eq!(2, answer);
    }

    #[test]
    fn part2() {
        let invalid = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let valid = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert_eq!(0, get_answer2(invalid));
        assert_eq!(4, get_answer2(valid));
    }
}