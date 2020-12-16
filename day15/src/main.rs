use std::collections::HashMap;

fn main() {
    let buf = "1,20,8,12,0,14";

    let lines = process_input(&buf);
    
    let answer = get_answer(&lines);
    let answer2 = get_answer2(&lines);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> Vec<usize> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

fn get_answer(lines: &Vec<usize>) -> usize {
    Game::new(lines).nth(2020 - 1).unwrap()
}

fn get_answer2(lines: &Vec<usize>) -> usize {
    Game::new(lines).nth(30000000 - 1).unwrap()
}

struct Game<I: Iterator> {
    iteration: usize,
    starting: std::iter::Fuse<I>,
    spoken: HashMap<usize, usize>,
    next_num: Option<usize>,
}

impl<'a, I: Iterator<Item=&'a usize>> Game<I> {
    fn new<T: IntoIterator<IntoIter=I, Item=&'a usize>>(iter: T) -> Self {
        Game {
            iteration: 0,
            starting: iter.into_iter().fuse(),
            spoken: HashMap::new(),
            next_num: None,
        }
    }
}

impl<'a, I: Iterator<Item=&'a usize>> Iterator for Game<I> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let num = if let Some(num) = self.starting.next() {
            *num
        } else {
            if let Some(last_iteration) = self.next_num {
                 self.iteration - last_iteration -1 // we've already incremented
            } else {
                0
            }
        };

        self.next_num = self.spoken.insert(num, self.iteration);

        self.iteration += 1;
        Some(num)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "0,3,6";

    #[test]
    fn test_answer() {
        let lines = process_input(INPUT);
        assert_eq!(436, get_answer(&lines));
    }

    const INPUT2: &'static str = "0,3,6";

    #[test]
    fn test_answer2() {
        let lines = process_input(INPUT2);
        assert_eq!(175594, get_answer2(&lines));
    }
}
