use std::{collections::HashSet, fs::read_to_string};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|x| x.chars().map(|y| y.to_string().parse().unwrap()).collect())
        .collect();

    let mut starting_positions = vec![];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                starting_positions.push((j, i));
            }
        }
    }

    let mut scores = 0;
    for i in starting_positions {
        scores += traverse_route(&map, i).len();
    }

    println!("{}", scores);
}

fn traverse_route(map: &Vec<Vec<u32>>, starting_pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let (mut x, mut y) = starting_pos;

    let mut num_trails = HashSet::new();
    loop {
        let current_height = map[y][x];
        if current_height == 9 {
            num_trails.insert((x, y));
            return num_trails;
        }
        let mut next_positions = vec![];
        if x > 0 && map[y][x - 1] == current_height + 1 {
            next_positions.push((x - 1, y));
        }
        if x < map[y].len() - 1 && map[y][x + 1] == current_height + 1 {
            next_positions.push((x + 1, y));
        }
        if y > 0 && map[y - 1][x] == current_height + 1 {
            next_positions.push((x, y - 1));
        }
        if y < map.len() - 1 && map[y + 1][x] == current_height + 1 {
            next_positions.push((x, y + 1));
        }

        if next_positions.is_empty() {
            break;
        } else if next_positions.len() > 1 {
            for i in next_positions {
                num_trails.extend(traverse_route(map, i));
            }
            break;
        }

        (x, y) = next_positions[0];
    }
    num_trails
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|x| x.chars().map(|y| y.to_string().parse().unwrap()).collect())
        .collect();

    let mut starting_positions = vec![];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                starting_positions.push((j, i));
            }
        }
    }

    let mut num_trails = 0;
    for i in starting_positions {
        num_trails += traverse_route2(&map, i);
    }

    println!("{}", num_trails);
}

fn traverse_route2(map: &Vec<Vec<u32>>, starting_pos: (usize, usize)) -> usize {
    let (mut x, mut y) = starting_pos;

    let mut num_trails = 0;
    loop {
        let current_height = map[y][x];
        if current_height == 9 {
            num_trails += 1;
            return num_trails;
        }
        let mut next_positions = vec![];
        if x > 0 && map[y][x - 1] == current_height + 1 {
            next_positions.push((x - 1, y));
        }
        if x < map[y].len() - 1 && map[y][x + 1] == current_height + 1 {
            next_positions.push((x + 1, y));
        }
        if y > 0 && map[y - 1][x] == current_height + 1 {
            next_positions.push((x, y - 1));
        }
        if y < map.len() - 1 && map[y + 1][x] == current_height + 1 {
            next_positions.push((x, y + 1));
        }

        if next_positions.is_empty() {
            break;
        } else if next_positions.len() > 1 {
            for i in next_positions {
                num_trails += traverse_route2(map, i);
            }
            break;
        }

        (x, y) = next_positions[0];
    }
    num_trails
}
