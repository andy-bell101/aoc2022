#[macro_use]
extern crate lazy_static;
extern crate load_file;

mod utils;

use clap::Parser;
use std::collections::HashMap;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which day to run. Default is to run all days.
    #[arg(short, long)]
    day: Option<u8>,
    /// Which part to run. Default is to run both parts.
    #[arg(short, long)]
    part: Option<u8>,
}

macro_rules! runner {
    ($($module:tt,)*) => (
        {
            let mut m = HashMap::new();
            let mut i: u8 = 0;
            $(
                i += 1;
                let file_contents = load_file::load_str!(format!("../input_files/day{}.txt", i).as_str());
                m.insert(i, HashMap::from([(1u8, ($module::part_1 as fn(&str) -> String, file_contents)), (2u8, ($module::part_2 as fn(&str) -> String, file_contents))]));
            )*
            m
        }
    )
}

fn main() {
    let func_map: HashMap<u8, HashMap<u8, (fn(&str) -> String, &str)>> = runner!(
        day1, day2, day3, day4, day5, day6,
        day7, day8,
        // day9,
        // day10,
        // day11,
        // day12,
        // day13,
        // day14,
        // day15,
        // day16,
        // day17,
        // day18,
        // day19,
        // day20,
        // day21,
        // day22,
        // day23,
        // day24,
        // day25,
    );
    let args = Args::parse();

    let days = match args.day {
        Some(x) => {
            if !func_map.contains_key(&x) {
                panic!("Invalid day input. Solution may not be implemented yet");
            }
            vec![x]
        }
        None => (1..=func_map.len() as u8).collect(),
    };
    let parts = match args.part {
        Some(x) => {
            if x == 1 || x == 2 {
                vec![x]
            } else {
                panic!("Invalid part input");
            }
        }
        None => vec![1, 2],
    };

    for i in days.iter() {
        for j in parts.iter() {
            let (func, contents) = func_map[i][j];
            println!("Day {} part {} solution: {}", i, j, func(contents));
        }
    }
}
