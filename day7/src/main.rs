use petgraph::{algo, graphmap::DiGraphMap};
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
    let rules: Vec<_> = input.lines().map(Rule::from_str).collect();
    let mut graph = RuleGraph::new();
    graph.add_rules(&rules);
    graph.count_parents("shiny gold") - 1
}

fn get_answer2(input: &str) -> usize {
    let rules: Vec<_> = input.lines().map(Rule::from_str).collect();
    let mut graph = RuleGraph::new();
    graph.add_rules(&rules);
    graph.count_contents("shiny gold") - 1
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule<'a> {
    container: &'a str,
    contents: Vec<(usize, String)>
}

impl<'r> Rule<'r> {
    fn from_str(s: &'r str) -> Self {
        let container = s.split(" bags contain ").next().unwrap();
        let contents_str = s.split(" bags contain ").nth(1).unwrap();

        let contents;
        if contents_str == "no other bags." {
            contents = Vec::new()
        } else {
            contents = contents_str.split(", ")
                .map(|s| {
                    let mut words = s.split_whitespace();
                    let num = words.next().unwrap().parse().unwrap();
                    let descriptor1 = words.next().unwrap();
                    let descriptor2 = words.next().unwrap();
                    let bag = format!("{} {}", descriptor1, descriptor2);

                    (num, bag)
                })
                .collect();
        }

        Rule {
            container,
            contents,
        }
    }
}

struct RuleGraph<'g> {
    graph: DiGraphMap<&'g str, usize>
}

impl<'g> RuleGraph<'g> {
    fn new() -> Self {
        RuleGraph {
            graph: DiGraphMap::new(),
        }
    }

    fn add_rules<'r: 'g, 's: 'g>(&mut self, rules: &'s [Rule<'r>]) {
        for rule in rules {
            self.graph.add_node(rule.container);
        }
        for rule in rules {
            for (weight, content) in rule.contents.iter() {
                self.graph.add_edge(rule.container, &*content, *weight);
            }
        }
    }

    fn count_parents(&self, name: &str) -> usize {
        self.graph.nodes()
            .filter(|n| algo::has_path_connecting(&self.graph, n, name, None)).count()
    }

    fn count_contents(&self, name: &str) -> usize {
        let mut count = 1;
        for (_, neighbor, edge) in self.graph.edges(name) {
            count += edge * self.count_contents(neighbor);
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rule_parse() {
        let text = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let rule = Rule {
            container: "light red",
            contents: vec![
                (1, "bright white".to_string()),
                (2, "muted yellow".to_string()),
            ]
        };

        assert_eq!(rule, Rule::from_str(text));
    }

    #[test]
    fn test_count_parents() {
        let input = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(4, get_answer(input))
    }

    #[test]
    fn test_count_contents() {
        let input = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        assert_eq!(126, get_answer2(input));
    }
}
