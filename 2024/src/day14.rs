use regex::Regex;
use std::fs::read_to_string;

#[derive(Clone, Copy)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let mut robots = vec![];
    for i in input.lines() {
        let re = Regex::new(r"(-?[0-9]+),(-?[0-9]+).*=(-?[0-9]+),(-?[0-9]+)").unwrap();
        for (_, [no1, no2, no3, no4]) in re.captures_iter(i).map(|c| c.extract()) {
            robots.push(Robot {
                pos: (no1.parse().unwrap(), no2.parse().unwrap()),
                vel: (no3.parse().unwrap(), no4.parse().unwrap()),
            });
        }
    }

    // for i in robots.clone() {
    //     println!("p={},{} v={},{}", i.pos.0, i.pos.1, i.vel.0, i.vel.1);
    // }

    let add_vectors = |a: (i32, i32), b: (i32, i32)| -> (i32, i32) { (a.0 + b.0, a.1 + b.1) };

    let mut new_bots = vec![];
    let width = 101;
    let height = 103;
    for mut robot in robots {
        for _ in 0..100 {
            let mut new_pos = add_vectors(robot.pos, robot.vel);
            if new_pos.0 < 0 {
                new_pos.0 += width;
            }
            if new_pos.0 >= width {
                new_pos.0 -= width;
            }
            if new_pos.1 < 0 {
                new_pos.1 += height;
            }
            if new_pos.1 >= height {
                new_pos.1 -= height;
            }

            robot.pos = new_pos;
        }
        new_bots.push(robot);
    }

    let q1_count = new_bots
        .iter()
        .filter(|x| {
            x.pos.0 < (width as f32 / 2.0).floor() as i32
                && x.pos.1 < (height as f32 / 2.0).floor() as i32
        })
        .count();
    let q2_count = new_bots
        .iter()
        .filter(|x| {
            x.pos.0 > (width as f32 / 2.0).floor() as i32
                && x.pos.1 < (height as f32 / 2.0).floor() as i32
        })
        .count();
    let q3_count = new_bots
        .iter()
        .filter(|x| {
            x.pos.0 > (width as f32 / 2.0).floor() as i32
                && x.pos.1 > (height as f32 / 2.0).floor() as i32
        })
        .count();
    let q4_count = new_bots
        .iter()
        .filter(|x| {
            x.pos.0 < (width as f32 / 2.0).floor() as i32
                && x.pos.1 > (height as f32 / 2.0).floor() as i32
        })
        .count();

    println!("{}", q1_count * q2_count * q3_count * q4_count);
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let mut robots = vec![];
    for i in input.lines() {
        let re = Regex::new(r"(-?[0-9]+),(-?[0-9]+).*=(-?[0-9]+),(-?[0-9]+)").unwrap();
        for (_, [no1, no2, no3, no4]) in re.captures_iter(i).map(|c| c.extract()) {
            robots.push(Robot {
                pos: (no1.parse().unwrap(), no2.parse().unwrap()),
                vel: (no3.parse().unwrap(), no4.parse().unwrap()),
            });
        }
    }

    // for i in robots.clone() {
    //     println!("p={},{} v={},{}", i.pos.0, i.pos.1, i.vel.0, i.vel.1);
    // }

    let add_vectors = |a: (i32, i32), b: (i32, i32)| -> (i32, i32) { (a.0 + b.0, a.1 + b.1) };

    const width: i32 = 101;
    const height: i32 = 103;
    let mut map = [["."; width as usize]; height as usize];
    for i in 0..22 {
        let mut new_bots = vec![];
        map = [["."; width as usize]; height as usize];
        for mut robot in robots.clone() {
            let mut new_pos = add_vectors(robot.pos, robot.vel);
            if new_pos.0 < 0 {
                new_pos.0 += width;
            }
            if new_pos.0 >= width {
                new_pos.0 -= width;
            }
            if new_pos.1 < 0 {
                new_pos.1 += height;
            }
            if new_pos.1 >= height {
                new_pos.1 -= height;
            }

            robot.pos = new_pos;
            new_bots.push(robot);
            map[robot.pos.1 as usize][robot.pos.0 as usize] = "#";
        }
        robots = new_bots;
    }
    println!("\n\n{}-------", 22);
    for i in map {
        for j in i {
            print!("{j}");
        }
        println!();
    }
    for i in 1..1000 {
        let mut map = [["."; width as usize]; height as usize];
        for j in 0..101 {
            map = [["."; width as usize]; height as usize];
            let mut new_bots = vec![];
            for mut robot in robots.clone() {
                let mut new_pos = add_vectors(robot.pos, robot.vel);
                if new_pos.0 < 0 {
                    new_pos.0 += width;
                }
                if new_pos.0 >= width {
                    new_pos.0 -= width;
                }
                if new_pos.1 < 0 {
                    new_pos.1 += height;
                }
                if new_pos.1 >= height {
                    new_pos.1 -= height;
                }

                robot.pos = new_pos;
                new_bots.push(robot);
                map[robot.pos.1 as usize][robot.pos.0 as usize] = "#";
            }
            robots = new_bots;
        }

        println!("\n\n{}-------", 22 + i * 101);
        for i in map {
            for j in i {
                print!("{j}");
            }
            println!();
        }
    }
}
