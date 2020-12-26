use std::collections::HashMap;
const TILE_ID: i32 = 0;
const TILE: i32 = 1;

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut input_part = TILE_ID;
    let mut tile: Vec<Vec<char>> = Vec::new();
    let mut id = 0;
    let mut images: HashMap<u64, Image> = HashMap::new();
    for line in input.lines() {
        let line = line.unwrap();
        match input_part {
            TILE_ID => {
                let len = line.len();
                id = line[5..(len-1)].parse::<u64>().unwrap();
                input_part = TILE;
            }
            TILE => {
                if line.is_empty() {
                    images.insert(id, Image::from_vec(&tile));
                    tile = Vec::new();
                    input_part = TILE_ID;
                } else {
                    tile.push(line.chars().collect());
                }
            },
            _ => unreachable!(),
        }
    }
    let mut hm_hash: HashMap<u64, Vec<u64>> = HashMap::new();
    images.iter().for_each(|(k,v)| v.hashed_edges.iter().for_each(|(c,ac)| {
        hm_hash.entry(*c).or_insert(Vec::new()).push(*k);
        hm_hash.entry(*ac).or_insert(Vec::new()).push(*k);
    }));
    let corners = hm_hash.values()
                         .filter(|v| v.len() == 1)
                         .map(|v| v[0])
                         .fold(HashMap::new(), |mut acc, v| {
                             *acc.entry(v).or_insert(0) += 1;
                             acc
                         }).iter()
                           .filter(|&(_,v)| *v == 4)
                           .map(|(k,_)| *k)
                           .collect::<Vec<u64>>();
    let ans1 = part_one(&corners);
    let mut full_img = create_full_image(&corners, &mut images, &hm_hash);
    let ans2 = part_two(&mut full_img);
    (ans1, ans2)
}

fn part_one(corners: &Vec<u64>) -> String {
    corners.iter().product::<u64>().to_string()
}

fn part_two(img: &mut Image) -> String {
    let monster_starts = loop {
        let monster_starts = get_monster_starts(&img);
        if monster_starts.is_empty() == false {
            break monster_starts;
        }
        img.flip_r();
        let monster_starts = get_monster_starts(&img);
        if monster_starts.is_empty() == false {
            break monster_starts;
        }
        img.flip_r();
        img.flip_c();
        let monster_starts = get_monster_starts(&img);
        if monster_starts.is_empty() == false {
            break monster_starts;
        }
        img.flip_c();
        img.rot_l();
    };
    let total_tiles =
    img.tile.iter().map(|v| v.iter().map(|&c| if c == '#' {1} else {0}).sum::<u64>()).sum::<u64>();
    let monster_tiles = count_monster_tiles(&monster_starts);
    (total_tiles - monster_tiles).to_string()
}

fn create_full_image(corners: &Vec<u64>, images: &mut HashMap<u64, Image>, hm: &HashMap<u64, Vec<u64>>) -> Image {
    let corner = images.get_mut(&corners[0]).unwrap();
    //let debug_tile = |tile: &Image| tile.tile.iter().map(|v| v.iter().collect::<String>()).collect::<Vec<String>>().join("\n");
    loop {
        let left = corner.hashed_edges[Image::LEFT].0;
        let top = corner.hashed_edges[Image::TOP].0;
        if hm.get(&left).unwrap().len() == 1 &&
            hm.get(&top).unwrap().len() == 1 {
                break;
            }
        corner.rot_l();
    }
    let mut img: Vec<Vec<u64>> = Vec::new();
    let mut row = vec![corners[0]];
    loop {
        match row.last() {
            Some(v) => {
                let hash = images.get(&v).unwrap().hashed_edges[Image::RIGHT];
                let id = hm.get(&hash.0).unwrap().iter().filter(|&a| v != a).nth(0);
                if let Some(id) = id {
                    let pic = images.get_mut(id).unwrap();
                    pic.match_left(hash);
                    row.push(*id);
                } else {
                    img.push(row);
                    row = Vec::new();
                }
            },
            None => {
                let v = img.last().unwrap()[0];
                let hash = images.get(&v).unwrap().hashed_edges[Image::BOT];
                let id = hm.get(&hash.0).unwrap().iter().filter(|&a| v != *a).nth(0);
                if let Some(id) = id {
                    let pic = images.get_mut(id).unwrap();
                    pic.match_top(hash);
                    row.push(*id);
                } else {
                    break;
                }
            },
        }
    }
    let h = images.get(&img[0][0]).unwrap().height;
    let full_img = img.iter().fold(Vec::new(), |mut acc, row| {
        let mut v = (0..(h-2)).map(|i| {
            row.iter().fold(Vec::new(), |mut acc, id| {
                let im = images.get(&id).unwrap().trim_edges();
                acc.append(&mut im.tile[i].clone());
                acc
            })
        }).collect::<Vec<_>>();
        acc.append(&mut v);
        acc
    });
    let full_img = Image::from_vec_no_hash(&full_img);
    full_img
}

