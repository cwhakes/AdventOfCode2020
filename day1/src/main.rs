use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

const TARGET: i32 = 2020;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let entries: Vec<i32> = reader.lines().filter_map(|s| s.unwrap().parse().ok()).collect();
    let answer = get_answer2(&entries);
    println!("{}", answer);
}

fn get_answer(entries: &[i32]) -> i32 {
    let mut entries: Vec<_> = entries.iter().collect();
    entries.sort();
    let count = entries.len();
    let mut head = 0;
    let mut foot = count -1;

    loop {
        let sum = entries[head] + entries[foot];
        match sum.cmp(&TARGET) {
            Ordering::Less => {head +=1; continue;}
            Ordering::Greater => {foot -=1; continue;}
            Ordering::Equal => {break;}
        }
    }

    entries[head] * entries[foot]
}

fn get_answer2(entries: &[i32]) -> i32 {
    let mut entries: Vec<_> = entries.iter().collect();
    entries.sort();
    let count = entries.len();

    'middle: for middle in entries.iter() {
        let mut head = 0;
        let mut foot = count -1;
    
        loop {
            if head == foot {continue 'middle};

            let sum = entries[head] + entries[foot];
            match sum.cmp(&(TARGET - *middle)) {
                Ordering::Less => {head +=1; continue;}
                Ordering::Greater => {foot -=1; continue;}
                Ordering::Equal => {break;}
            }

        }
    
        return *middle * entries[head] * entries[foot];
    }
    panic!();
}
