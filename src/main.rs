use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "d",long)]
    day: usize,
}

use std::fs::File;
use std::io::BufReader;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn get_file_name(opt: &Opt) -> String {
    let day = opt.day;
    let pref = if day < 10 {
        "0"
    } else {
        ""
    };
    let day = pref.to_owned() + &day.to_string();
    return String::from("data/day") + &day + ".txt";
}

fn main() {
    let opt = Opt::from_args();

    let file_name = get_file_name(&opt);
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let (part1, part2) = match opt.day {
        1 => day01::run(reader),
        2 => day02::run(reader),
        3 => day03::run(reader),
        4 => day04::run(reader),
        5 => day05::run(reader),
        6 => day06::run(reader),
        _ => panic!("This day is yet to come."),
    };
    println!("Anwser for day {} part 1: {}", opt.day, part1);
    println!("Anwser for day {} part 2: {}", opt.day, part2);
}
