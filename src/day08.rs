#[derive(Copy, Clone)]
enum Command {
    Nop {val: i32},
    Acc {inc: i32},
    Jmp {dist: i32},
}

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut commands: Vec<Command> = Vec::new();
    for line in input.lines() {
        let line = line.unwrap();
        if line.is_empty() {continue;}
        let command = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let c = match command[0] {
            "nop" => {Command::Nop{val: command[1].parse().unwrap()}},
            "acc" => {Command::Acc{inc: command[1].parse().unwrap()}},
            "jmp" => {Command::Jmp{dist: command[1].parse().unwrap()}},
            _ => panic!("No such command"),
        };
        commands.push(c);
    }
    let ans1 = part_one(&commands);
    let ans2 = part_two(&mut commands);
    (ans1, ans2)
}

struct Data {
    pos: usize,
    acc: i32
}

fn loops(input: &Vec<Command>) -> (bool, i32) {
    let mut history: Vec<bool> = vec![false; input.len()];
    let mut data = Data{pos: 0, acc: 0};
    let mut looped = false;
    loop {
        if data.pos >= input.len() {
            break;
        }
        if history[data.pos] {
            looped = true;
            break;
        }
        history[data.pos] = true;
        match input[data.pos] {
            Command::Nop{val: _} => data.pos += 1,
            Command::Acc{inc} => {data.acc += inc; data.pos += 1},
            Command::Jmp{dist} => data.pos = ((data.pos as i32) + dist) as usize
        }
    }
    (looped, data.acc)
}

fn part_one(input: &Vec<Command>) -> String {
    let res = loops(input);
    res.1.to_string()
}

fn part_two(input: &mut Vec<Command>) -> String {
    let mut result = -1;
    for i in 0..input.len() {
        let original_command = input[i];
        let res = match input[i] {
            Command::Nop{val} => {
                input[i] = Command::Jmp{dist: val};
                loops(&input)
            },
            Command::Jmp{dist} => {
                input[i] = Command::Nop{val: dist};
                loops(&input)
            }
            _ => continue,
        };
        if !res.0 {
            result = res.1;
            break;
        }
        input[i] = original_command;
    }
    result.to_string()
}
