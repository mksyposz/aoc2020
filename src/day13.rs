pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut e_time: i64 = 0;
    let mut bus = String::new();
    input.lines().enumerate().for_each(|(i, val)| {
        match i {
            0 => e_time = val.unwrap().parse::<i64>().unwrap(),
            1 => bus = val.unwrap().clone(),
            _ => unreachable!(),
        }
    });
    let bus_times: Vec<i64> = bus.split(',')
                                 .filter(|&x| x != "x")
                                 .map(|x| x.parse::<i64>().unwrap()).collect();
    let ans1 = part_one(e_time, &bus_times);
    let bus_t: Vec<(i64, i64)> = bus.split(',')
                                    .enumerate()
                                    .filter(|&(_, val)| val != "x")
                                    .map(|(i, val)| (i as i64, val.parse::<i64>().unwrap()))
                                    .collect();
    let ans2 = part_two(&bus_t);
    (ans1, ans2)
}

fn part_one(time: i64, bus_times: &Vec<i64>) -> String {
    let mut best_time = time;
    let mut result = 0;
    bus_times.iter().for_each(|x| {
        if time % x == 0 {
            best_time = 0;
            result = 0;
        } else {
            let k = (time / x) + 1;
            if best_time > x*k - time {
                best_time = x*k - time;
                result = best_time * x;
            }
        }
    });
    result.to_string()
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> i64 {
    let (_, x, _) = egcd(x, n);
    (x % n + n) % n
}

fn part_two(times: &Vec<(i64, i64)>) -> String {
    let mut prod: i64 = 1;
    times.iter().for_each(|(_, t)| prod *= t);
    let result = times.iter().map(|(a, t)| {
        (prod/t) * mod_inv(prod/t, *t) * (t - (a % t))
    })
    .fold(0, |sum, val| (sum+val) % prod );
    result.to_string()
}
