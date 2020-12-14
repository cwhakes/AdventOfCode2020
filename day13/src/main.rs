use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let (timestamp, busses) = process_input(&buf);
    
    let answer = get_answer(timestamp, &busses);
    let answer2 = get_answer2(&busses);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> (i64, Vec<Option<i64>>) {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse().unwrap();
    let busses = lines.next().unwrap().split(",").map(|s| s.parse().ok()).collect();

    (timestamp, busses)
}

fn get_answer(timestamp: i64, busses: &Vec<Option<i64>>) -> i64 {
    let busses: Vec<_> = busses.iter().filter_map(|a| a.as_ref()).collect();
    let times_since_last = busses.iter().map(|b| timestamp % *b);
    let times_until_next = times_since_last.enumerate().map(|(index, last)| busses[index] - last);
    let (bus_index, next) = times_until_next.enumerate().min_by_key(|(_, next)| *next).unwrap();
    next * busses[bus_index]
}

fn get_answer2(busses: &Vec<Option<i64>>) -> i64 {
    let offsets_and_busses: Vec<_> = busses.iter().enumerate()
        .filter_map(|(offset, bus)| bus.map(|b| (offset as i64, b))).collect();
    let busses: Vec<_> = offsets_and_busses.iter().map(|(_, b)| *b).collect();
    let offsets: Vec<_> = offsets_and_busses.iter().map(|(o, b)| b - o).collect();
    chinese_remainder(&*offsets, &*busses).unwrap()
}

//https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "\
939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_answer() {
        let (timestamp, busses) = process_input(INPUT);
        assert_eq!(295, get_answer(timestamp, &busses));
    }

    #[test]
    fn test_answer2() {
        let (_, busses) = process_input(INPUT);
        assert_eq!(1068781, get_answer2(&busses));
    }
}