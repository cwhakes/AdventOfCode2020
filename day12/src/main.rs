use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let commands = process_input(&buf);
    
    let answer = get_answer(&commands);
    let answer2 = get_answer2(&commands);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> Vec<Command> {
    input.lines().map(Command::from_str).collect()
}

fn get_answer(commands: &Vec<Command>) -> i32 {
    let mut ferry = Ferry::new();
    for command in commands {
        ferry.run_command(*command);
    }
    ferry.north.abs() + ferry.east.abs()
}

fn get_answer2(commands: &Vec<Command>) -> i32 {
    let mut ferry = Ferry::new();
    for command in commands {
        ferry.run_command2(*command);
    }
    ferry.north.abs() + ferry.east.abs()
}

#[derive(Clone, Copy, Debug)]
enum Command {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Forward(i32),
    Left(i32),
    Right(i32)
}

impl Command {
    fn from_str(s: &str) -> Self {
        let (direction, num) = s.split_at(1);
        let num = num.parse::<i32>().unwrap();
        match direction {
            "N" => Command::North(num),
            "E" => Command::East(num),
            "S" => Command::South(num),
            "W" => Command::West(num),
            "F" => Command::Forward(num),
            "L" => Command::Left(num),
            "R" => Command::Right(num),
            _ => panic!()
        }
    }
}

#[derive(Debug)]
struct Ferry {
    north: i32,
    east: i32,
    direction: i32,
    waypoint_north: i32,
    waypoint_east: i32,
}

impl Ferry {
    fn new() -> Self {
        Ferry {
            north: 0,
            east: 0,
            direction: 90,
            waypoint_north: 1,
            waypoint_east: 10,
        }
    }

    fn run_command(&mut self, command: Command) {
        match command {
            Command::North(num) => self.north += num,
            Command::East(num) => self.east += num,
            Command::South(num) => self.north -= num,
            Command::West(num) => self.east -= num,
            
            Command::Forward(num) => match self.direction {
                0 => self.north += num,
                90 => self.east += num,
                180 => self.north -= num,
                270 => self.east -= num,
                _ => panic!()
            }
            
            Command::Left(num) => self.direction = (self.direction - num + 360) % 360,
            Command::Right(num) => self.direction = (self.direction + num) % 360,
        }
    }

    fn run_command2(&mut self, command: Command) {
        match command {
            Command::North(num) => self.waypoint_north += num,
            Command::East(num) => self.waypoint_east += num,
            Command::South(num) => self.waypoint_north -= num,
            Command::West(num) => self.waypoint_east -= num,
            
            Command::Forward(num) => {
                self.north += num * self.waypoint_north;
                self.east += num * self.waypoint_east;
            }
            
            Command::Left(num) => self.rotate_waypoint(num),
            Command::Right(num) => self.rotate_waypoint(-num),
        }
    }

    fn rotate_waypoint(&mut self, degrees: i32) {
        let num = ((360 + degrees) % 360)/90;
        for _ in 0..num {
            let temp = self.waypoint_north;
            self.waypoint_north = self.waypoint_east;
            self.waypoint_east = - temp;
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "\
F10
N3
F7
R90
F11";

    #[test]
    fn test_answer() {
        let commands = process_input(INPUT);
        assert_eq!(25, get_answer(&commands));
    }

    #[test]
    fn test_answer2() {
        let mut ferry = process_input(INPUT);
        assert_eq!(286, get_answer2(&mut ferry));
    }
}