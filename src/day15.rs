pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let buf = input.lines().next().unwrap().unwrap();
    let start_numbers = buf.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let ans1 = part_one(&start_numbers);
    let ans2 = part_two(&start_numbers);
    (ans1, ans2)
}


fn play_the_game(s: &Vec<i64>, end: usize) -> i64 {
    use std::collections::HashMap;
    let mut memory: HashMap<i64, usize> = HashMap::new();
    let mut turn = 1;
    let mut curr;
    let mut next = 0;
    while turn < end {
        curr = if turn <= s.len() {s[turn-1]} else {next};
        next = match memory.get(&curr) {
            Some(v) => (turn - v) as i64,
            None => 0,
        };
        memory.insert(curr, turn);
        turn += 1;
    }
    next
}

fn part_one(s: &Vec<i64>) -> String {
    play_the_game(s, 2020).to_string()
}

fn part_two(s: &Vec<i64>) -> String {
    play_the_game(s, 30000000).to_string()
}
