use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let now = Instant::now();

    day7::part_one();
    day7::part_two();

    let elapsed = now.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}
