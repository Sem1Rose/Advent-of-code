use std::fs;

pub fn part_one() {
    let input = fs::read_to_string("./src/input")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.to_string())
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
        })
        .rev()
        .collect::<Vec<_>>();

    // println!("{} {}", input.len(), input[0].len());
    // for i in input {
    //     for j in i {
    //         print!("{j} ");
    //     }
    //     println!();
    // }

    let mut grand_total = 0;
    for i in 0..input[0].len() {
        let op = &input[0][i];
        let mut total = if op == "+" { 0 } else { 1 };
        for j in 1..input.len() {
            if op == "+" {
                total += input[j][i].parse::<u64>().unwrap();
            } else {
                total *= input[j][i].parse::<u64>().unwrap();
            }
        }
        grand_total += total;
    }

    println!("{grand_total}");
}

pub fn part_two() {
    let input = fs::read_to_string("./src/input")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .rev()
        .collect::<Vec<_>>();

    let mut grand_total: u64 = 0;
    let mut op = ' ';
    let mut total = 0;
    for i in 0..input[0].len() {
        let x = &input[0][i];
        if *x != ' ' {
            // println!("{total}");
            grand_total += total;
            op = *x;
            // println!("{op}");
            total = if op == '+' { 0 } else { 1 };
        }

        let mut num = 0;
        let mut pow = 0;
        for j in 1..input.len() {
            let x = &input[j][i];
            if *x == ' ' {
                continue;
            }
            num += x.to_digit(10).unwrap() as u64 * 10u64.pow(pow);
            pow += 1;
        }

        if num == 0 {
            continue;
        }

        if op == '+' {
            total += num;
        } else {
            total *= num;
        }

        // println!("{num}");
    }
    // println!("{total}");
    grand_total += total;

    println!("{grand_total}");
}
