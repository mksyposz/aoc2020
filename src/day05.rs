fn get_val(s: &str, c: char, start: usize) -> usize {
    s.chars().map(|l| if l == c {1} else {0}).fold((0,start), |acc, x| (acc.0 + x*acc.1, acc.1 / 2)).0
}

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut seats: Vec<usize> = Vec::new();
    for line in input.lines() {
        let seat =  line.unwrap();
        let row = &seat[..7];
        let col = &seat[7..];
        let id = get_val(row,'B', 64) * 8 + get_val(col, 'R', 4);
        seats.push(id);
    }
    let ans1 = part_one(&seats);
    let ans2 = part_two(&mut seats);
    (ans1, ans2)
}

fn part_one(input: &Vec<usize>) -> String {
    let max = input.iter().max().unwrap();
    max.to_string()
}

fn part_two(input: &mut Vec<usize>) -> String {
    input.sort();
    let mut prev: Option<usize> = None;
    for s in input.into_iter() {
        match prev {
            None => prev = Some(*s),
            Some(v) => {
                if v + 1 != *s {
                    return (v+1).to_string();
                };
                prev = Some(*s);
            }
        }
    }
    return String::from("Couldn't find solution for given input");
}
