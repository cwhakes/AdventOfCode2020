use std::fs::File;
use std::io::Read;

const TOBAGGANS: [(usize, usize); 5] = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
];

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let mut product = 1;
    for (right, down) in &TOBAGGANS {
        let forest = buf.lines().map(Row::from_bytes);
        let answer = count_trees(*right, *down, forest);
        product *= answer;
    }
    println!("{}", product);
}

struct Row<'a> {
    trees: &'a[u8],
}

impl<'a> Row<'a> {
    fn from_bytes(bytes: &'a (impl AsRef<[u8]> +?Sized)) -> Self {
        Row {
            trees: bytes.as_ref()
        }
    }

    fn has_tree_at(&self, column: usize) -> bool {
        let column = column % self.trees.len();
        let square = self.trees[column]; //panics
        match square {
            b'#' => true,
            b'.' => false,
            _ => panic!(),
        }
    }
}

fn count_trees<'a>(right: usize, down: usize, forest: impl Iterator<Item = Row<'a>>) -> usize {
    let mut count = 0;
    let mut column = 0;
    for row in forest.into_iter().step_by(down) {
        if row.has_tree_at(column) {
            count += 1;
        }
        column += right;
    }
    count
}

#[cfg(test)]
mod test {
    use super:: *;

const INPUT: &'static str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn example() {
        let forest = INPUT.lines().map(Row::from_bytes);
        let answer = count_trees(3, 1, forest);
        assert_eq!(7, answer);
    }

    #[test]
    fn example2() {

        let mut product = 1;
        for (right, down) in &TOBAGGANS {
            let forest = INPUT.lines().map(Row::from_bytes);
            let answer = count_trees(*right, *down, forest);
            product *= answer;
        }
        assert_eq!(336, product);
    }
}
