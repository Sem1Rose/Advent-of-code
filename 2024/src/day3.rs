use regex::Regex;
use std::fs::read_to_string;

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let re = Regex::new(r"mul\(([0-9]{1,3})\,([0-9]{1,3})\)").unwrap();
    let mut numbers: Vec<(u32, u32)> = vec![];
    let mut sum = 0;

    for (_, [first_num, second_num]) in re.captures_iter(&input).map(|c| c.extract()) {
        numbers.push((first_num.parse().unwrap(), second_num.parse().unwrap()));
    }

    for i in numbers {
        sum += i.0 * i.1;
    }

    println!("{sum}")
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let re = Regex::new(r"(mul\([0-9]{1,3}\,[0-9]{1,3}\))|(do\(\))|(don\'t\(\))").unwrap();
    let mut numbers: Vec<&str> = vec![];
    let mut sum = 0;

    for (_, [mat]) in re.captures_iter(&input).map(|c| c.extract()) {
        numbers.push(mat);
    }

    let multiply_match = Regex::new(r"mul\(([0-9]{1,3})\,([0-9]{1,3})\)").unwrap();
    let do_match = Regex::new(r"do\(\)").unwrap();
    let dont_match = Regex::new(r"don\'t\(\)").unwrap();
    let mut dont_multiply = false;
    for i in numbers {
        if do_match.is_match(i) {
            dont_multiply = false;
            continue;
        }
        if dont_multiply {
            continue;
        }
        if dont_match.is_match(i) {
            dont_multiply = true;
            continue;
        }

        let (_, [first, second]) = multiply_match.captures(i).unwrap().extract();

        sum += first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap();
    }

    println!("{sum}")
}
