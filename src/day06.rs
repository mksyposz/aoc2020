use std::collections::HashSet;

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut groups: Vec<String> = Vec::new();
    let mut current_group = String::new();
    for line in input.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            groups.push(current_group);
            current_group = String::new();
        } else {
            current_group = if current_group.is_empty() {line} else
                {current_group + ":" + &line}
        }
    }
    let ans1 = part_one(&groups);
    let ans2 = part_two(&groups);
    (ans1, ans2)
}

fn part_one(input: &Vec<String>) -> String {
    input.iter().map(|s| unique_letters(&s) ).sum::<i32>().to_string()
}

fn part_two(input: &Vec<String>) -> String {
    let mut res = 0;
    for s in input.iter() {
        let questionary_results: Vec<&str> = s.split(":").collect();
        let mut set: HashSet<char> = questionary_results[0].chars().collect();
        for r in questionary_results.iter().skip(1) {
            set = set.into_iter().filter(|c| r.chars().any(|x| c == &x)).collect()
        }
        res += set.len();
    }
    res.to_string()
}

fn unique_letters(s: &String) -> i32 {
    let set: HashSet<char> = s.chars().collect();
    let correction = if s.contains(":") {1} else {0};
    set.len() as i32 - correction
}
