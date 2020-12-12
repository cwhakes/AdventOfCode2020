use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let mut ferry = process_input(&buf);
    
    let answer = get_answer(&ferry);
    let answer2 = get_answer2(&ferry);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> Vec<Vec<Seat>> {
    let mut ferry: Vec<_> = Some(Vec::new()).into_iter()
        .chain(input.lines()
            .map(|row| {
                Some(Seat::Null).into_iter()
                    .chain(row.chars().map(Seat::from_char))
                    .chain(Some(Seat::Null).into_iter())
                    .collect::<Vec<Seat>>()
            }))
        .chain(Some(Vec::new()).into_iter())
        .collect();
    let cols = ferry[1].len();
    ferry[0] = vec![Seat::Null; cols];
    let rows = ferry.len();
    ferry[rows - 1] = vec![Seat::Null; cols];
    ferry
}

fn get_answer(ferry: &Vec<Vec<Seat>>) -> usize {
    let mut ferry = ferry.clone();
    let mut new_ferry = step_automation(&mut ferry);
    while new_ferry != ferry {
        ferry = new_ferry;
        new_ferry = step_automation(&mut ferry);
    }
    ferry.iter().flat_map(|row| row.iter()).filter(|s| s.is_occupied()).count()
}

fn get_answer2(ferry: &Vec<Vec<Seat>>) -> usize {
    let mut ferry = ferry.clone();
    let mut new_ferry = step_automation2(&mut ferry);
    while new_ferry != ferry {
        ferry = new_ferry;
        new_ferry = step_automation2(&mut ferry);
    }
    ferry.iter().flat_map(|row| row.iter()).filter(|s| s.is_occupied()).count()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
    Null,
}

impl Seat {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            '.' => Seat::Floor,
            _ => Seat::Null
        }
    }

    fn is_occupied(&self) -> bool {
        *self == Seat::Occupied
    }
}

fn count_occupied_adj(row: usize, col: usize, ferry: &Vec<Vec<Seat>>) -> usize {
    let mut count = 0;
    if ferry[row - 1][col - 1].is_occupied() {count += 1}
    if ferry[row - 1][col    ].is_occupied() {count += 1}
    if ferry[row - 1][col + 1].is_occupied() {count += 1}
    if ferry[row    ][col - 1].is_occupied() {count += 1}
    if ferry[row    ][col + 1].is_occupied() {count += 1}
    if ferry[row + 1][col - 1].is_occupied() {count += 1}
    if ferry[row + 1][col    ].is_occupied() {count += 1}
    if ferry[row + 1][col + 1].is_occupied() {count += 1}
    count
}

fn count_occupied_sight(row: usize, col: usize, ferry: &Vec<Vec<Seat>>) -> usize {
    let mut count = 0;

    if sightline_occupied(row, col, -1, -1, ferry) {count += 1}
    if sightline_occupied(row, col, -1, 0, ferry) {count += 1}
    if sightline_occupied(row, col, -1, 1, ferry) {count += 1}
    if sightline_occupied(row, col, 0, -1, ferry) {count += 1}
    if sightline_occupied(row, col, 0, 1, ferry) {count += 1}
    if sightline_occupied(row, col, 1, -1, ferry) {count += 1}
    if sightline_occupied(row, col, 1, 0, ferry) {count += 1}
    if sightline_occupied(row, col, 1, 1, ferry) {count += 1}
    count
}

fn sightline_occupied(row: usize, col: usize, down: isize, right: isize, ferry: &Vec<Vec<Seat>>) -> bool {
    let row = (row as isize + down) as usize;
    let col = (col as isize + right) as usize;

    match ferry[row][col] {
        Seat::Empty | Seat::Null => false,
        Seat::Occupied => true,
        Seat::Floor => sightline_occupied(row, col, down, right, ferry)
    }
}

fn step_automation(ferry: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    let rows = ferry.len();
    let cols = ferry[0].len();
    let mut new_ferry = vec![vec![Seat::Null; cols]; rows];

    for row_num in 1..rows-1 {
        for col_num in 1..cols-1 {
            new_ferry[row_num][col_num] = match ferry[row_num][col_num] {
                Seat::Floor => Seat::Floor,
                Seat::Empty => if 0 == count_occupied_adj(row_num, col_num, ferry) {
                        Seat::Occupied
                    } else {
                        Seat::Empty
                    }
                Seat::Occupied => if 4 <= count_occupied_adj(row_num, col_num, ferry) {
                        Seat::Empty
                    } else {
                        Seat::Occupied
                    }
                _ => Seat::Null
            }
        }
    }

    new_ferry
}

fn step_automation2(ferry: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    let rows = ferry.len();
    let cols = ferry[0].len();
    let mut new_ferry = vec![vec![Seat::Null; cols]; rows];

    for row_num in 1..rows-1 {
        for col_num in 1..cols-1 {
            new_ferry[row_num][col_num] = match ferry[row_num][col_num] {
                Seat::Floor => Seat::Floor,
                Seat::Empty => if 0 == count_occupied_sight(row_num, col_num, ferry) {
                        Seat::Occupied
                    } else {
                        Seat::Empty
                    }
                Seat::Occupied => if 5 <= count_occupied_sight(row_num, col_num, ferry) {
                        Seat::Empty
                    } else {
                        Seat::Occupied
                    }
                _ => Seat::Null
            }
        }
    }

    new_ferry
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    #[test]
    fn test_answer() {
        let mut ferry = process_input(INPUT);
        assert_eq!(37, get_answer(&mut ferry));
    }

    #[test]
    fn test_answer2() {
        let mut ferry = process_input(INPUT);
        assert_eq!(26, get_answer2(&mut ferry));
    }
}
