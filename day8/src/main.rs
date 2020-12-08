use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let mut code = process_input(&buf);
    let answer = code.get_answer();
    let answer2 = code.get_answer2();

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> Code {
    Code {
        instructions: input.lines().map(Instruction::from_str).collect()
    }
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

struct Code {
    instructions: Vec<Instruction>,
}

impl Code {
    fn get_answer(&self) -> i16 {
        let mut accumulator = 0;
        let mut index = 0;
        let mut visited = vec![false; self.instructions.len()];

        while (index as usize) < self.instructions.len() {
            if visited[index as usize] {break;}

            visited[index as usize] = true;

            match self.instructions[index as usize] {
                Instruction::Acc(num) => {accumulator += num; index += 1;},
                Instruction::Jmp(num) => {index += num},
                Instruction::Nop(_) => {index += 1;}
            }
        }

        accumulator
    }

    fn terminates(&self) -> bool {
        let mut index = 0;
        let mut visited = vec![false; self.instructions.len()];

        while (index as usize) < self.instructions.len() {
            if visited[index as usize] {return false;}

            visited[index as usize] = true;

            match self.instructions[index as usize] {
                Instruction::Acc(_) => {index += 1;},
                Instruction::Jmp(num) => {index += num},
                Instruction::Nop(_) => {index += 1;}
            }
        }

        true
    }

    fn get_answer2(&mut self) -> i16 {
        for changed_index in 0..self.instructions.len() {
            self.instructions[changed_index].flip();
            if self.terminates() { break; };
            self.instructions[changed_index].flip();
        }
        self.get_answer()
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

        assert_eq!(5, code.get_answer())
    }
}
