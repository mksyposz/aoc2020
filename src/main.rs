use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "d",long)]
    day: Option<u32>,
}

use std::fs::File;
use std::io::BufReader;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod tools;

fn get_file_name(day: u32) -> String {
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
    let day = tools::get_data(opt.day);

    let file_name = get_file_name(day);
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let (part1, part2) = match day {
        1 => day01::run(reader),
        2 => day02::run(reader),
        3 => day03::run(reader),
        4 => day04::run(reader),
        5 => day05::run(reader),
        6 => day06::run(reader),
        7 => day07::run(reader),
        _ => panic!("This day is yet to come."),
    };
    println!("Anwser for day {} part 1: {}", day, part1);
    println!("Anwser for day {} part 2: {}", day, part2);
}
