use std::collections::HashMap;

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let lines = input.lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let ans1 = part_one(&lines);
    let ans2 = part_two(&lines);
    (ans1, ans2)
}

fn part_one(input: &Vec<String>) -> String {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask_ones = 0;
    let mut mask_zeroes = 0;
    for line in input.iter() {
        let s: Vec<&str> = line.split("=").collect();
        match &s[0][..4] {
            "mask" => {
                mask_ones = 0;
                mask_zeroes = 0;
                s[1].chars().for_each(|x| {
                    mask_ones *= 2;
                    mask_zeroes *= 2;
                    match x {
                        '1' => {
                            mask_ones += 1;
                            mask_zeroes += 1;
                        },
                        'X' => mask_zeroes += 1,
                        _ => {},
                    };
                });
            },
            "mem[" => {
                let mut num: u64 = 0;
                s[0][4..].chars().for_each(|x| match x.to_digit(10) {
                    Some(v) => num = 10*num + (v as u64),
                    None => {},
                });
                let mut value = s[1][1..].parse::<u64>().unwrap();
                value = (value & mask_zeroes) | mask_ones;
                memory.insert(num, value);
            },
            _ => unreachable!(),
        }
    }
    memory.values().sum::<u64>().to_string()
}

fn part_two(input: &Vec<String>) -> String {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut masks: Vec<(u64, u64)> = Vec::new();
    for line in input.iter() {
        let s: Vec<&str> = line.split("=").collect();
        match &s[0][..4] {
            "mask" => {
                masks = vec![(0,0)];
                s[1].chars().for_each(|x| {
                    masks.iter_mut().for_each(|x| *x = (2*x.0, 2*x.1));
                    match x {
                        '0' => {
                            masks.iter_mut().for_each(|x| *x = (x.0+1, x.1));
                        }
                        '1' => {
                            masks.iter_mut().for_each(|x| *x = (x.0+1, x.1+1));
                        },
                        'X' => {
                            let mut new_masks: Vec<(u64,u64)> = masks.iter().map(|x| (x.0+1, x.1+1)).collect();
                            masks.append(&mut new_masks);
                        },
                        _ => {},
                    }
                });
            },
            "mem[" => {
                let mut num: u64 = 0;
                s[0][4..].chars().for_each(|x| match x.to_digit(10) {
                    Some(v) => num = 10*num + (v as u64),
                    None => {},
                });
                let value = s[1][1..].parse::<u64>().unwrap();
                masks.iter().for_each(|(z, o)| {memory.insert( (num & z) | o, value); ()} );
            },
            _ => unreachable!(),
        }
    }
    memory.values().sum::<u64>().to_string()
}
