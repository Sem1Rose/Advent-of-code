use regex::Regex;
use std::fs::read_to_string;

struct ClawMachine {
    a_pos_inc: (i32, i32),
    b_pos_inc: (i32, i32),
    prize_pos: (i32, i32),
}

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let mut machines = vec![];

    let split_input = input.split("\n\n").collect::<Vec<_>>();
    for split in split_input {
        let re = Regex::new(r"([0-9]+)\b").unwrap();
        let mut captures = vec![];
        for (_, [num]) in re.captures_iter(split).map(|c| c.extract()) {
            captures.push(num.parse().unwrap());
        }

        machines.push(ClawMachine {
            a_pos_inc: (captures[0], captures[1]),
            b_pos_inc: (captures[2], captures[3]),
            prize_pos: (captures[4], captures[5]),
        });
    }

    let mut possible_values_per_machine = vec![];
    for (i, machine) in machines.iter().enumerate() {
        // println!("{i}");
        possible_values_per_machine.push(vec![]);

        let det = |a: (i32, i32), b: (i32, i32)| -> i32 { (a.0 * b.1 - b.0 * a.1) };
        // let enfloat = |a: (i32, i32)| -> (f64, f64) { (a.0 as f64, a.1 as f64) };
        let in_range = |v: i32, min: i32, max: i32| -> bool { v >= min && v <= max };

        if det(machine.a_pos_inc, machine.b_pos_inc) == 0 {
            if (machine.prize_pos.0 as f64 / machine.prize_pos.1 as f64)
                == (machine.a_pos_inc.0 as f64 / machine.b_pos_inc.0 as f64)
            {
                let fx = |x: i32| -> f64 {
                    (-x * (machine.a_pos_inc.1 + machine.a_pos_inc.0)
                        + (machine.prize_pos.0 + machine.prize_pos.1)) as f64
                        / (machine.b_pos_inc.0 + machine.b_pos_inc.1) as f64
                };

                for x in 0..=100i32 {
                    let y = fx(x);

                    if y.floor() == y && in_range(y as i32, 0, 100) {
                        possible_values_per_machine[i].push((x, y as i32));
                    }
                }
            }
        } else {
            let x = det(machine.prize_pos, machine.b_pos_inc) as f64
                / det(machine.a_pos_inc, machine.b_pos_inc) as f64;
            let y = det(machine.a_pos_inc, machine.prize_pos) as f64
                / det(machine.a_pos_inc, machine.b_pos_inc) as f64;

            if x.floor() == x
                && y.floor() == y
                && in_range(x as i32, 0, 100)
                && in_range(y as i32, 0, 100)
            {
                possible_values_per_machine[i].push((x as i32, y as i32));
            }
        }
    }

    // let mut possible_values_per_machine = vec![];
    // for (i, machine) in machines.iter().enumerate() {
    //     println!("{i}");
    //     possible_values_per_machine.push(vec![]);
    //     let fx = |x: f64, a: (f64, f64), b: (f64, f64), p: (f64, f64)| -> f64 {
    //         (-x * (a.0 + a.1) + (p.0 + p.1)) / (b.0 + b.1)
    //     };
    //     let enfloat = |a: (i32, i32)| -> (f64, f64) { (a.0 as f64, a.1 as f64) };
    //     let in_range = |v: i32, min: i32, max: i32| -> bool { v >= min && v <= max };

    //     for x in 0..=100u32 {
    //         let y = fx(
    //             x as f64,
    //             enfloat(machine.a_pos_inc),
    //             enfloat(machine.b_pos_inc),
    //             enfloat(machine.prize_pos),
    //         );

    //         if y.floor() == y && in_range(y as i32, 0, 100) {
    //             println!("{}, ({}, {})", y, x, y as i32);
    //             possible_values_per_machine[i].push((x, y as u32));
    //         }
    //     }
    //     print!("---");
    // }
    // println!("\n");

    let mut min_sum = 0;
    possible_values_per_machine = possible_values_per_machine
        .iter()
        .filter(|x| !x.is_empty())
        .cloned()
        .collect();
    for machine in possible_values_per_machine {
        let mut min_price = i32::MAX;
        for value in machine {
            let y = 3 * value.0 + value.1;
            if y < min_price {
                min_price = y;
                println!("{}, ({}, {})", min_price, value.0, value.1);
            }
        }

        min_sum += min_price;
        println!("------\n");
    }

    println!("{min_sum}");
}

