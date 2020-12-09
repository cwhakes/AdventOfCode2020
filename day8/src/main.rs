use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let mut code = process_input(&buf);
    
    let answer = get_answer(&*code);
    let answer2 = get_answer2(&mut *code);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from_str).collect()
}

fn get_answer(code: &[Instruction]) -> i16 {
    let mut machine = Machine::default();
    machine.run(code);
    machine.accumulator
}

fn get_answer2(code: &mut [Instruction]) -> i16 {
    let mut machine = Machine::default();

    for changed_index in 0..code.len() {
        code[changed_index].flip();

        machine.run(code);
        if machine.terminates {break;}
        machine.reset();

        code[changed_index].flip();
    }

    machine.accumulator
}

#[derive(Copy, Clone)]
enum Instruction {
    Acc(i16),
    Jmp(i16),
    Nop(i16),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let (name, value) = s.split_at(3);
        match name {
            "acc" => Instruction::Acc(value.trim().parse().unwrap()),
            "jmp" => Instruction::Jmp(value.trim().parse().unwrap()),
            "nop" => Instruction::Nop(value.trim().parse().unwrap()),
            _ => panic!(),
        }
    }

    fn flip(&mut self) {
        match self {
            Instruction::Acc(_) => {},
            Instruction::Jmp(num) => {*self = Instruction::Nop(*num)}
            Instruction::Nop(num) => {*self = Instruction::Jmp(*num)}
        }
    }
}

#[derive(Default)]
struct Machine {
    index: i16,
    accumulator: i16,
    terminates: bool,
}

impl Machine {
    fn index(&self) -> usize {
        assert!(0 <= self.index);
        self.index as usize
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Acc(num) => {self.accumulator += num},
            Instruction::Jmp(num) => {self.index += num - 1},
            Instruction::Nop(_) => {}
        }
        self.index += 1;
    }

    fn run(&mut self, instructions: &[Instruction]) {
        let mut visited = vec![false; instructions.len()];

        while self.index() < instructions.len() {
            if visited[self.index()] {return;}

            visited[self.index()] = true;

            self.execute(instructions[self.index()]);
        }

        self.terminates = true;
    }

    fn reset(&mut self) {
        *self = Machine::default();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_answer() {
        let code = process_input(INPUT);

        assert_eq!(5, get_answer(&*code));
    }
}
