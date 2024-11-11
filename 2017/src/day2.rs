use std::fs::read_to_string;

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let mut nums = line
            .split(char::is_whitespace)
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        nums.sort();

        sum += nums.last().unwrap() - nums.first().unwrap();
    }

    println!("{sum}");
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let mut nums = line
            .split(char::is_whitespace)
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        nums.sort();
        let mut nums = nums.iter();

        loop {
            if nums.clone().count() == 0 {
                break;
            }

            let dividend = nums.next_back().unwrap();
            for divisor in nums.clone() {
                if dividend % divisor == 0 {
                    sum += dividend / divisor;
                }
            }
        }
    }

    println!("{sum}");
}
