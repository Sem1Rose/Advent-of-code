use std::fs::read_to_string;

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let mut sum: u32 = 0;
    let mut chars = input.chars();
    let mut current_char = chars.next().unwrap();

    loop {
        if chars.clone().count() == 0 {
            break;
        }

        loop {
            let next = chars.next().unwrap();
            if next == current_char {
                sum += current_char.to_string().parse::<u32>().unwrap();
            } else {
                current_char = next;
                break;
            }

            if chars.clone().count() == 0 {
                break;
            }
        }
    }

    if input.chars().next().unwrap() == current_char {
        sum += current_char.to_string().parse::<u32>().unwrap();
    }

    println!("{sum}");
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let mut sum: u32 = 0;
    let halfway_around = input.chars().count() as u32 / 2;

    for (i, c) in input.chars().enumerate() {
        let mut chars = input.chars().cycle();
        for _ in 0..(halfway_around + i as u32) {
            chars.next();
        }

        let next = chars.next().unwrap();
        if next == c {
            sum += c.to_string().parse::<u32>().unwrap();
        }
    }

    println!("{sum}");
}
