use std::fs::File;
use std::io::Read;
use petgraph::{algo, graphmap::DiGraphMap};

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let ints = process_input(&buf);
    
    let answer = get_answer(&*ints);
    let answer2 = get_answer2(&*ints);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> Vec<i64> {
    let mut vec: Vec<_> = input.lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    vec.sort();
    vec
}

fn get_answer(ints: &[i64]) -> i64 {
    let mut ones = 0;
    let mut threes = 1;
    ints.iter()
        .scan(0, |prev, current| {
            let diff = *current - *prev;
            *prev = *current;
            Some(diff)
        })
        .for_each(|diff| match dbg!(diff) {
            1 => {ones += 1;},
            3 => {threes += 1;}
            _ => {}
        });
    ones * threes

}

fn get_answer2(ints: &[i64]) -> usize {
    let vec: Vec<_> = ints.iter()
        .scan(0, |prev, current| {
            let diff = *current - *prev;
            *prev = *current;
            Some(diff)
        })
        .collect();
    vec.split(|i| *i == 3).map(calc_permutations).product()
}

fn calc_permutations(diffs: &[i64]) -> usize {
    if diffs.len() < 2 {return 1;}

    let mut graph: DiGraphMap<usize, ()> = DiGraphMap::new();
    for i in 0..(diffs.len()+1) {
        graph.add_node(i);
    }


    for (index, diff) in diffs.windows(3).enumerate() {
        if diff[0] <= 3 {
            graph.add_edge(index, index + 1, ());
        }
        if diff[0] + diff[1] <= 3 {
            graph.add_edge(index, index + 2, ());
        }
        if diff[0] + diff[1] + diff[2] <= 3 {
            graph.add_edge(index, index + 3, ());
        }
    }

    if diffs[diffs.len() - 2] <= 3 {
        graph.add_edge(diffs.len() - 2, diffs.len() - 1 , ());
    }

    if diffs[diffs.len() - 2] + diffs[diffs.len() - 1] <= 3 {
        graph.add_edge(diffs.len() - 2, diffs.len(), ());
    }

    if diffs[diffs.len() - 1] <= 3 {
        graph.add_edge(diffs.len() - 1, diffs.len() , ());
    }
    
    let count = algo::all_simple_paths::<Vec<usize>, &DiGraphMap<usize, ()>>(&graph, 0, diffs.len(), 0, None).count();
    dbg!(count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer() {
        let input = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let processed = process_input(input);
        assert_eq!(220, get_answer(&*processed));
    }

    #[test]
    fn test_answer2() {
        let input = "\
16
10
15
5
1
11
7
19
6
12
4";
        let processed = process_input(input);
        assert_eq!(8, get_answer2(&*processed));
    } 
}