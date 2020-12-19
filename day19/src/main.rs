use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let (rules, messages) = process_input(&buf);
    
    let answer = get_answer(&rules, &messages);
    let answer2 = get_answer2(&rules, &messages);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> (RuleSet, Vec<&str>) {
    let mut iter = input.split("\n\n");
    let rules = iter.next().unwrap().lines().map(Rule::from_str).collect();
    let messages = iter.next().unwrap().lines().collect();
    (RuleSet(rules), messages)
}

fn get_answer(rules: &RuleSet, messages: &[&str]) -> usize {
    messages.iter()
        .filter(|message| {
            rules.validate(0, message)
        })
        .count()
}

fn get_answer2(rules: &RuleSet, messages: &[&str]) -> usize {
    let mut rules = rules.clone();
    let (_, rule8) = Rule::from_str("8: 42 | 42 8");
    let (_, rule11) = Rule::from_str("11: 42 31 | 42 11 31");

    rules.0.insert(8, rule8);
    rules.0.insert(11, rule11);

    messages.iter()
    .filter(|message| {
        rules.validate(0, message)
    })
    .count()
}

#[derive(Clone)]
struct RuleSet<'a>(HashMap<usize, Rule<'a>>);

impl<'a> RuleSet<'a> {
    fn validate(&self, rule_index: usize, message: &str) -> bool {
        let rule_to_validate = self.0.get(&rule_index).unwrap();
        let remainders = rule_to_validate.validate(message, self);
        remainders.into_iter().any(str::is_empty)
    }
}

#[derive(Clone, Debug)]
enum Rule<'a> {
    Terminator(&'a str),
    Reference(HashSet<Vec<usize>>),
}

impl<'a> Rule<'a> {
    fn from_str(input: &'a str) -> (usize, Self) {
        let mut iter = input.split(": ");
        let index = iter.next().unwrap().parse().unwrap();
        
        let text = iter.next().unwrap();
        if text.starts_with("\"") {
            let (_head, tail) = text.split_at(1);
            let (body, _tail) = tail.split_at(tail.len()-1);
            (index, Rule::Terminator(body))
        } else {
            let h = text.split(" | ").map(|s| s.split(" ").map(|s| s.parse().unwrap()).collect()).collect();
            (index, Rule::Reference(h))
        }
    }

    fn validate<'s, 'r: 's>(&'r self, message: &'s str, rules: &'r RuleSet) -> Box<dyn Iterator<Item=&'s str> + 's> {
        if message.is_empty() { return Box::new(None.into_iter()) }

        match self {
            Rule::Terminator(s) => {
                if message.starts_with(s) {
                    let (_matched, unmatched) = message.split_at(s.len());
                    Box::new(Some(unmatched).into_iter())
                } else {
                    Box::new(None.into_iter())
                }
            }
            Rule::Reference(branches) => {
                Box::new(branches.iter().flat_map(move |sequence| {
                    let mut remaining_message = vec![message];
                    for rule in sequence.iter().map(|index| rules.0.get(index).unwrap()) {
                        remaining_message = remaining_message.iter().flat_map(|mess|
                            rule.validate(mess, rules)
                        ).collect()
                    }
                    remaining_message.into_iter()
                }))
            }
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

    #[test]
    fn test_answer() {
        let (rules, messages) = process_input(INPUT);
        assert_eq!(2, get_answer(&rules, &messages));
    }

    #[test]
    fn test_terminator() {
        let (_, rule) = dbg!(Rule::from_str("0: \"a\""));
        let rules = RuleSet(HashMap::new());
        let mut matches = rule.validate("a", &rules);
        assert_eq!("", matches.next().unwrap());
    }

    #[test]
    fn test_answer2() {
        let (rules, messages) = process_input(INPUT2);
        assert_eq!(3, get_answer(&rules, &messages));
        assert_eq!(12, get_answer2(&rules, &messages));
    }

    const INPUT2: &'static str = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";


}
