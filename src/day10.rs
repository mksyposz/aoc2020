pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut input: Vec<i64> = input.lines().map(|x| x.unwrap()).filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap()).collect();
    input.push(0);
    input.sort();
    input.push(input[input.len()-1]+3);
    let ans1 = part_one(&input);
    let ans2 = part_two(&input);
    (ans1, ans2)
}

fn part_one(input: &Vec<i64>) -> String {
    let mut prev = 0;
    let mut res1 = 0;
    let mut res3 = 0;
    for v in input.iter() {
        if v - prev == 1 {res1 += 1};
        if v - prev == 3 {res3 += 1};
        prev = *v;
    }
    (res1*res3).to_string()
}

fn part_two(input: &Vec<i64>) -> String {
    let mut dp: Vec<i64> = vec![0; input.len()];
    dp[0] = 1;
    for (i, v) in input.iter().enumerate() {
        for j in 1..=3 {
            if (i as isize) - (j as isize) >= 0 {
                if v - input[i-j] <= 3 {
                    dp[i] += dp[i - j];
                }
            }
        }
    }
    dp[input.len()-1].to_string()
}
