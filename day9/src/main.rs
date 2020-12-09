use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let ints = process_input(&buf);
    
    let answer = get_answer(&*ints);
    let answer2 = get_answer2(answer, &*ints);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> Vec<i64> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

fn get_answer(ints: &[i64]) -> i64 {
    for index in 25..ints.len() {
        if !check_int(ints[index], &ints[index-25..index]) {
            return ints[index];
        }
    }
    panic!()
}

fn check_int(int: i64, slice: &[i64]) -> bool{
    let mut array = slice.into_iter().collect::<Vec<&i64>>();
    array.sort();

    let mut head = 0;
    let mut foot = array.len() - 1;

    while head < foot {
        let sum = array[head] + array[foot];
        match sum.cmp(&int) {
            Ordering::Less => {head +=1; continue;}
            Ordering::Greater => {foot -=1; continue;}
            Ordering::Equal => {return true;}
        }
    }

    false
}

fn get_answer2(key: i64, ints: &[i64]) -> i64 {
    let mut head = 0;
    let mut foot = 2;

    loop {
        match ints[head..foot].iter().sum::<i64>().cmp(&key) {
            Ordering::Less => {foot += 1; continue;}
            Ordering::Greater => {head +=1; continue;}
            Ordering::Equal => {break;}
        }
    }

    ints[head..foot].iter().min().unwrap() + ints[head..foot].iter().max().unwrap() 
}
