use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::value,
    error::Error,
    IResult,
    multi::many0,
};

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let paths = process_input(&buf);
    
    let answer = get_answer(&paths);
    let answer2 = get_answer2(&paths, 100);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> Vec<TilePath> {
    input.lines().map(TilePath::from_str).collect()
}

fn get_answer(paths: &[TilePath]) -> usize {
    let mut floor = Floor::new();
    for path in paths {
        floor.flip(path.coords());
    }
    floor.count()
}

fn get_answer2(paths: &[TilePath], iterations: usize) -> usize {
    let mut floor = Floor::new();
    for path in paths {
        floor.flip(path.coords());
    }

    for _ in 0..iterations {
        floor.step();
    }

    floor.count()
}

#[derive(Clone, Debug)]
struct Floor(HashMap<(i32, i32), Tile>);

impl Floor {
    const ADJACENT: [(i32, i32); 6] = [
        (2, 0),
        (1, -1),
        (-1, -1),
        (-2, 0),
        (-1, 1),
        (1, 1),
    ];

    fn new() -> Self {
        Floor(HashMap::new())
    }

    fn get(&self, coords: (i32, i32)) -> Tile {
        if let Some(tile) = self.0.get(&coords) {
            *tile
        } else {
            Tile::White
        }
    }

    fn flip(&mut self, coords: (i32, i32)) {
        if let Some(tile) = self.0.get_mut(&coords) {
            tile.toggle();
        } else {
            self.0.insert(coords, Tile::Black);
        }
    }

    fn adjacents(&self, coords: (i32, i32)) -> impl Iterator<Item=(i32, i32)> + '_ {
        Floor::ADJACENT.iter().map(move |(offset_x, offset_y)| {
            let x = coords.0 + offset_x;
            let y = coords.1 + offset_y;
            (x, y)
        })
    }

    fn step(&mut self) {
        let mut new_floor = Floor::new();

        let black_tiles = self.0.iter().filter(|(_c, t)| **t == Tile::Black).map(|(c, _t)| c);
        let white_tiles: HashSet<_> = black_tiles.clone().flat_map(|c| self.adjacents(*c)).filter(|c| self.get(*c) == Tile::White).collect();

        for &coord in black_tiles {
            if let 1..=2 = self.adjacents(coord).filter(|c| self.get(*c) == Tile::Black).count() {
                new_floor.flip(coord)
            }
        }

        for &coord in white_tiles.iter() {
            if 2 == self.adjacents(coord).filter(|c| self.get(*c) == Tile::Black).count() {
                new_floor.flip(coord)
            }
        }

        *self = new_floor;
    }

    fn count(&self) -> usize {
        self.0.values().filter(|t| **t == Tile::Black).count()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    White,
    Black,
}

impl Tile {
    fn toggle(&mut self) {
        match self {
            Tile::White => *self = Tile::Black,
            Tile::Black => *self = Tile::White,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::White
    }
}

struct TilePath(Vec<Direction>);

impl TilePath {

    fn from_str(input: &str) -> Self {
        let (s, vec) = many0(parse_direction)(input).unwrap();
        assert!(s.is_empty());
        TilePath(vec)
    }

    fn coords(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;

        for direction in &self.0 {
            match direction {
                Direction::East => {x += 2},
                Direction::SouthEast => {x += 1; y -= 1},
                Direction::SouthWest => {x -= 1; y -= 1},
                Direction::West => {x -= 2},
                Direction::NorthWest => {x -= 1; y += 1},
                Direction::NorthEast => {x += 1; y += 1},
            }
        }

        (x, y)
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn parse_direction(i: &str) -> IResult<&str, Direction, Error<&str>> {
    let east = value(Direction::East, tag("e"));
    let southeast = value(Direction::SouthEast, tag("se"));
    let southwest = value(Direction::SouthWest, tag("sw"));
    let west = value(Direction::West, tag("w"));
    let northwest = value(Direction::NorthWest, tag("nw"));
    let northeast = value(Direction::NorthEast, tag("ne"));

    alt((east, southeast, southwest, west, northwest, northeast))(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer() {
        let paths = process_input(INPUT);
        assert_eq!(10, get_answer(&paths));
    }

    #[test]
    fn test_answer2() {
        let paths = process_input(INPUT);
        assert_eq!(2208, get_answer2(&paths, 100));
    }

    const INPUT: &'static str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

}
