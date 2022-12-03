mod utils;

pub mod day1;
pub mod day2;

macro_rules! runner {
    ($($module:tt),*) => (
        let mut i = 0;
        $(
            i += 1;
            println!("Day {}, Part 1: {}", i, $module::part_1(format!("input_files/day{}.txt", i).as_str()));
            println!("Day {}, Part 2: {}", i, $module::part_2(format!("input_files/day{}.txt", i).as_str()));
        )*
    )
}

fn main() {
    runner!(day1, day2);
}
