use std::fs;

pub fn part_one() {
    let input = fs::read_to_string("./src/input").unwrap();

    let mut sum_jolts = 0;
    for line in input.lines() {
        let jolts = line
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        let mut max1 = *jolts.iter().max().unwrap();
        let max1_pos = jolts.iter().position(|&x| x == max1).unwrap();

        let max2;
        if max1_pos == jolts.len() - 1 {
            max1 = *jolts[..jolts.len() - 1].iter().max().unwrap();
            let max1_pos = jolts.iter().position(|&x| x == max1).unwrap();

            max2 = *jolts[(max1_pos + 1)..].iter().max().unwrap();
        } else {
            max2 = *jolts[(max1_pos + 1)..].iter().max().unwrap();
        }

        let max_jolts = max1 * 10 + max2;

        println!("{max_jolts}");

        sum_jolts += max_jolts;
    }

    println!("\n{sum_jolts}");
}

fn get_max_combination(jolts: &Vec<u64>, i: usize, start_index: usize) -> Option<u64> {
    // if start_index == jolts.len() - 1 {
    //     return None;
    // }
    // let mut combinations = vec![];
    let mut num = None;
    // println!("{i}:");
    for n in (0..=9).rev() {
        // print!("\t{n}: ");
        let mut done = true;

        // let mut instances = vec![];
        // jolts[start_index..jolts.len() - i + 1]
        //     .iter()
        //     .enumerate()
        //     .for_each(|x| {
        //         if *x.1 as usize == n {
        //             instances.push(x.0 + start_index);
        //             // print!("{} ", x.0 + start_index);
        //         }
        //     });
        let instances = (start_index..(jolts.len() - i + 1))
            .filter(|&idx| jolts[idx] == n)
            .collect::<Vec<usize>>();

        if instances.is_empty() {
            // println!("none");
            done = false;
        } else {
            // println!();
            for instance in instances {
                num = Some(jolts[instance]);

                if i - 1 == 0 {
                    // combinations.push(num);
                    return num;
                } else {
                    let max_combinations = get_max_combination(jolts, i - 1, instance + 1);
                    if max_combinations.is_none() {
                        done = false;
                        break;
                    }
                    num = Some(num.unwrap() * 10u64.pow(i as u32 - 1) + max_combinations.unwrap());
                    // combinations.push(num);
                }
                break;
            }
        }
        if done {
            break;
        }
    }

    // if i == 12 {
    //     println!("{start_index} {i}:");
    //     for i in combinations.clone() {
    //         println!("\t{i}");
    //     }
    // }

    // if combinations.is_empty() {
    //     return None;
    // }
    num
}

pub fn part_two() {
    let input = fs::read_to_string("./src/input").unwrap();

    let mut sum_jolts = 0;
    for line in input.lines() {
        let jolts = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();

        let max_jolts = get_max_combination(&jolts, 12, 0).unwrap();
        println!("{max_jolts}");
        // for i in 0..=9 {
        //     let mut instances = vec![];
        //     jolts.iter().enumerate().for_each(|x| {
        //         if *x.1 == i {
        //             instances.push(x.0);
        //         }
        //     });

        //     for index in instances {
        //         if index == jolts.len() - 1 {
        //             continue;
        //         }
        //     }
        // }

        sum_jolts += max_jolts;
    }

    println!("\n{sum_jolts}");
}
