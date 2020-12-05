use std::collections::HashSet;

fn twosum(target: isize, values: &HashSet<isize>) -> Option<isize> {
    for x in values.iter() {
        if let Some(y) = values.get(&(target-x)) {
            return Some(x*y);
        }
    }
    None
}

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut v: Vec<isize> = Vec::new();
    for line in input.lines() {
        if let Ok(num) = line {
            v.push(num.parse::<isize>().unwrap());
        }
    }
    let ans1 = part_one(&v);
    let ans2 = part_two(&v);
    (ans1, ans2)
}

fn part_one(input: &Vec<isize>) -> String {
    let prev_values: HashSet<isize> = input.iter().cloned().collect();
    let res = twosum(2020, &prev_values);
    if let Some(res) = res {
        return res.to_string();
    }
    String::from("No anwser for day 1 part 1.")
}

fn part_two(input: &Vec<isize>) -> String {
    let mut prev_values: HashSet<isize> = HashSet::new();
    for v in input.iter() {
        if let Some(res) = twosum(2020-v, &prev_values) {
            return (res*v).to_string();
        }
        prev_values.insert(v.clone());
    }
    String::from("No anwser for day 1 part 2.")
}
