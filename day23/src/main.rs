fn main() {
    let cups = process_input("137826495");
    
    let answer = get_answer(100, &cups);
    let answer2 = get_answer2(10_000_000, &cups);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> Cups {
    Cups::from_str(input)
}

fn get_answer(iterations: usize, cups: &Cups) -> String {
    let mut cups = cups.clone();
    let mut current = cups.0[cups.1];
    for _ in 0..iterations {
        cups.rotate(current);
        current = cups.0[current]
    }
    cups.score()
}

fn get_answer2(iterations: usize, cups: &Cups) -> usize {
    let mut cups = cups.clone().expand(1_000_000);
    let mut current = cups.0[cups.1];

    for _ in 0..iterations {
        cups.rotate(current);
        current = cups.0[current]
    }
    cups.score2()
}

#[derive(Clone, Debug)]
struct Cups(Vec<usize>, usize);

impl Cups {
    fn from_str(input: &str) -> Self {

        let cups_real: Vec<_> = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

        let mut cups = vec![0; cups_real.len() + 1];
        let mut iter = cups_real.iter().peekable();

        let last_cup = **iter.peek().unwrap();
        while let Some(cup) = iter.next() {
            let next_cup = iter.peek().map(|c| **c).unwrap_or(last_cup);
            cups[*cup] = next_cup;
        }

        Cups(cups, cups_real[cups_real.len() -1])
    }

    fn expand(mut self, final_count: usize) -> Self {
        let last_cup = self.1;
        let first_cup = self.0[last_cup];

        self.0[last_cup] = self.0.len();

        self.0.reserve(final_count + 1);
        for index in self.0.len()..final_count {
            self.0.push(index+1)
        }
        self.0.push(first_cup);
        self.1 = final_count;
        assert_eq!(final_count + 1, self.0.len());
        self
    }

    fn rotate(&mut self, current: usize) {
        let three = self.pickup_3(current);
        let dst = self.find_destination(current, &three);
        self.insert_3(dst, three);
    }

    fn iter_from(&self, index: usize) -> CupsIter {
        CupsIter { cups: self, current: index }
    }

    fn find_destination(&self, mut current: usize, excluded: &[usize; 3]) -> usize {
        loop {
            current -= 1;
            if current == 0 { current = self.0.len() - 1}
            if !excluded.contains(&current) {
                break current;
            }
        }
    }

    fn pickup_3(&mut self, current: usize) -> [usize; 3] {
        let mut iter = self.iter_from(current);
        let a = *iter.next().unwrap();
        let b = *iter.next().unwrap();
        let c = *iter.next().unwrap();
        std::mem::drop(iter);

        self.0[current] = self.0[c];

        [a, b, c]
    }

    fn insert_3(&mut self, destination: usize, cups: [usize; 3]) {
        let last = self.0[destination];
        self.0[destination] = cups[0];
        self.0[cups[0]] = cups[1];
        self.0[cups[1]] = cups[2];
        self.0[cups[2]] = last;
    }

    fn score(&mut self) -> String {
        let len = self.0.len();

        let mut cups_string = String::new();
        for cup in self.iter_from(1).take(len-2) {
            cups_string.push_str(&cup.to_string());
        }
        cups_string
    }

    fn score2(&mut self) -> usize {
        let mut iter = self.iter_from(1);
        
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();

        *a * *b
    }
}

impl std::fmt::Display for Cups {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let len = self.0.len();
        let last = self.1;

        for cup in self.iter_from(last).take(len - 1) {
            write!(f, " {} ", cup)?;
        }
        Ok(())
    }
}

struct CupsIter<'a> {
    cups: &'a Cups,
    current: usize,
}

impl<'a> Iterator for CupsIter<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.cups.0.get(self.current).unwrap();
        self.current = *item;
        Some(item)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer() {
        let cups = process_input(INPUT);
        assert_eq!("92658374", &get_answer(10, &cups));
    }


    // #[test]
    // fn test_answer2() {
    //     let cups = process_input(INPUT);
    //     assert_eq!(149245887792, get_answer2(10_000_000, &cups));
    // }

    const INPUT: &'static str = "389125467";

}
