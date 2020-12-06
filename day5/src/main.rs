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
    input.lines().map(number_from_binary).max().unwrap()
}

fn get_answer2(input: &str) -> usize {
    let mut vec: Vec<usize> = input.lines().map(number_from_binary).collect();
    vec.sort();
    for pair in vec.windows(2) {
        if pair[1] - pair[0] > 1 {
            return pair[0] + 1;
        }
    }
    panic!("No empty seats")
}

fn number_from_binary(bytes: &(impl AsRef<[u8]> + ?Sized)) -> usize {
    bytes.as_ref().iter().fold(0, |num, byte| {
        match byte {
            b'F' | b'L' => 2 * num,
            b'B' | b'R' => 2 * num + 1,
            _ => panic!(*byte)
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(567, number_from_binary("BFFFBBFRRR"));
        assert_eq!(119, number_from_binary("FFFBBBFRRR"));
        assert_eq!(820, number_from_binary("BBFFBBFRLL"));
    }
}