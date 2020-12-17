use std::fmt;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let grid = process_input(&buf);
    println!("{}", grid);
    let answer = get_answer(grid.clone());

    let grid = process_input2(&buf);
    let answer2 = get_answer2(grid.clone());

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> Grid3d<bool> {
    let mut cols = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for character in line.chars() {
            match character {
                '#' => row.push(true),
                '.' => row.push(false),
                _ => panic!(),
            }
        }
        cols.push(row);
    }
    Grid3d::from_vecs(vec![cols])
}

fn process_input2(input: &str) -> Grid4d<bool> {
    let mut cols = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for character in line.chars() {
            match character {
                '#' => row.push(true),
                '.' => row.push(false),
                _ => panic!(),
            }
        }
        cols.push(row);
    }
    Grid4d::from_vecs(vec![vec![cols]])
}

fn get_answer(mut grid: Grid3d<bool>) -> usize {
    for _ in 0..6 {
        grid.step();
        println!("{}", grid);
    }
    grid.iter_mut().filter(|b| **b).count()
}

fn get_answer2(mut grid: Grid4d<bool>) -> usize {
    //println!("{}", grid);
    grid.step();
    //println!("{}", grid);

    for _ in 1..6 {
        grid.step();
    }
    grid.iter_mut().filter(|b| **b).count()
}

#[derive(Debug, Clone)]
struct Grid3d<T> {
    allocation: Vec<T>,

    row_stride: usize,
    row_offset: usize,
    row_len: usize,

    col_stride: usize,
    col_offset: usize,
    col_len: usize,

    pil_offset: usize,
    pil_len: usize,
}

impl<T: Default> Grid3d<T> {
    fn from_vecs(elements: Vec<Vec<Vec<T>>>) -> Self {
        let pil_len = elements.len();
        let col_len = elements[0].len();
        let row_len = elements[0][0].len();

        //let buffer = *vec![pil_len, row_len, col_len].iter().max().unwrap();
        let buffer = 7;

        let row_stride = 2 * buffer + row_len;
        let col_stride = (2 * buffer + col_len) * row_stride;

        let mut allocation: Vec<T> = Vec::new();
        allocation.resize_with((2 * buffer + pil_len) * col_stride, Default::default);

        let row_offset = buffer;
        let col_offset = buffer * row_stride;
        let pil_offset = dbg!(buffer * col_stride);

        println!("{}", allocation.len());

        for (pil_i, column) in elements.into_iter().enumerate() {
            assert_eq!(col_len, column.len());
            let current_col_offset = col_offset + pil_i * col_stride + pil_offset;
            for (col_i, row) in column.into_iter().enumerate() {
                assert_eq!(row_len, row.len());
                let current_row_offset =  row_offset + col_i * row_stride + current_col_offset;

                let buffer = &mut allocation[current_row_offset..(current_row_offset+row_len)];
                for (location, element) in buffer.into_iter().zip(row.into_iter()) {
                    *location = element;
                }
            }
        }

        let row_offset = 0;
        let col_offset = 0;
        let pil_offset = 0;

        let row_len = row_stride;
        let col_len = col_stride/row_stride;
        let pil_len = allocation.len()/col_stride;

        Grid3d {
            allocation,

            row_stride,
            row_offset,
            row_len,

            col_stride,
            col_offset,
            col_len,

            pil_offset,
            pil_len
        }
    }

    fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        self.allocation.iter_mut()
    }
}

impl<T>  std::ops::Index<(usize, usize, usize)> for Grid3d<T> {
    type Output = T;

    fn index(&self, (pil, col, row): (usize, usize, usize,)) -> &T {

        let current_col_offset = self.col_offset + pil * self.col_stride + self.pil_offset;
        let current_row_offset = self.row_offset + col * self.row_stride + current_col_offset;
        &self.allocation[row + current_row_offset]

    }
}

