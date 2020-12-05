struct Password {
    range_min: usize,
    range_max: usize,
    letter: char,
    password: String,
}

impl Password {
    fn from_line(line: &str) -> Password {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let range = parts[0].split('-').collect::<Vec<_>>();
        let range_min = range[0].parse::<usize>().unwrap();
        let range_max = range[1].parse::<usize>().unwrap();
        let letter = parts[1].chars().nth(0).unwrap();
        let password = parts[2].to_string();
        Password {
            range_min,
            range_max,
            letter,
            password,
        }
    }
}

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut passwords: Vec<Password> = Vec::new();
    for line in input.lines() {
        let line = line.unwrap();
        passwords.push(Password::from_line(&line));
    }

    let ans1 = part_one(&passwords);
    let ans2 = part_two(&passwords);
    (ans1, ans2)
}

fn part_one(input: &Vec<Password>) -> String {
    let mut res = 0;
    for pass in input.iter() {
        let cnt = pass.password.chars().filter(|&c| c == pass.letter).count();
        if cnt >= pass.range_min && cnt <= pass.range_max {
            res += 1;
        }
    }
    res.to_string()
}

fn part_two(input: &Vec<Password>) -> String {
    let mut res = 0;
    for pass in input.iter() {
        let c1 = pass.password.chars().nth(pass.range_min-1).unwrap();
        let c2 = pass.password.chars().nth(pass.range_max-1).unwrap();
        if (c1 == pass.letter) ^ (c2 == pass.letter) {
            res += 1;
        }
    }
    res.to_string()
}
