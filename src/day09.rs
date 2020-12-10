use std::collections::HashMap;

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let numbers: Vec<i64> = input.lines().map(|x| x.unwrap()).filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap()).collect();
    let ans1 = part_one(&numbers);
    let ans2 = part_two(&numbers);
    (ans1, ans2)
}

fn check(set: &HashMap<i64, u32>, target: i64) -> bool {
    for (key, val) in set.iter() {
        if val > &0 {
            let k = set.get(&(target-key)).unwrap_or(&0);
            if target-key != *key && k > &0 {
                return true;
            }
        }
    }
    return false;
}

fn part_one(input: &Vec<i64>) -> String {
    let mut set: HashMap<i64, u32> = HashMap::new();
    for i in 0..25 {
        let val = set.entry(input[i]).or_insert(0);
        *val += 1;
    }
    let mut start = 0;
    let mut end = 25;
    let mut res = 0;
    loop {
        if end >= input.len() {
            break;
        }
        if !check(&set, input[end]) {
            res = input[end];
            break;
        }
        let val = set.entry(input[start]).or_insert(0);
        *val -= 1;
        start+=1;
        let val = set.entry(input[end]).or_insert(0);
        *val += 1;
        end += 1;
    }
    res.to_string()
}

fn part_two(input: &Vec<i64>) -> String {
    let mut pref_sum: HashMap<i64, isize> = HashMap::new();
    let mut curr_sum = 0;
    let target = part_one(&input).parse::<i64>().unwrap();
    let mut result = -1;
    for (pos, val) in input.iter().enumerate() {
        curr_sum += val;
        if let Some(&start) = pref_sum.get(&(curr_sum - target)) {
            let start = start as usize;
            result = input[start+1..=pos].iter().min().unwrap() +
                input[start+1..=pos].iter().max().unwrap();
            break;
        } else {
            pref_sum.insert(curr_sum, pos as isize);
        }
    }
    result.to_string()
}
