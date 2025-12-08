use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let now = Instant::now();

    day8::part_one();
    day8::part_two();

    let elapsed = now.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}
