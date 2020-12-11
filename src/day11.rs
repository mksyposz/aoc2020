pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let seats = input.lines()
                     .map(|x| x.unwrap())
                     .map(|x| x.chars().collect::<Vec<_>>())
                     .collect::<Vec<_>>();
    let ans1 = part_one(&seats);
    let ans2 = part_two(&seats);
    (ans1, ans2)
}

fn part_one(seats: &Vec<Vec<char>>) -> String {
    let (changed, seats) = simulate(seats, should_swap);
    if !changed {
        return seats.iter().flat_map(|row| row.iter()).filter(|&&c| c == '#').count().to_string()
    }
    return part_one(&seats);
}

fn part_two(seats: &Vec<Vec<char>>) -> String {
    let (changed, seats) = simulate(seats,should_swap_pt2);
    if !changed {
        return seats.iter().flat_map(|row| row.iter()).filter(|&&c| c == '#').count().to_string()
    }
    return part_two(&seats);
}

fn simulate(seats: &Vec<Vec<char>>,
    should_swap: fn(&Vec<Vec<char>>, usize, usize) -> bool) -> (bool, Vec<Vec<char>>) {
    let mut changed = false;
    let new_seats = (0..seats.len()).map(|y| {
        (0..seats[y].len()).map(|x| {
            if seats[y][x] == '.' {return '.';}
            let c = match (seats[y][x], should_swap(seats, x, y)) {
                ('L', true) => {changed = true; '#'},
                ('#', true) => {changed = true; 'L'},
                (c, _) => c,
            };
            c
        })
        .collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();
    (changed, new_seats)
}

static DIRS: [(i64, i64); 8] = [(-1, -1), (-1, 0), (-1, 1),
                                (0, -1) ,          (0, 1),
                                (1, -1) , (1, 0) , (1, 1)];

fn get_close_neighbours(seats: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)>{
    DIRS.iter().filter_map(|(dy, dx)| {
        let y = (y as i64) + dy;
        let x = (x as i64) + dx;
        match seats.get(y as usize).and_then(|row| row.get(x as usize)) {
            Some('#') | Some('L') => {Some((y as usize, x as usize))},
            _ => {None},
        }
    }).collect::<Vec<(usize, usize)>>()
}

fn get_nonempty_seats(seats: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    DIRS.iter().filter_map(|(dy, dx)| {
        let mut y = y as i64;  let mut x = x as i64;
        let res: Option<(usize, usize)>;
        loop {
            y += dy;
            x += dx;
            res = match seats.get(y as usize).and_then(|row| row.get(x as usize)) {
                Some('.') => continue,
                Some('#') | Some('L') => {Some((y as usize, x as usize))},
                _ => None
            };
            break;
        }
        res
    }).collect()
}

fn should_swap(seats: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let neighbours = get_close_neighbours(seats, x, y); //this should be cashed
    match seats[y][x] {
        'L' => neighbours.iter().all(|&(i,j)| seats[i][j] != '#'),
        '#' => neighbours.iter().filter(|&&(i,j)| seats[i][j] == '#').count() >= 4,
        _ => unreachable!(),
    }
}

fn should_swap_pt2(seats: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let neighbours = get_nonempty_seats(seats, x, y); //this should be cashed
    match seats[y][x] {
        'L' => neighbours.iter().all(|&(i,j)| seats[i][j] != '#'),
        '#' => neighbours.iter().filter(|&&(i,j)| seats[i][j] == '#').count() >= 5,
        _ => unreachable!(),
    }
}
