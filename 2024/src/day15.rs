use std::fs::read_to_string;

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let mut map: Vec<Vec<String>> = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|x| x.chars().map(|y| y.to_string()).collect())
        .collect();
    let moves: Vec<String> = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .chars()
        .map(|x| x.to_string())
        .collect();

    // for i in map {
    //     for j in i {
    //         print!("{j}");
    //     }
    //     println!()
    // }
    // println!();
    // for m in moves {
    //     print!("{m}");
    // }

    let mut robot_pos = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == "@" {
                robot_pos = (j as i32, i as i32);
                break;
            }
        }
    }

    for m in moves {
        let dir = match m.as_str() {
            "<" => (-1, 0),
            "^" => (0, -1),
            ">" => (1, 0),
            "v" => (0, 1),
            _ => (0, 0),
        };

        if test_move(&mut map, robot_pos, dir) {
            robot_pos = (robot_pos.0 + dir.0, robot_pos.1 + dir.1);
        }

        // println!("Move {}:", m);
        // for i in 0..map.len() {
        //     for j in 0..map[i].len() {
        //         print!("{}", map[i][j]);
        //     }
        //     println!()
        // }
        // println!()
    }
    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j]);
            if map[i][j] == "O" {
                sum += i * 100 + j;
            }
        }
        println!()
    }

    println!("{sum}");
}

fn test_move(map: &mut Vec<Vec<String>>, pos: (i32, i32), dir: (i32, i32)) -> bool {
    if map[pos.1 as usize][pos.0 as usize] == "." {
        return true;
    }
    if map[pos.1 as usize][pos.0 as usize] == "#" {
        return false;
    }

    let c = map[pos.1 as usize][pos.0 as usize].clone();
    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if test_move(map, next_pos, dir) {
        map[next_pos.1 as usize][next_pos.0 as usize] = c;
        map[pos.1 as usize][pos.0 as usize] = ".".to_string();
        return true;
    }
    false
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let mut map: Vec<Vec<String>> = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| match y {
                    '#' => "##".to_string(),
                    '.' => "..".to_string(),
                    '@' => "@.".to_string(),
                    'O' => "[]".to_string(),
                    _ => "".to_string(),
                })
                .collect::<Vec<String>>()
                .concat()
                .chars()
                .map(|x| x.to_string())
                .collect()
        })
        .collect();
    let moves: Vec<String> = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .chars()
        .map(|x| x.to_string())
        .collect();

    // for i in map.clone() {
    //     for j in i {
    //         print!("{j}");
    //     }
    //     println!()
    // }
    // println!();
    // for m in moves {
    //     print!("{m}");
    // }

    let mut robot_pos = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == "@" {
                robot_pos = (j as i32, i as i32);
                break;
            }
        }
    }

    for m in moves {
        let dir = match m.as_str() {
            "<" => (-1, 0),
            "^" => (0, -1),
            ">" => (1, 0),
            "v" => (0, 1),
            _ => (0, 0),
        };

        if test_move2(&map, robot_pos, dir) {
            make_move(&mut map, robot_pos, dir);
            robot_pos = (robot_pos.0 + dir.0, robot_pos.1 + dir.1);
        }

        // println!("Move {}:", m);
        // for i in 0..map.len() {
        //     for j in 0..map[i].len() {
        //         print!("{}", map[i][j]);
        //     }
        //     println!()
        // }
        // println!()
    }
    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j]);
            if map[i][j] == "[" {
                sum += i * 100 + j;
            }
        }
        println!()
    }

    println!("{sum}");
}

fn test_move2(map: &Vec<Vec<String>>, pos: (i32, i32), dir: (i32, i32)) -> bool {
    let c = map[pos.1 as usize][pos.0 as usize].clone();
    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);

    if c == "." {
        return true;
    }
    if c == "#" {
        return false;
    }

    if (c == "[" || c == "]") && dir.1 != 0 {
        let second_half_offset = if c == "[" { 1 } else { -1 };
        let second_half_pos = (pos.0 + second_half_offset, pos.1);
        let second_half_next_pos = (second_half_pos.0 + dir.0, second_half_pos.1 + dir.1);

        // if map[next_pos.1 as usize][next_pos.0 as usize] == "#"
        //     || map[second_half_next_pos.1 as usize][second_half_next_pos.0 as usize] == "#"
        // {
        //     return 0;
        // }

        if test_move2(map, next_pos, dir) && test_move2(map, second_half_next_pos, dir) {
            return true;
        }
        // if test_move2(map, next_pos, dir) {
        //     return Some(depth);
        // }
        return false;
    }

    // println!(
    //     "({}, {}), ({}, {}), ({}, {})",
    //     pos.0, pos.1, dir.0, dir.1, next_pos.0, next_pos.1
    // );
    if test_move2(map, next_pos, dir) {
        return true;
    }

    false
}

fn make_move(map: &mut Vec<Vec<String>>, pos: (i32, i32), dir: (i32, i32)) {
    let c = map[pos.1 as usize][pos.0 as usize].clone();
    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);

    if c == "." {
        return;
    }
    if c == "#" {
        return;
    }

    if (c == "[" || c == "]") && dir.1 != 0 {
        let second_half_offset = if c == "[" { 1 } else { -1 };
        let second_half_pos = (pos.0 + second_half_offset, pos.1);
        let second_half_next_pos = (second_half_pos.0 + dir.0, second_half_pos.1 + dir.1);

        make_move(map, next_pos, dir);
        make_move(map, second_half_next_pos, dir);
        map[second_half_next_pos.1 as usize][second_half_next_pos.0 as usize] = if c == "[" {
            "]".to_string()
        } else {
            "[".to_string()
        };
        map[second_half_pos.1 as usize][second_half_pos.0 as usize] = ".".to_string();
        map[next_pos.1 as usize][next_pos.0 as usize] = c;
        map[pos.1 as usize][pos.0 as usize] = ".".to_string();
        return;
    }
    make_move(map, next_pos, dir);
    map[next_pos.1 as usize][next_pos.0 as usize] = c;
    map[pos.1 as usize][pos.0 as usize] = ".".to_string();
}
