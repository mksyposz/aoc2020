use std::collections::{HashSet, HashMap};

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let grid = input.lines()
                    .map(|s| s.unwrap())
                    .map(|s| s.chars().collect::<Vec<char>>())
                    .collect::<Vec<_>>();
    let mut active_fields3 = HashSet::new();
    let mut active_fields4 = HashSet::new();
    for (i, g) in grid.iter().enumerate() {
        for (j, &c) in g.iter().enumerate() {
            if c == '#' {
                active_fields3.insert(Grid3{x: j as i64, y: i as i64, z: 0});
                active_fields4.insert(Grid4{x: j as i64, y: i as i64, z: 0, w: 0});
            }
        }
    }
    let ans1 = part_one(&active_fields3);
    let ans2 = part_two(&active_fields4);
    (ans1, ans2)
}

fn part_one(af: &HashSet<Grid3>) -> String {
    let mut turn = 0;
    let mut current_field: HashSet<Grid3> = (*af).clone();
    while turn != 6 {
        let mut active_close: HashMap<Grid3, usize>= HashMap::new();
        current_field.iter().for_each(|g| g.close().iter().for_each(|&gg| {
            let k = active_close.entry(gg).or_insert(0);
            *k += 1;
        }));
        let new_field: HashSet<Grid3> = active_close.iter()
                                                    .filter(|(key, val)| {
                                                        if current_field.contains(key) && (**val == 2 || **val == 3) {true}
                                                        else if !current_field.contains(key) && **val == 3 {true}
                                                        else {false}
                                                    })
                                                    .map(|(key, _)| *key)
                                                    .collect();
        current_field = new_field.clone();
        turn += 1;
    }
    current_field.len().to_string()
}

fn part_two(af: &HashSet<Grid4>) -> String {
    let mut turn = 0;
    let mut current_field: HashSet<Grid4> = (*af).clone();
    while turn != 6 {
        let mut active_close: HashMap<Grid4, usize>= HashMap::new();
        current_field.iter().for_each(|g| g.close().iter().for_each(|&gg| {
            let k = active_close.entry(gg).or_insert(0);
            *k += 1;
        }));
        let new_field: HashSet<Grid4> = active_close.iter()
                                                    .filter(|(key, val)| {
                                                        if current_field.contains(key) && (**val == 2 || **val == 3) {true}
                                                        else if !current_field.contains(key) && **val == 3 {true}
                                                        else {false}
                                                    })
                                                    .map(|(key, _)| *key)
                                                    .collect();
        current_field = new_field.clone();
        turn += 1;
    }
    current_field.len().to_string()
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Grid3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Grid3 {
    const NEIGHBOURS: [(i64, i64, i64); 26] = [
        (-1,-1,-1),
        (-1,-1, 0),
        (-1,-1, 1),
        (-1, 0,-1),
        (-1, 0, 0),
        (-1, 0, 1),
        (-1, 1,-1),
        (-1, 1, 0),
        (-1, 1, 1),
        ( 0,-1,-1),
        ( 0,-1, 0),
        ( 0,-1, 1),
        ( 0, 0,-1),
        ( 0, 0, 1),
        ( 0, 1,-1),
        ( 0, 1, 0),
        ( 0, 1, 1),
        ( 1,-1,-1),
        ( 1,-1, 0),
        ( 1,-1, 1),
        ( 1, 0,-1),
        ( 1, 0, 0),
        ( 1, 0, 1),
        ( 1, 1,-1),
        ( 1, 1, 0),
        ( 1, 1, 1),
    ];

    fn close<'a>(&'a self) -> Vec<Grid3> {
        Grid3::NEIGHBOURS.iter().map(|(a,b,c)|
                                     Grid3{x: self.x + a, y: self.y + b, z: self.z + c}).collect()
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Grid4 {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Grid4 {
    fn close(&self) -> Vec<Grid4> {
        let mut close: Vec<Grid4> = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if !(x == 0 && y == 0 && z == 0 && w == 0) {
                            close.push(Grid4{x: self.x+x, y: self.y + y, z: self.z + z, w: self.w + w});
                        }
                    }
                }
            }
        }
        close
    }
}