fn get_monster() -> Vec<Vec<char>> {
    vec![
        "                  # ".chars().collect::<Vec<char>>(),
        "#    ##    ##    ###".chars().collect::<Vec<char>>(),
        " #  #  #  #  #  #   ".chars().collect::<Vec<char>>(),
    ]
}

fn is_monster_start(img: &Image, (x,y): (usize, usize)) -> bool {
    let monster = get_monster();
    for sy in 0..monster.len() {
        for sx in 0..monster[sy].len() {
            if img.tile.get(y+sy) == None {return false;}
            else if img.tile[y+sy].get(x+sx) == None {return false;}
            else if monster[sy][sx] == '#' && img.tile[y+sy][x+sx] !=  '#' {return false;}
        }
    }
    true
}

fn get_monster_starts(img: &Image) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for y in 0..img.tile.len() {
        for x in 0..img.tile[y].len() {
            if is_monster_start(&img, (x,y)) {
                result.push((x,y));
            }
        }
    }
    result
}

use std::collections::HashSet;
fn count_monster_tiles(monster_starts: &Vec<(usize, usize)>) -> u64 {
    let mut tiles = HashSet::new();
    let monster = get_monster();
    for (x,y) in monster_starts.iter() {
        for my in 0..monster.len() {
            for mx in 0..monster[my].len() {
                if monster[my][mx] == '#' {
                    tiles.insert((y+my, x+mx));
                }
            }
        }
    }
    tiles.len() as u64
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Image {
    tile: Vec<Vec<char>>,
    width: usize,
    height: usize,
    hashed_edges: [(u64, u64); 4],
}

impl Image {
    const TOP: usize   = 0;
    const RIGHT: usize = 1;
    const BOT: usize   = 2;
    const LEFT: usize  = 3;

    fn from_vec_no_hash(tile: &Vec<Vec<char>>) -> Self {
        let w = tile[0].len();
        let h = tile.len();
        Self {
            tile: tile.clone(),
            width: w,
            height: h,
            hashed_edges: [(0,0); 4],
        }
    }

    fn from_vec(tile: &Vec<Vec<char>>) -> Self {
        let w = tile[0].len();
        let h = tile.len();
        Self {
            tile: tile.clone(),
            width: w,
            height: h,
            hashed_edges: [
                Image::hash_top(&tile, w),
                Image::hash_right(&tile, w, h),
                Image::hash_bot(&tile, w, h),
                Image::hash_left(&tile, h),
            ]
        }
    }
    fn hash_top(tile: &Vec<Vec<char>>, width: usize) -> (u64, u64) {
        (Image::hash((0..width).map(|i| tile[0][i])),
         Image::hash((0..width).rev().map(|i| tile[0][i])))
    }

    fn hash_right(tile: &Vec<Vec<char>>, width: usize, height: usize) -> (u64, u64) {
        (Image::hash((0..height).map(|i| tile[i][width-1])),
         Image::hash((0..height).rev().map(|i| tile[i][width-1])))
    }

    fn hash_bot(tile: &Vec<Vec<char>>, width: usize, height: usize) -> (u64, u64) {
        (Image::hash((0..width).rev().map(|i| tile[height-1][i])),
         Image::hash((0..width).map(|i| tile[height-1][i])))
    }

    fn hash_left(tile: &Vec<Vec<char>>, height: usize) -> (u64, u64) {
        (Image::hash((0..height).rev().map(|i| tile[i][0])),
         Image::hash((0..height).map(|i| tile[i][0])))
    }

    fn hash<I>(it: I) -> u64
    where
        I: Iterator<Item = char>,
    {
        it.fold(0, |acc, c| (2*acc) + if c == '#' {1} else {0})
    }

    fn rot_r(&mut self) {
        self.tile = (0..self.width).map(|i| (0..self.height).rev().map(|j| self.tile[j][i]).collect::<Vec<char>>()).collect();
        self.hashed_edges = [
            self.hashed_edges[Image::LEFT],
            self.hashed_edges[Image::TOP],
            self.hashed_edges[Image::RIGHT],
            self.hashed_edges[Image::BOT]
        ];
        let w = self.width;
        self.width = self.height;
        self.height = w;
    }

    fn rot_l(&mut self) {
        self.tile = (0..self.width).rev().map(|i| (0..self.height).map(|j| self.tile[j][i]).collect::<Vec<char>>()).collect();
        self.hashed_edges = [
            self.hashed_edges[Image::RIGHT],
            self.hashed_edges[Image::BOT],
            self.hashed_edges[Image::LEFT],
            self.hashed_edges[Image::TOP]
        ];
        let w = self.width;
        self.width = self.height;
        self.height = w;
    }

    fn flip_r(&mut self) {
        self.tile = (0..self.height).map(|i| self.tile[i].iter().copied().rev().collect()).collect();
        self.hashed_edges = [
            (self.hashed_edges[Image::TOP].1, self.hashed_edges[Image::TOP].0),
            (self.hashed_edges[Image::LEFT].1, self.hashed_edges[Image::LEFT].0),
            (self.hashed_edges[Image::BOT].1, self.hashed_edges[Image::BOT].0),
            (self.hashed_edges[Image::RIGHT].1, self.hashed_edges[Image::RIGHT].0),
        ]
    }

    fn flip_c(&mut self) {
        self.tile = (0..self.height).rev().map(|i| self.tile[i].iter().copied().collect()).collect();
        self.hashed_edges = [
            (self.hashed_edges[Image::BOT].1, self.hashed_edges[Image::BOT].0),
            (self.hashed_edges[Image::RIGHT].1, self.hashed_edges[Image::RIGHT].0),
            (self.hashed_edges[Image::TOP].1, self.hashed_edges[Image::TOP].0),
            (self.hashed_edges[Image::LEFT].1, self.hashed_edges[Image::LEFT].0),
        ]
    }

    fn match_left(&mut self, pp: (u64, u64)) {
        loop {
            if pp == self.hashed_edges[Image::LEFT] {
                self.flip_c();
                break;
            } else if (pp.1, pp.0) == self.hashed_edges[Image::LEFT] {
                break;
            }
            self.rot_r();
        }
    }

    fn match_top(&mut self, pp: (u64, u64)) {
        loop {
            if pp == self.hashed_edges[Image::TOP] {
                self.flip_r();
                break;
            } else if (pp.1, pp.0) == self.hashed_edges[Image::TOP] {
                break;
            }
            self.rot_r();
        }
    }

    fn trim_edges(&self) -> Self {
        let mut tile: Vec<Vec<char>> = self.tile.iter().skip(1).map(|v| v.iter().skip(1).copied().collect()).collect();
        tile.pop();
        for i in 0..(self.height - 2) {
            tile[i].pop();
        }
        Self {
            tile,
            width: self.width - 2,
            height: self.height - 2,
            hashed_edges: [(0,0); 4],
        }
    }
}

#[test]
fn test_rot_r() {
    let mut tile = Image::from_vec(&vec![vec!['.', '#', '.'],
                                         vec!['.', '.', '.'],
                                         vec!['.', '.', '.']]);
    let tile_rot_r = Image::from_vec(&vec![vec!['.', '.', '.'],
                                           vec!['.', '.', '#'],
                                           vec!['.', '.', '.']]);
    tile.rot_r();
    assert_eq!(tile, tile_rot_r);
}

#[test]
fn test_rot_l() {
    let mut tile = Image::from_vec(&vec![vec!['.', '#', '.'],
                                         vec!['.', '.', '.'],
                                         vec!['.', '.', '.']]);
    let tile_rot_l = Image::from_vec(&vec![vec!['.', '.', '.'],
                                           vec!['#', '.', '.'],
                                           vec!['.', '.', '.']]);
    tile.rot_l();
    assert_eq!(tile, tile_rot_l);
}

#[test]
fn test_flip_c() {
    let mut tile = Image::from_vec(&vec![vec!['.', '#', '.'],
                                         vec!['.', '.', '.'],
                                         vec!['.', '.', '.']]);
    let tile_flipped = Image::from_vec(&vec![vec!['.', '.', '.'],
                                             vec!['.', '.', '.'],
                                             vec!['.', '#', '.']]);
    tile.flip_c();
    assert_eq!(tile, tile_flipped);
}

#[test]
fn test_flip_r() {
    let mut tile = Image::from_vec(&vec![vec!['#', '.', '.'],
                                         vec!['#', '.', '.'],
                                         vec!['#', '.', '.']]);
    let tile_flipped = Image::from_vec(&vec![vec!['.', '.', '#'],
                                             vec!['.', '.', '#'],
                                             vec!['.', '.', '#']]);
    tile.flip_r();
    assert_eq!(tile, tile_flipped);
}

#[test]
fn test_find_monster() {
    let tile = vec![
    ".####...#####..#...###..".chars().collect::<Vec<char>>(),
    "#####..#..#.#.####..#.#.".chars().collect::<Vec<char>>(),
    ".#.#...#.###...#.##.##..".chars().collect::<Vec<char>>(),
    "#.#.##.###.#.##.##.#####".chars().collect::<Vec<char>>(),
    "..##.###.####..#.####.##".chars().collect::<Vec<char>>(),
    "...#.#..##.##...#..#..##".chars().collect::<Vec<char>>(),
    "#.##.#..#.#..#..##.#.#..".chars().collect::<Vec<char>>(),
    ".###.##.....#...###.#...".chars().collect::<Vec<char>>(),
    "#.####.#.#....##.#..#.#.".chars().collect::<Vec<char>>(),
    "##...#..#....#..#...####".chars().collect::<Vec<char>>(),
    "..#.##...###..#.#####..#".chars().collect::<Vec<char>>(),
    "....#.##.#.#####....#...".chars().collect::<Vec<char>>(),
    "..##.##.###.....#.##..#.".chars().collect::<Vec<char>>(),
    "#...#...###..####....##.".chars().collect::<Vec<char>>(),
    ".#.##...#.##.#.#.###...#".chars().collect::<Vec<char>>(),
    "#.###.#..####...##..#...".chars().collect::<Vec<char>>(),
    "#.###...#.##...#.######.".chars().collect::<Vec<char>>(),
    ".###.###.#######..#####.".chars().collect::<Vec<char>>(),
    "..##.#..#..#.#######.###".chars().collect::<Vec<char>>(),
    "#.#..##.########..#..##.".chars().collect::<Vec<char>>(),
    "#.#####..#.#...##..#....".chars().collect::<Vec<char>>(),
    "#....##..#.#########..##".chars().collect::<Vec<char>>(),
    "#...#.....#..##...###.##".chars().collect::<Vec<char>>(),
    "#..###....##.#...##.##.#".chars().collect::<Vec<char>>(),
    ];
    let good_tile =
        vec![
            ".####...#####..#...###..".chars().collect::<Vec<char>>(),
            "#####..#..#.#.####..#.#.".chars().collect::<Vec<char>>(),
            ".#.#...#.###...#.##.##..".chars().collect::<Vec<char>>(),
            "#.#.##.###.#.##.##.#####".chars().collect::<Vec<char>>(),
            "..##.###.####..#.####.##".chars().collect::<Vec<char>>(),
            "...#.#..##.##...#..#..##".chars().collect::<Vec<char>>(),
            "#.##.#..#.#..#..##.#.#..".chars().collect::<Vec<char>>(),
            ".###.##.....#...###.#...".chars().collect::<Vec<char>>(),
            "#.####.#.#....##.#..#.#.".chars().collect::<Vec<char>>(),
            "##...#..#....#..#...####".chars().collect::<Vec<char>>(),
            "..#.##...###..#.#####..#".chars().collect::<Vec<char>>(),
            "....#.##.#.#####....#...".chars().collect::<Vec<char>>(),
            "..##.##.###.....#.##..#.".chars().collect::<Vec<char>>(),
            "#...#...###..####....##.".chars().collect::<Vec<char>>(),
            ".#.##...#.##.#.#.###...#".chars().collect::<Vec<char>>(),
            "#.###.#..####...##..#...".chars().collect::<Vec<char>>(),
            "#.###...#.##...#.######.".chars().collect::<Vec<char>>(),
            ".###.###.#######..#####.".chars().collect::<Vec<char>>(),
            "..##.#..#..#.#######.###".chars().collect::<Vec<char>>(),
            "#.#..##.########..#..##.".chars().collect::<Vec<char>>(),
            "#.#####..#.#...##..#....".chars().collect::<Vec<char>>(),
            "#....##..#.#########..##".chars().collect::<Vec<char>>(),
            "#...#.....#..##...###.##".chars().collect::<Vec<char>>(),
            "#..###....##.#...##.##.#".chars().collect::<Vec<char>>(),
        ];
    assert_eq!(tile, good_tile);
}
