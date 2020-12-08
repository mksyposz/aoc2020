#[derive(Copy, Clone)]
enum Command {
    Nop (i32),
    Acc (i32),
    Jmp (i32),
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
            "nop" => Command::Nop(command[1].parse().unwrap()),
            "acc" => Command::Acc(command[1].parse().unwrap()),
            "jmp" => Command::Jmp(command[1].parse().unwrap()),
            _ => panic!("No such command"),
        };
        commands.push(c);
    }
    let ans1 = part_one(&commands);
    let ans2 = part_two_speedy(&mut commands);
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
            Command::Nop(_) => data.pos += 1,
            Command::Acc(inc) => {data.acc += inc; data.pos += 1},
            Command::Jmp(dist) => data.pos = ((data.pos as i32) + dist) as usize
        }
    }
    (looped, data.acc)
}

fn part_one(input: &Vec<Command>) -> String {
    let res = loops(input);
    res.1.to_string()
}

fn dfs(pos: usize, graph: &Vec<Command>, vis: &mut Vec<bool>, can_reach_end: &mut Vec<bool>) {
    if pos >= graph.len() {can_reach_end[pos] = true; return;}
    if vis[pos] {
        return;
    }
    vis[pos] = true;
    let next = match graph[pos] {
        Command::Acc(_) | Command::Nop(_) => pos + 1,
        Command::Jmp(dist) => ((pos as i32) + dist) as usize,
    };
    if next < graph.len() {
        dfs(next, graph, vis, can_reach_end);
        if can_reach_end[next] {
            can_reach_end[pos] = true;
        }
    } else {
        can_reach_end[pos] = true;
    }
}

fn next_pos(j: i32, curr_pos: usize) -> usize {
    ((curr_pos as i32) + j) as usize
}

fn part_two_speedy(input: &mut Vec<Command>) -> String {
    let mut history: Vec<bool> = vec![false; input.len()];
    let mut can_reach_end: Vec<bool> = vec![false; input.len()];
    input.iter().enumerate().for_each(|(pos, _)| dfs(pos, &input, &mut history, &mut can_reach_end));
    let mut data = Data{pos: 0, acc: 0};
    let mut jumped = false;
    loop {
        if data.pos >= input.len() {
            break;
        }
        match input[data.pos] {
            Command::Nop(dist) => {
                if !jumped && can_reach_end[next_pos(dist, data.pos)] {
                    jumped = true;
                    data.pos = next_pos(dist, data.pos);
                } else {
                    data.pos = data.pos + 1;
                }
            },
            Command::Acc(inc) => {data.acc += inc; data.pos += 1},
            Command::Jmp(dist) => {
                if !jumped && can_reach_end[data.pos+1] {
                    jumped = true;
                    data.pos = data.pos + 1;
                } else {
                    data.pos = next_pos(dist, data.pos);
                }
            },
        }
    }
    data.acc.to_string()
}