#[derive(Clone, Copy)]
struct ClawMachine2 {
    a_pos_inc: (i64, i64),
    b_pos_inc: (i64, i64),
    prize_pos: (i64, i64),
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let mut machines = vec![];

    let split_input = input.split("\n\n").collect::<Vec<_>>();
    for split in split_input {
        let re = Regex::new(r"([0-9]+)\b").unwrap();
        let mut captures = vec![];
        for (_, [num]) in re.captures_iter(split).map(|c| c.extract()) {
            captures.push(num.parse().unwrap());
        }

        machines.push(ClawMachine2 {
            a_pos_inc: (captures[0], captures[1]),
            b_pos_inc: (captures[2], captures[3]),
            prize_pos: (captures[4] + 10e12f64 as i64, captures[5] + 10e12f64 as i64),
        });
    }

    // for machine in machines.clone() {
    //     println!(
    //         "({}, {}), ({}, {}), ({}, {})",
    //         machine.a_pos_inc.0,
    //         machine.a_pos_inc.1,
    //         machine.b_pos_inc.0,
    //         machine.b_pos_inc.1,
    //         machine.prize_pos.0,
    //         machine.prize_pos.1
    //     );
    // }

    let mut possible_values_per_machine = vec![];
    for (i, machine) in machines.iter().enumerate() {
        // println!("{i}");
        possible_values_per_machine.push(vec![]);

        let det = |a: (i64, i64), b: (i64, i64)| -> i64 { a.0 * b.1 - b.0 * a.1 };
        // let enfloat = |a: (i64, i64)| -> (f64, f64) { (a.0 as f64, a.1 as f64) };
        // let in_range = |v: i64, min: i64, max: i64| -> bool { v >= min && v <= max };

        if det(machine.a_pos_inc, machine.b_pos_inc) == 0 {
            if (machine.prize_pos.0 as f64 / machine.prize_pos.1 as f64)
                == (machine.a_pos_inc.0 as f64 / machine.b_pos_inc.0 as f64)
            {
                let fx = |x: i64| -> f64 {
                    (-x * (machine.a_pos_inc.1 + machine.a_pos_inc.0)
                        + (machine.prize_pos.0 + machine.prize_pos.1)) as f64
                        / (machine.b_pos_inc.0 + machine.b_pos_inc.1) as f64
                };

                let mut x = 0;
                loop {
                    let y = fx(x);

                    if y < 0.0 {
                        break;
                    }

                    if y.floor() == y {
                        possible_values_per_machine[i].push((x, y as i64));
                    }
                    x += 1;
                }
            }
        } else {
            let x = det(machine.prize_pos, machine.b_pos_inc) as f64
                / det(machine.a_pos_inc, machine.b_pos_inc) as f64;
            let y = det(machine.a_pos_inc, machine.prize_pos) as f64
                / det(machine.a_pos_inc, machine.b_pos_inc) as f64;

            if x.floor() == x && y.floor() == y && x >= 0.0 && y >= 0.0 {
                possible_values_per_machine[i].push((x as i64, y as i64));
            }
        }
    }

    let mut min_sum = 0;
    possible_values_per_machine = possible_values_per_machine
        .iter()
        .filter(|x| !x.is_empty())
        .cloned()
        .collect();
    for machine in possible_values_per_machine {
        let mut min_price = i64::MAX;
        for value in machine {
            let y = 3 * value.0 + value.1;
            if y < min_price {
                min_price = y;
                // println!("{}, ({}, {})", min_price, value.0, value.1);
            }
        }

        min_sum += min_price;
        // println!("------\n");
    }

    println!("{min_sum}");
}