impl<T>  std::ops::IndexMut<(usize, usize, usize)> for Grid3d<T> {
    fn index_mut(&mut self, (pil, col, row): (usize, usize, usize,)) -> &mut T {

        let current_col_offset = self.col_offset + pil * self.col_stride + self.pil_offset;
        let current_row_offset = self.row_offset + col * self.row_stride + current_col_offset;
        &mut self.allocation[row + current_row_offset]

    }
}

impl Grid3d<bool> {
    fn step(&mut self) {
        let mut new_grid = self.clone();

        for pil_i in 1..self.pil_len-1 {
            for col_i in 1..self.col_len-1 {
                for row_i in 1..self.row_len-1 {
                    let mut count = 0;
                    for pil_offset in -1..=1 {
                        for col_offset in -1..=1 {
                            for row_offset in -1..=1 {
                                if self[(
                                    (pil_i as isize + pil_offset) as usize,
                                    (col_i as isize + col_offset) as usize,
                                    (row_i as isize + row_offset) as usize,
                                )] {
                                    count += 1;
                                }
                            }
                        }
                    }
                    match self[(pil_i, col_i, row_i)] {
                        true => new_grid[(pil_i, col_i, row_i)] = count == 3 || count == 4,
                        false => new_grid[(pil_i, col_i, row_i)] = count == 3,
                    }
                }
            }
        }

        *self = new_grid;
    }
}

impl std::fmt::Display for Grid3d<bool> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for pil_i in 0..self.pil_len {
            for col_i in 0..self.col_len {
                for row_i in 0..self.row_len {
                    match self[(pil_i, col_i, row_i)] {
                        true => write!(f, "#")?,
                        false => write!(f, ".")?,
                    }
                }
                writeln!(f)?;
            }
            writeln!(f)?;
            writeln!(f)?;
        }
        writeln!(f, "~~~~~~~~~~~~~~~")?;
        Ok(())
    }
}


#[derive(Debug, Clone)]
struct Grid4d<T> {
    allocation: Vec<T>,

    row_stride: usize,
    row_offset: usize,
    row_len: usize,

    col_stride: usize,
    col_offset: usize,
    col_len: usize,

    pil_stride: usize,
    pil_offset: usize,
    pil_len: usize,

    pan_offset: usize,
    pan_len: usize,
}

impl<T: Default> Grid4d<T> {
    fn from_vecs(elements: Vec<Vec<Vec<Vec<T>>>>) -> Self {
        let pan_len = elements.len();
        let pil_len = elements[0].len();
        let col_len = elements[0][0].len();
        let row_len = elements[0][0][0].len();

        //let buffer = *vec![pan_len, pil_len, row_len, col_len].iter().max().unwrap();
        let buffer = 7;

        let row_stride = 2 * buffer + row_len;
        let col_stride = (2 * buffer + col_len) * row_stride;
        let pil_stride = (2 * buffer + pil_len) * col_stride;

        let mut allocation: Vec<T> = Vec::new();
        allocation.resize_with((2 * buffer + pan_len) * pil_stride, Default::default);

        let row_offset = buffer;
        let col_offset = buffer * row_stride;
        let pil_offset = buffer * col_stride;
        let pan_offset = buffer * pil_stride;

        for (pan_i, pillar) in elements.into_iter().enumerate() {
            assert_eq!(pil_len, pillar.len());
            let current_pil_offset = pil_offset + pan_i * pil_stride + pan_offset;
            for (pil_i, column) in pillar.into_iter().enumerate() {
                assert_eq!(col_len, column.len());
                let current_col_offset = col_offset + pil_i * col_stride + current_pil_offset;
                for (col_i, row) in column.into_iter().enumerate() {
                    assert_eq!(row_len, row.len());
                    let current_row_offset =  row_offset + col_i * row_stride + current_col_offset;

                    let buffer = &mut allocation[current_row_offset..(current_row_offset+row_len)];
                    for (location, element) in buffer.into_iter().zip(row.into_iter()) {
                        *location = element;
                    }
                }
            }
        }

        let row_offset = 0;
        let col_offset = 0;
        let pil_offset = 0;
        let pan_offset = 0;

        let row_len = row_stride;
        let col_len = col_stride/row_stride;
        let pil_len = pil_stride/col_stride;
        let pan_len = allocation.len()/pil_stride;

        Grid4d {
            allocation,

            row_stride,
            row_offset,
            row_len,

            col_stride,
            col_offset,
            col_len,

            pil_stride,
            pil_offset,
            pil_len,

            pan_offset,
            pan_len,
        }
    }

    fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        self.allocation.iter_mut()
    }
}

impl<T>  std::ops::Index<(usize, usize, usize, usize)> for Grid4d<T> {
    type Output = T;

    fn index(&self, (pan, pil, col, row): (usize, usize, usize, usize)) -> &T {

        let current_pil_offset = self.pil_offset + pan * self.pil_stride + self.pan_offset;
        let current_col_offset = self.col_offset + pil * self.col_stride + current_pil_offset;
        let current_row_offset = self.row_offset + col * self.row_stride + current_col_offset;
        &self.allocation[row + current_row_offset]

    }
}

impl<T>  std::ops::IndexMut<(usize, usize, usize, usize)> for Grid4d<T> {
    fn index_mut(&mut self, (pan, pil, col, row): (usize, usize, usize, usize)) -> &mut T {

        let current_pil_offset = self.pil_offset + pan * self.pil_stride + self.pan_offset;
        let current_col_offset = self.col_offset + pil * self.col_stride + current_pil_offset;
        let current_row_offset = self.row_offset + col * self.row_stride + current_col_offset;
        &mut self.allocation[row + current_row_offset]
    }
}

impl Grid4d<bool> {
    fn step(&mut self) {
        let mut new_grid = self.clone();

        for pan_i in 1..self.pan_len-1 {
            for pil_i in 1..self.pil_len-1 {
                for col_i in 1..self.col_len-1 {
                    for row_i in 1..self.row_len-1 {
                        let mut count = 0;
                        for pan_offset in -1..=1 {
                            for pil_offset in -1..=1 {
                                for col_offset in -1..=1 {
                                    for row_offset in -1..=1 {
                                        if self[(
                                            (pan_i as isize + pan_offset) as usize,
                                            (pil_i as isize + pil_offset) as usize,
                                            (col_i as isize + col_offset) as usize,
                                            (row_i as isize + row_offset) as usize,
                                        )] {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
                        match self[(pan_i, pil_i, col_i, row_i)] {
                            true => new_grid[(pan_i, pil_i, col_i, row_i)] = count == 3 || count == 4,
                            false => new_grid[(pan_i, pil_i, col_i, row_i)] = count == 3,
                        }
                    }
                }
            }
        }

        *self = new_grid;
    }
}

impl std::fmt::Display for Grid4d<bool> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for pan_i in 0..self.pan_len {
            for pil_i in 0..self.pil_len {
                for col_i in 0..self.col_len {
                    for row_i in 0..self.row_len {
                        match self[(pan_i, pil_i, col_i, row_i)] {
                            true => write!(f, "#")?,
                            false => write!(f, ".")?,
                        }
                    }
                    writeln!(f)?;
                }
                writeln!(f)?;
                writeln!(f)?;
            }
            writeln!(f, "~~~~~~~~~~~~~~~")?;
        }
        writeln!(f, "---------------")?;
        writeln!(f, "---------------")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "\
.#.
..#
###";

    #[test]
    fn test_answer() {
        let grid = process_input(INPUT);
        println!("{}", grid);
        assert_eq!(112, get_answer(grid));
    }

    const INPUT2: &'static str = "\
.#.
..#
###";

    #[test]
    fn test_answer2() {
        let grid = process_input2(INPUT2);
        assert_eq!(848, get_answer2(grid));
    }
}
