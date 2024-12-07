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
        // println!(
        //     "{}: {}",
        //     i.0,
        //     i.1.iter()
        //         .map(|x| x.to_string())
        //         .collect::<Vec<_>>()
        //         .join(" ")
        // );
        if evaluate(i.0, i.1) {
            // println!("{}", i.0);
            sum += i.0;
        }
    }

    println!("{sum}");
}

fn evaluate(result: u128, numbers: Vec<u128>) -> bool {
    let num_operations = numbers.len() as u32 - 1;

    // println!("{} {}\n--\n", num_operations, 2u128.pow(num_operations));
    for i in 0..2u128.pow(num_operations) {
        let mut nums = numbers.iter();
        let mut evaluation = *nums.next().unwrap();

        // let mut debug = evaluation.to_string();
        for j in 0..nums.len() {
            // println!(
            //     "{} {} {} {}",
            //     i,
            //     j,
            //     2u128.pow(j as u128),
            //     i & 2u128.pow(j as u128)
            // );
            if i & 2u128.pow(j as u32) == 2u128.pow(j as u32) {
                // let x = nums.next().unwrap();
                // debug = format!("{} * {}", debug, x);
                // evaluation *= x;
                evaluation *= nums.next().unwrap();
            } else {
                // let x = nums.next().unwrap();
                // debug = format!("{} + {}", debug, x);
                // evaluation += x;
                evaluation += nums.next().unwrap();
            }
        }

        // println!("{} = {}", debug, evaluation);

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
        // println!(
        //     "{}: {}",
        //     i.0,
        //     i.1.iter()
        //         .map(|x| x.to_string())
        //         .collect::<Vec<_>>()
        //         .join(" ")
        // );
        if evaluate2(i.0, i.1) {
            // println!("{}", i.0);
            sum += i.0;
        }
    }

    println!("{sum}");
}

fn evaluate2(result: u128, numbers: Vec<u128>) -> bool {
    let num_operations = numbers.len() as u32 - 1;

    // println!("{} {}\n--\n", num_operations, 2u128.pow(num_operations));
    let mut operations = (0..num_operations).map(|_| 0).collect::<Vec<u32>>();
    for _ in 0..3u128.pow(num_operations) {
        // (0..operations.len()).for_each({
        //     |j| {
        //         if operations[j] == 2 {
        //             let new_num =
        //                 format!("{}{}", modified_nums.last().unwrap(), nums.next().unwrap())
        //                     .parse()
        //                     .unwrap();
        //             modified_nums.remove(modified_nums.len() - 1);
        //             modified_nums.push(new_num);
        //             // if j < operations.len() - 1 {
        //             //     if operations[j + 1] != 2 {
        //             //         modified_operations.push(operations[j + 1]);
        //             //     }
        //             // j += 1;
        //             // }
        //         } else {
        //             modified_operations.push(operations[j]);
        //             modified_nums.push(*nums.next().unwrap());
        //         }
        //     }
        // });
        // modified_nums.extend(nums);
        let mut nums = numbers.iter();

        let mut evaluation = *nums.next().unwrap();
        // let mut debug = evaluation.to_string();
        (0..nums.len()).for_each(|j| {
            match operations[j] {
                0 => evaluation += nums.next().unwrap(),
                1 => evaluation *= nums.next().unwrap(),
                2 => {
                    evaluation = format!("{}{}", evaluation, nums.next().unwrap())
                        .parse()
                        .unwrap()
                }
                _ => (),
            }
            // if modified_operations[j] % 3 == 0 {
            //     let x = modified_nums.next().unwrap();
            //     debug = format!("{} + {}", debug, x);
            //     evaluation += x;
            //     // evaluation += modified_nums.next().unwrap();
            // } else if modified_operations[j] % 3 == 1 {
            //     let x = modified_nums.next().unwrap();
            //     debug = format!("{} * {}", debug, x);
            //     evaluation *= x;
            //     // evaluation *= modified_nums.next().unwrap();
            // }
        });

        // println!("{} = {}", debug, evaluation);

        if evaluation == result {
            return true;
        }

        // println!(
        //     "{}",
        //     operations
        //         .iter()
        //         .map(|x| x.to_string())
        //         .collect::<Vec<String>>()
        //         .join(" ")
        // );

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
