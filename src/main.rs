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
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod tools;

fn main() {
    let opt = Opt::from_args();
    let day = tools::get_data(opt.day);

    let file_name = tools::get_day_relative_path(day);
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
        8 => day08::run(reader),
        9 => day09::run(reader),
        10 => day10::run(reader),
        11 => day11::run(reader),
        12 => day12::run(reader),
        13 => day13::run(reader),
        14 => day14::run(reader),
        15 => day15::run(reader),
        16 => day16::run(reader),
        17 => day17::run(reader),
        18 => day18::run(reader),
        20 => day20::run(reader),
        21 => day21::run(reader),
        22 => day22::run(reader),
        23 => day23::run(reader),
        24 => day24::run(reader),
        _ => panic!("This day is yet to come."),
    };
    println!("Anwser for day {} part 1: {}", day, part1);
    println!("Anwser for day {} part 2: {}", day, part2);
}
