use std::fs::read_to_string;

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let data: Vec<(u128, Vec<u128>)> = input
        .lines()
        .map(|x| {
            (
                x.split(':').collect::<Vec<_>>()[0].parse().unwrap(),
                x.split(':').collect::<Vec<_>>()[1]
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let mut sum = 0;
    for i in data {
        if evaluate(i.0, i.1) {
            sum += i.0;
        }
    }

    println!("{sum}");
}

fn evaluate(result: u128, numbers: Vec<u128>) -> bool {
    let num_operations = numbers.len() as u32 - 1;

    for i in 0..2u128.pow(num_operations) {
        let mut nums = numbers.iter();
        let mut evaluation = *nums.next().unwrap();

        for j in 0..nums.len() {
            if i & 2u128.pow(j as u32) == 2u128.pow(j as u32) {
                evaluation *= nums.next().unwrap();
            } else {
                evaluation += nums.next().unwrap();
            }
        }

        if evaluation == result {
            return true;
        }
    }
    false
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let data: Vec<(u128, Vec<u128>)> = input
        .lines()
        .map(|x| {
            (
                x.split(':').collect::<Vec<_>>()[0].parse().unwrap(),
                x.split(':').collect::<Vec<_>>()[1]
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let mut sum = 0;
    for i in data {
        if evaluate2(i.0, i.1) {
            sum += i.0;
        }
    }

    println!("{sum}");
}

fn evaluate2(result: u128, numbers: Vec<u128>) -> bool {
    let num_operations = numbers.len() as u32 - 1;

    let mut operations = (0..num_operations).map(|_| 0).collect::<Vec<u32>>();
    for _ in 0..3u128.pow(num_operations) {
        let mut nums = numbers.iter();

        let mut evaluation = *nums.next().unwrap();
        (0..nums.len()).for_each(|j| match operations[j] {
            0 => evaluation += nums.next().unwrap(),
            1 => evaluation *= nums.next().unwrap(),
            2 => {
                evaluation = format!("{}{}", evaluation, nums.next().unwrap())
                    .parse()
                    .unwrap()
            }
            _ => (),
        });

        if evaluation == result {
            return true;
        }

        operations[0] += 1;
        for i in 0..(operations.len() - 1) {
            if operations[i] >= 3 {
                operations[i] = 0;
                operations[i + 1] += 1;
            }
        }
    }
    false
}
