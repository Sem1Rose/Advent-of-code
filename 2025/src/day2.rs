use std::fs;

// damn
// fn generate_ids(
//     num: usize,
//     id: usize,
//     num_digits: usize,
//     min_range: usize,
//     max_range: usize,
//     possible_digits: &Vec<Vec<i32>>,
// ) -> u64 {
//     let mut sum_invalid_ids = 0;

//     for i in 0..possible_digits[id].len() {
//         let mut new_num = num;
//         new_num = new_num * 10 + possible_digits[id][i] as usize;
//         if id + 1 < num_digits / 2 {
//             sum_invalid_ids += generate_ids(
//                 new_num,
//                 id + 1,
//                 num_digits,
//                 min_range,
//                 max_range,
//                 possible_digits,
//             );
//         } else {
//             let id = new_num * 10usize.pow(num_digits as u32 / 2) + new_num;
//             if id >= min_range && id <= max_range {
//                 print!("{id} ");
//                 sum_invalid_ids += id as u64;
//             }
//         }
//     }

//     sum_invalid_ids
// }
fn generate_ids(
    num: usize,
    id: usize,
    num_digits: usize,
    min_range: usize,
    max_range: usize,
) -> u64 {
    let mut sum_invalid_ids = 0;

    for i in 0..=9 {
        if id == 0 && i == 0 {
            continue;
        }
        let mut new_num = num;
        new_num = new_num * 10 + i;
        if id + 1 < num_digits / 2 {
            sum_invalid_ids += generate_ids(new_num, id + 1, num_digits, min_range, max_range);
        } else {
            let id = new_num * 10usize.pow(num_digits as u32 / 2) + new_num;
            if id >= min_range && id <= max_range {
                // print!("{id} ");
                sum_invalid_ids += id as u64;
            }
        }
    }

    sum_invalid_ids
}

pub fn part_one() {
    let input = fs::read_to_string("./src/input").unwrap();

    let ranges: Vec<[String; 2]> = input
        .split(',')
        .map(|x| {
            <[String; 2]>::try_from(x.split('-').map(|x| x.to_owned()).collect::<Vec<String>>())
                .unwrap()
        })
        .collect();

    let mut sum_invalid_ids = 0;
    for range in ranges {
        // print!("{}-{}: ", range[0], range[1]);
        let range_l_num_of_digits = range[0].len();
        let range_h_num_of_digits = range[1].len();

        let mut num_digits = range_l_num_of_digits;
        if num_digits & 1 == 1 {
            num_digits += 1;
        }
        while num_digits <= range_h_num_of_digits {
            // damn this is all useless
            // let mut possible_digits: Vec<Vec<i32>> = vec![];
            // for pos in 0..num_digits / 2 {
            //     possible_digits.push(vec![]);
            //     // XX00
            //     // let place_position = num_digits - pos - 1;
            //     for n in 0..=9 {
            //         if pos == 0 && n == 0 {
            //             continue;
            //         }
            //         // HAHA FUCK YOU PAST ME I GOT IT WORKING
            //         // if num_digits == range_h_num_of_digits {
            //         //     if n > range[1].get(pos..=pos).unwrap().parse().unwrap() {
            //         //         continue;
            //         //     }
            //         // }
            //         // if num_digits == range_l_num_of_digits {
            //         //     if n < range[0].get(pos..=pos).unwrap().parse().unwrap() {
            //         //         continue;
            //         //     }
            //         // }

            //         possible_digits[pos].push(n);
            //     }
            // }

            // for i in 0..num_digits / 2 {
            //     print!("{i}: ");
            //     for j in 0..possible_digits[i].len() {
            //         print!("{} ", possible_digits[i][j]);
            //     }
            //     println!();
            // }
            sum_invalid_ids += generate_ids(
                0,
                0,
                num_digits,
                range[0].parse().unwrap(),
                range[1].parse().unwrap(),
                // &possible_digits,
            );

            num_digits += 2;
        }

        // panic!("ass");
        // println!();
    }
    println!("{sum_invalid_ids}");
}

fn generate_ids2(
    num: usize,
    id: usize,
    num_digits: usize,
    partition: usize,
    min_range: usize,
    max_range: usize,
    found_invalid_ids: &mut Vec<usize>,
) -> u64 {
    let mut sum_invalid_ids = 0;

    for i in 0..=9 {
        if id == 0 && i == 0 {
            continue;
        }
        let mut new_num = num;
        new_num = new_num * 10 + i;
        if id + 1 < partition {
            sum_invalid_ids += generate_ids2(
                new_num,
                id + 1,
                num_digits,
                partition,
                min_range,
                max_range,
                found_invalid_ids,
            );
        } else {
            let mut id = new_num;
            for _ in 0..(num_digits / partition - 1) {
                id = id * 10usize.pow(partition as u32) + new_num;
            }
            if !found_invalid_ids.contains(&id) && id >= min_range && id <= max_range {
                // print!("{id} ");
                sum_invalid_ids += id as u64;
                found_invalid_ids.push(id);
            }
        }
    }

    sum_invalid_ids
}

pub fn part_two() {
    let input = fs::read_to_string("./src/input").unwrap();

    let ranges: Vec<[String; 2]> = input
        .split(',')
        .map(|x| {
            <[String; 2]>::try_from(x.split('-').map(|x| x.to_owned()).collect::<Vec<String>>())
                .unwrap()
        })
        .collect();

    let mut sum_invalid_ids = 0;
    for range in ranges {
        // println!("{}-{}:", range[0], range[1]);
        let mut found_invalid_ids = vec![];
        let range_l_num_of_digits = range[0].len();
        let range_h_num_of_digits = range[1].len();

        for num_digits in range_l_num_of_digits..=range_h_num_of_digits {
            // println!("  {num_digits}:");

            for partition in 1..=num_digits / 2 {
                // print!("    {}: ", partition);
                if num_digits % partition != 0 {
                    // println!();
                    continue;
                }

                sum_invalid_ids += generate_ids2(
                    0,
                    0,
                    num_digits,
                    partition,
                    range[0].parse().unwrap(),
                    range[1].parse().unwrap(),
                    &mut found_invalid_ids,
                );

                // println!();
            }
        }

        // println!();
    }
    println!("{sum_invalid_ids}");
}
