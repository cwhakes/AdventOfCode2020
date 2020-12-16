use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let (fields, your_ticket, nearby_tickets) = process_input(&buf);
    
    let answer = get_answer(&fields, &nearby_tickets);
    let answer2 = get_answer2(&fields, &your_ticket, &nearby_tickets);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> (RuleSet, Vec<i64>, Vec<Vec<i64>>) {
    let mut iter = input.split("\n\n");

    let ruleset = RuleSet::from_str(iter.next().unwrap());

    let your_ticket = iter.next().unwrap().lines().nth(1).unwrap().split(",").map(|s| {
        s.parse().unwrap()
    }).collect();

    let nearby_tickets = iter.next().unwrap().lines().skip(1).map(|s| {
        s.split(",").map(|s| {
            s.parse().unwrap()
        }).collect()
    }).collect();

    (ruleset, your_ticket, nearby_tickets)
}

fn get_answer(ruleset: &RuleSet, nearby_tickets: &Vec<Vec<i64>>) -> i64 {
    let mut count = 0;
    for ticket in nearby_tickets {
        count += ticket.iter().filter(|value| {
            !ruleset.validate(**value)
        }).sum::<i64>();
    }
    count
}

fn get_answer2(ruleset: &RuleSet, your_ticket: &Vec<i64>, nearby_tickets: &Vec<Vec<i64>>) -> i64 {
    let nearby_tickets: Vec<_> = nearby_tickets.iter().filter(|ticket| {
        !ticket.iter().any(|value| {
            !ruleset.validate(*value)
        })
    }).collect();

    let num_tickets = nearby_tickets.len();
    let num_fields = nearby_tickets[0].len();

    let mut rule_indexes: Vec<_> = (0..num_fields).map( |field_number| {
        let mut possible_rules: HashSet<_> = ruleset.0.iter().collect();
        for ticket_number in 0..num_tickets {
            possible_rules.retain(|(_, rule)| {
                ruleset.matching_rules(nearby_tickets[ticket_number][field_number])
                    .any(|matching_rule| matching_rule.1 == *rule)
            });
        }
        (field_number, possible_rules)
    }).collect();

    rule_indexes.sort_by_key(|rules| rules.1.len());

    for index in (1..num_fields) {
        let (good_rulesets, bad_rulesets) = rule_indexes.split_at_mut(index);

        let good_rule_name = good_rulesets.last().unwrap().1.iter().next().unwrap().0;
        for rule_set in bad_rulesets {
            rule_set.1.retain(|rule| rule.0 != good_rule_name);
        }
    }

    let rule_indexes: HashMap<_,_> = rule_indexes.iter().map(|(rule_number, rules)| {
        assert_eq!(1, rules.len());
        let rule = rules.iter().next().unwrap();
        (rule.0.clone(), *rule_number)
    }).collect();


    let location = rule_indexes.get("departure location").unwrap();
    let station = rule_indexes.get("departure station").unwrap();
    let platform = rule_indexes.get("departure platform").unwrap();
    let track = rule_indexes.get("departure track").unwrap();
    let date = rule_indexes.get("departure date").unwrap();
    let time = rule_indexes.get("departure time").unwrap();

    your_ticket[*location] *
    your_ticket[*station] *
    your_ticket[*platform] *
    your_ticket[*track] *
    your_ticket[*date] *
    your_ticket[*time]
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Rule(Vec<(i64, i64)>);

impl Rule {
    fn from_str(input: &str) -> Self {
        let subrule_strs = input.split(" or ");
        let subrules = subrule_strs.map(|s| {
            let mut iter = s.split("-");
            let min = iter.next().unwrap().parse().unwrap();
            let max = iter.next().unwrap().parse().unwrap();
            (min, max)
        }).collect();
        Rule(subrules)
    }

    fn validate(&self, value: i64) -> bool {
        self.0.iter().any(|(min, max)| {
            *min <= value && value <= *max
        })
    }
}

struct RuleSet(HashMap<String, Rule>);

impl RuleSet {
    fn from_str(input: &str) -> Self {
        let rules = input.lines().map(|s| {
            let mut field_iter = s.split(": ");
            let name = field_iter.next().unwrap().to_owned();
            let rule = Rule::from_str(field_iter.next().unwrap());
            (name, rule)
        }).collect();
        RuleSet(rules)
    }

    fn validate(&self, value: i64) -> bool {
        self.0.values().any(|rule| {
            rule.validate(value)
        })
    }

    fn matching_rules(&self, value: i64) -> impl Iterator<Item=(&String, &Rule)> {
        self.0.iter().filter(move |(_, rule)|{
            rule.validate(value)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_answer() {
        let (fields, your_ticket, nearby_tickets) = process_input(INPUT);
        assert_eq!(71, get_answer(&fields, &nearby_tickets));
    }

//     const INPUT2: &'static str = "\
// class: 0-1 or 4-19
// row: 0-5 or 8-19
// seat: 0-13 or 16-19

// your ticket:
// 11,12,13

// nearby tickets:
// 3,9,18
// 15,1,5
// 5,14,9";

//     #[test]
//     fn test_answer2() {
//         let lines = process_input(INPUT2);
//         assert_eq!(208, get_answer2(&lines));
//     }
}
