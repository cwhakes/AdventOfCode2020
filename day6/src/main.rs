use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let answer = get_answer2(&buf);

    println!("{}", answer);
}

fn get_answer(input: &str) -> usize {
    input.split("\n\n").map(get_group_count).sum()
}

fn get_group_count(group_answers: &str) -> usize {
    let questions = "abcdefghijklmnopqrstuvwxyz";

    questions.chars().filter(|c| group_answers.contains(*c)).count()
}

fn get_answer2(input: &str) -> usize {
    input.split("\n\n").map(get_group_count2).sum()
}

fn get_group_count2(group_answers: &str) -> usize {
    let questions = "abcdefghijklmnopqrstuvwxyz";
    let mut answers: Vec<_> = questions.chars().collect();

    for individual in group_answers.lines() {
        answers.retain(|c| individual.contains(*c))
    }

    answers.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(11, get_answer(GROUP_ANSWERS));
    }

    #[test]
    fn part2() {
        assert_eq!(6, get_answer2(GROUP_ANSWERS));
    }

    const GROUP_ANSWERS: &'static str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";
}