type Tile = (i32, i32);

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead
{
    let tiles = input.lines()
         .map(|line| {
             let line = line.unwrap();
             let mut chars = line.chars();
             let (mut x, mut y) = (0, 0);
             while let Some(c) = chars.next() {
                 let (dx, dy) = match c {
                 'e' => (1, 0),
                 'w' => (-1, 0),
                 _ => match (c, chars.next().unwrap()) {
                     ('n', 'e') => (0, 1),
                     ('n', 'w') => (-1, 1),
                     ('s', 'e') => (1, -1),
                     ('s', 'w') => (0, -1),
                     _ => (0, 0),
                     }
                 };
                x += dx;
                y += dy;
             }
             (x,y)
         }).collect::<Vec<Tile>>();
    let black_tiles = get_black_tiles(&tiles);
    let ans1 = part_one(&black_tiles);
    let ans2 = part_two(black_tiles, 100);
    (ans1, ans2)
}

use std::collections::HashSet;
fn get_black_tiles(tiles: &Vec<Tile>) -> HashSet<Tile> {
    tiles.into_iter().fold(HashSet::new(), |mut acc, tile| {
        if acc.take(&tile).is_none() {
            acc.insert(*tile);
        }
        acc
    })
}

fn part_one(black_tiles: &HashSet<Tile>) -> String {
    black_tiles.len().to_string()
}

use std::collections::HashMap;
const NEIGHBOURS: [(i32,i32); 6] = [(1,0), (-1,0), (0, 1), (-1, 1), (1, -1), (0, -1)];
fn part_two(black_tiles: HashSet<Tile>, days: u32) -> String {
    let all_tiles = |blacks: &HashSet<Tile>| {
        blacks.iter().fold(HashMap::new(), |mut acc, tile| {
            NEIGHBOURS.iter().for_each(|(dx, dy)| {
                let c = acc.entry((tile.0 + dx, tile.1+dy)).or_insert(0);
                *c += 1;
            });
            acc
        })
    };
    (0..days).fold(black_tiles, |blacks, _| {
        all_tiles(&blacks)
            .iter()
            .filter_map(|(k, v)|match (blacks.contains(k), v) {
                (true, n) if (1..3).contains(n) => Some(*k),
                (false, 2) => Some(*k),
                _ => None
            }).collect()
    }).len().to_string()
}
