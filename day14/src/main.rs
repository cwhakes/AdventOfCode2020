use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let lines = process_input(&buf);
    
    let answer = get_answer(&lines);
    let answer2 = get_answer2(&lines);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> Vec<Line> {
    input.lines().map(|s| {
        if s.starts_with("mask") { return Line::Mask(Mask::from_str(s))}
        if s.starts_with("mem") { return Line::Command(Command::from_str(s))}
        panic!()
    }).collect()
}

fn get_answer(lines: &Vec<Line>) -> u64 {
    let mut memory = Vec::new();
    let mut mask = Mask::default();

    for line in lines {
        match line {
            Line::Mask(m) => mask = m.clone(),
            Line::Command(command) => {
                if memory.len() <= command.index {
                    memory.resize_with(command.index + 1, Default::default)
                }
                memory[command.index] = mask.mask(command.value)
            }
        }
    }

    memory.iter().sum()
}

fn get_answer2(lines: &Vec<Line>) -> u64 {
    let mut memory: HashMap<usize, u64>  = HashMap::new();
    let mut mask = Mask::default();

    for line in lines {
        match line {
            Line::Mask(m) => mask = m.clone(),
            Line::Command(command) => {
                for index in mask.indexes(command.index) {
                    memory.insert(index, command.value);
                }
            }
        }
    }

    memory.values().sum()
}

enum Line {
    Mask(Mask),
    Command(Command),
}

#[derive(Clone, Default, Debug)]
struct Mask {
    zeros: u64,
    ones: u64,
}

impl Mask {
    fn from_str(input: &str) -> Self {
        let mask = input.split(" = ").nth(1).unwrap();
        let mut zeros = u64::MAX;
        let mut ones = 0u64;
        for character in mask.chars() {
            zeros = zeros.rotate_left(1);
            ones = ones.rotate_left(1);
            match character {
                '0' => zeros -= 1,
                '1' => ones += 1,
                _ => {}
            }
        }
        //println!("ones:  {:064b}\nzeros: {:064b}", ones, zeros);
        Mask { zeros, ones }
    }

    fn mask(&self, num: u64) -> u64 {
        (num & self.zeros) | self.ones
    }

    fn indexes(&self, command_index: usize) -> MaskIter {
        let floating =  (self.zeros & (!self.ones)) % (2u64.pow(36));
        let command_index = (command_index | self.ones as usize) & !floating as usize;
        MaskIter {command_index, floating, internal_index: 0}
    }
}

struct MaskIter {
    command_index: usize,
    floating: u64,
    internal_index: usize,
}

impl Iterator for MaskIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut index = 0;
        let mut shift = 0;
        let digits = 64 - self.internal_index.leading_zeros();
        if digits > self.floating.count_ones() { return None; }

        for ii_shift in 0..digits {
            let current_digit = (self.internal_index >> ii_shift) & 1;
            while ((self.floating >> shift) & 1) == 0 {
                shift += 1;
            }
            index += current_digit << shift;
            shift += 1;
        }
        
        self.internal_index += 1;
        Some(self.command_index | index)
    }
}

struct Command {
    index: usize,
    value: u64,
}

impl Command {
    fn from_str(input: &str) -> Self {
        let mut input = input[4..].split("] = ");
        let index = input.next().unwrap().parse().unwrap();
        let value = input.next().unwrap().parse().unwrap();
        Command { index, value }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_answer() {
        let lines = process_input(INPUT);
        assert_eq!(165, get_answer(&lines));
    }

    const INPUT2: &'static str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_answer2() {
        let lines = process_input(INPUT2);
        assert_eq!(208, get_answer2(&lines));
    }
}
