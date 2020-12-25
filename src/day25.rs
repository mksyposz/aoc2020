const SUBJECT_NUMBER: u64 = 7;
const DIVIDER: u64 = 20201227;


pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut input_iter = input.lines();
    let card_public_key = input_iter.next().unwrap().unwrap().parse::<u64>().unwrap();
    let door_public_key = input_iter.next().unwrap().unwrap().parse::<u64>().unwrap();
    let loop_size = find_loop_size(card_public_key);
    let ans1 = part_one(loop_size, door_public_key);
    let ans2 = part_two();
    (ans1, ans2)
}

fn find_loop_size(key: u64) -> u64 {
    let mut loop_size = 0;
    let mut current_value = 1;
    while current_value != key {
        current_value *= SUBJECT_NUMBER;
        current_value %= DIVIDER;
        loop_size += 1;
    }
     loop_size
}

fn part_one(repeat: u64, public_key: u64) -> String {
    (0..repeat).fold(1, |mut acc, _| {acc *= public_key; acc %= DIVIDER; acc})
               .to_string()
}

fn part_two() -> String {
    String::from("There's none :(")
}
