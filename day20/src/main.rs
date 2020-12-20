use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;


const SEA_MONSTER: &'static str =
"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let tiles = process_input(&buf);
    
    let answer = get_answer(&tiles);
    let answer2 = get_answer2(&tiles);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> HashMap<usize, Tile> {
    input.split("\n\n").filter(|s| !s.is_empty()).map(Tile::from_str).collect()
}

fn get_answer(tiles: &HashMap<usize, Tile>) -> usize {
    let edge_map = EdgeMap::from_tile_map(tiles);

    edge_map.corner_ids().product()

}

fn get_answer2(tiles: &HashMap<usize, Tile>) -> usize {
    let raw_image = RawImage::from_tile_map(tiles);
    let image = Image::from_raw_image(raw_image, tiles);
    println!("{}", image);

    let count = dbg!(image.find_sea_monsters(SEA_MONSTER));
    dbg!(image.count_waves()) - count * 15
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Tile(Vec<Vec<bool>>);

impl Tile {
    fn from_str(input: &str) -> (usize, Self) {
        let mut iter = input.lines();
        let id = iter.next().unwrap()
            .strip_prefix("Tile ").unwrap()
            .strip_suffix(":").unwrap()
            .parse::<usize>().unwrap();
        
        let image = iter.map(|line| {
            line.chars().map(|c| {
                match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!(),
                }
            }).collect()
        }).collect();

        (id, Tile(image))
    }

    fn edges(&self) -> impl Iterator<Item=Edge> {
        vec![
            Edge::from_iter(self.0[0].iter().cloned()),
            Edge::from_iter((0..10).map(|i| self.0[i][9])),
            Edge::from_iter(self.0[9].iter().cloned()).reverse(),
            Edge::from_iter((0..10).map(|i| self.0[i][0])).reverse(),
        ].into_iter()
    }

    fn index(&self, flipped: bool, rotation: u8, x: usize, y: usize) -> bool {
        if flipped {
            match rotation {
                0 => self.0[y][9-x],
                1 => self.0[9-x][9-y],
                2 => self.0[9-y][x],
                3 => self.0[x][y],
                _ => panic!(),
            }
        } else {
            match rotation {
                0 => self.0[y][x],
                1 => self.0[9-x][y],
                2 => self.0[9-y][9-x],
                3 => self.0[x][9-y],
                _ => panic!(),
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Edge(u16);

impl Edge {
    fn from_iter<I: IntoIterator<Item=bool>>(iter: I) -> Self {
        let mut edge = 0;        
        for bit in iter {
            edge = edge << 1;
            if bit {edge += 1}
        }
        Edge(edge)
    }

    fn reverse(mut self) -> Self {
        let mut edge = 0;
        for _ in 0..10 {
            edge = edge << 1;
            if 1 == (1 & self.0) {
                edge += 1;
            }
            self.0 = self.0 >> 1;
        }
        Edge(edge)
    }
}

#[derive(Debug, Default)]
struct EdgeMap(HashMap<Edge, HashSet<(usize, bool, u8)>>);

impl EdgeMap {
    fn from_tile_map(tiles: &HashMap<usize, Tile>) -> Self {
        let mut edge_map = EdgeMap::default();
        for (id, tile) in tiles.iter() {
            edge_map.insert(*id, tile.edges())
        }
        edge_map
    }

    fn insert(&mut self, id: usize, edges: impl Iterator<Item=Edge>) {
        for (rotation, edge) in edges.enumerate() {
            let rot = rotation as u8;
            self.insert_one(edge, (id, false, rot));
            self.insert_one(edge.reverse(), (id, true, rot));
        }
    }

    fn insert_one(&mut self, edge:Edge, value: (usize, bool, u8)) {
        if let Some(hashset) = self.0.get_mut(&edge) {
            hashset.insert(value);
        } else {
            self.0.insert(edge, Some(value).into_iter().collect());
        }
    }

    fn corner_ids(&self) -> impl Iterator<Item = usize> {
        let mut counts: HashMap<usize, usize> = HashMap::new();

        for (_, ids) in self.0.iter().filter(|(_, ids)| ids.len() == 1) {
            let (id, _, _) = ids.iter().next().unwrap();
            if let Some(count) = counts.get_mut(&id) {
                *count += 1;
            } else {
                counts.insert(*id, 1);
            }
        }
    
        counts.into_iter().filter(|(_id, count)| *count == 4).map(|(id, _)| id)
    }
}

#[derive(Debug)]
struct RawImage(Vec<Vec<Option<(usize, bool, u8)>>>);

impl RawImage {
    fn from_tile_map(tiles: &HashMap<usize, Tile>) -> Self {
        let side_len = (tiles.len() as f64).sqrt() as usize;

        let mut raw_image = vec![vec![None; side_len]; side_len];

        let edge_map = EdgeMap::from_tile_map(tiles);
        let corner_id = edge_map.corner_ids().next().unwrap();

        let edges: Vec<_> = tiles.get(&corner_id).unwrap().edges().collect();
        let edge0_matches = edge_map.0.get(&edges[0]).unwrap().len();
        let edge1_matches = edge_map.0.get(&edges[1]).unwrap().len();
        
        let corner_rotation = match (edge0_matches == 2, edge1_matches == 2) {
            (true, true) => 1,
            (false, true) => 0,
            (false, false) => 3,
            (true, false) => 2,
        };

        raw_image[0][0] = Some((corner_id, false, corner_rotation));
        
        for i in 1..side_len {
            let (old_id, old_flipped, old_rotation) = raw_image[i-1][0].unwrap();

            let old_edge_rotation = if old_flipped {
                (2 + old_rotation as usize) % 4
            } else {
                (4 + 2 - old_rotation as usize) % 4
            };

            let old_edge = tiles.get(&old_id).unwrap().edges().nth(old_edge_rotation).unwrap();
            let (new_id, edge_flipped, new_edge_rotation) =
                edge_map.0.get(&old_edge).unwrap().iter().filter(|(id, _, _)| *id != old_id).next().unwrap();
            // Two unflipped tiles have edges that run opposite one another
            let new_flipped = !(old_flipped ^ edge_flipped);

            let new_rotation = if new_flipped {
                (0 + new_edge_rotation) % 4
            } else {
                (4 - new_edge_rotation) % 4
            };

            raw_image[i][0] = Some((*new_id, new_flipped, new_rotation));
        }

        for row in raw_image.iter_mut() {
            let (mut old_id, mut old_flipped, mut old_rotation) = row[0].unwrap();

            for tile in row.iter_mut().skip(1) {
                let old_edge_rotation = if old_flipped {
                    (3 + old_rotation as usize) % 4
                } else {
                    (4 + 1 - old_rotation as usize) % 4
                };
    
                let old_edge = tiles.get(&old_id).unwrap().edges().nth(old_edge_rotation).unwrap();
                let (new_id, edge_flipped, new_edge_rotation) =
                    edge_map.0.get(&old_edge).unwrap().iter().filter(|(id, _, _)| *id != old_id).next().unwrap();
                // Two unflipped tiles have edges that run opposite one another
                let new_flipped = !(old_flipped ^ edge_flipped);
    
                let new_rotation = if new_flipped {
                    (3 + new_edge_rotation) % 4
                } else {
                    (4 + 3 - new_edge_rotation) % 4
                };

                *tile = Some((*new_id, new_flipped, new_rotation));
                old_id = *new_id;
                old_flipped = new_flipped;
                old_rotation = new_rotation;
            }
        }

        RawImage(raw_image)
    }
}

struct Image(Vec<Vec<bool>>);

impl Image {
    fn from_raw_image(raw_image: RawImage, tiles: &HashMap<usize, Tile>) -> Self {
        let side_len = raw_image.0.len() * 8;

        Image((0..side_len).map(|y_index| {
            (0..side_len).map(|x_index| {
                let x_0 = x_index / 8;
                let x_1 = x_index % 8 + 1;
                let y_0 = y_index / 8;
                let y_1 = y_index % 8 + 1;

                let (id, flipped, rotation) = raw_image.0[y_0][x_0].unwrap();
                let tile = tiles.get(&id).unwrap();
                tile.index(flipped, rotation, x_1, y_1)
            }).collect()
        }).collect())
    }

    fn count_waves(&self) -> usize {
        self.0.iter().flat_map(|v| v.iter()).filter(|b| **b).count()
    }

    fn find_sea_monsters(&self, sea_monster: &str) -> usize {
        let sm_len = sea_monster.lines().next().unwrap().chars().count();
        let sm_hgt = sea_monster.lines().count();

        let sea_monster: Vec<(usize, usize)> = sea_monster.lines().enumerate().flat_map(|(y_index, line)| {
            line.chars().enumerate().filter(|(_, c)| *c == '#').map(move |(x_index, _)| (x_index, y_index))
        }).collect();

        let transforms: Vec<Box<dyn Fn(&(usize, usize)) -> (usize, usize)>> = vec![
            Box::new(|&(x, y)| (x, y)),
            Box::new(|&(x, y)| (sm_len - x, y)),
            Box::new(|&(x, y)| (x, sm_hgt -  y)),
            Box::new(|&(x, y)| (sm_len - x, sm_hgt - y)),
        ];
        let transforms90: Vec<Box<dyn Fn(&(usize, usize)) -> (usize, usize)>> = vec![
            Box::new(|&(x, y)| (y, x)),
            Box::new(|&(x, y)| (y, sm_len - x)),
            Box::new(|&(x, y)| (sm_hgt - y, x)),
            Box::new(|&(x, y)| (sm_hgt - y, sm_len - x)),
        ];

        let mut count = 0;

        for transform in transforms {
            let sea_monster: Vec<_> = sea_monster.iter().map(transform).collect();

            let max_y = self.0.len() - sm_hgt;
            let max_x = self.0[0].len() - sm_len;

            let this_count = (0..max_x).flat_map(|x| (0..max_y).map(move |y| (x, y)))
                .filter(|(x, y)| {
                    self.contains_monster_at_coords(&sea_monster, *x, *y)
                }).count();
            
            if this_count > count { count = this_count }
        }

        for transform in transforms90 {
            let sea_monster: Vec<_> = sea_monster.iter().map(transform).collect();

            let max_y = self.0.len() - sm_len;
            let max_x = self.0[0].len() - sm_hgt;

            let this_count = (0..max_x).flat_map(|x| (0..max_y).map(move |y| (x, y)))
                .filter(|(x, y)| {
                    self.contains_monster_at_coords(&sea_monster, *x, *y)
                }).count();
            
            if this_count > count { count = this_count }
        }

        count
    }

    fn contains_monster_at_coords(&self, sea_monster: &[(usize, usize)], x: usize, y: usize) -> bool {
        let sea_monster: HashSet<_> = sea_monster.iter().map(|(sm_x, sm_y)| (sm_x + x, sm_y + y)).collect();

        self.0.iter().enumerate().flat_map(|(y_index, line)| {
            let sea_monster = &sea_monster;
            line.iter().enumerate().filter(move |(x_index, _pixel)| {
                sea_monster.contains(&(*x_index, y_index))
            })
        }).all(|(_, pixel)| *pixel)
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for pixel in row {
                match pixel {
                    true => write!(f, "#")?,
                    false => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(Edge(210), Edge(300).reverse());
    }

    #[test]
    fn test_edge() {
        let tiles =  process_input(INPUT);
        let tile = tiles.get(&2311).unwrap();
        let edge = tile.edges().next().unwrap();
        assert_eq!(Edge(210), edge);
    }

    #[test]
    fn test_answer() {
        let tiles = process_input(INPUT);
        assert_eq!(20899048083289, get_answer(&tiles));
    }

    #[test]
    fn test_answer2() {
        let tiles = process_input(INPUT);
        assert_eq!(273, get_answer2(&tiles));
    }


    const INPUT: &'static str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
}
