use std::{fs, num, sync::WaitTimeoutResult};

pub fn part_one() {
    let input = fs::read_to_string("./src/input").unwrap();
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { true } else { false })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut num_accessible_rolls = 0;
    for i in 0..map.len() as i32 {
        for j in 0..map[0].len() as i32 {
            if !map[i as usize][j as usize] {
                continue;
            }
            let mut num_surrounding_rolls = 0;
            for k in -1..=1 {
                for l in -1..=1 {
                    if i + k < 0 || (i + k) as usize == map.len() {
                        continue;
                    }
                    if j + l < 0 || (j + l) as usize >= map[0].len() {
                        continue;
                    }
                    if k == 0 && l == 0 {
                        continue;
                    }

                    if map[(i + k) as usize][(j + l) as usize] {
                        num_surrounding_rolls += 1;
                    }
                }
            }

            if num_surrounding_rolls < 4 {
                num_accessible_rolls += 1;
            }
        }
    }

    println!("{num_accessible_rolls}");
}

pub fn part_two() {
    let input = fs::read_to_string("./src/input").unwrap();
    let mut map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { true } else { false })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut num_accessible_rolls = 0;
    loop {
        let mut new_map = map.clone();
        let mut finished = true;
        for i in 0..map.len() as i32 {
            for j in 0..map[0].len() as i32 {
                if !map[i as usize][j as usize] {
                    continue;
                }
                let mut num_surrounding_rolls = 0;
                for k in -1..=1 {
                    for l in -1..=1 {
                        if i + k < 0 || (i + k) as usize == map.len() {
                            continue;
                        }
                        if j + l < 0 || (j + l) as usize >= map[0].len() {
                            continue;
                        }
                        if k == 0 && l == 0 {
                            continue;
                        }

                        if map[(i + k) as usize][(j + l) as usize] {
                            num_surrounding_rolls += 1;
                        }
                    }
                }

                if num_surrounding_rolls < 4 {
                    num_accessible_rolls += 1;

                    finished = false;
                    new_map[i as usize][j as usize] = false;
                }
            }
        }

        if finished {
            break;
        }
        map = new_map;
    }
    println!("{num_accessible_rolls}");
}
