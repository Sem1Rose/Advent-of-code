use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let map: Vec<Vec<String>> = input
        .lines()
        .map(|x| x.chars().map(|y| y.to_string()).collect())
        .collect();
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    // println!("{width}X{height}");

    let mut antenna_groups: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    (0..map.len()).for_each(|i| {
        (0..map[i].len()).for_each(|j| {
            if map[i][j] != "." {
                antenna_groups
                    .entry(map[i][j].clone())
                    .and_modify(|x| x.push((j as i32, i as i32)))
                    .or_insert(vec![(j as i32, i as i32)]);
            }
        });
    });

    let in_range = |value: i32, min: i32, max: i32| -> bool { value < max && value >= min };
    let mut antinodes = HashSet::new();
    for (_, antennas) in antenna_groups.iter() {
        let mut antennas_iter = antennas.clone().into_iter();
        for _ in 0..antennas.len() {
            let x = antennas_iter.next().unwrap();
            // println!("({}, {})", x.0, x.1);

            for i in antennas_iter.clone() {
                let diff = (i.0 - x.0, i.1 - x.1);
                // println!("  ({}, {})", i.0, i.1);
                // println!("      ({}, {})", diff.0, diff.1);

                let backward = (x.0 - diff.0, x.1 - diff.1);
                let forward = (i.0 + diff.0, i.1 + diff.1);
                // println!("          ({}, {})", backward.0, backward.1);
                // println!("          ({}, {})", forward.0, forward.1);
                if in_range(backward.0, 0, width) && in_range(backward.1, 0, height) {
                    antinodes.insert(backward);
                }
                if in_range(forward.0, 0, width) && in_range(forward.1, 0, height) {
                    antinodes.insert(forward);
                }
            }
        }
    }
    println!("{}", antinodes.len());
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let map: Vec<Vec<String>> = input
        .lines()
        .map(|x| x.chars().map(|y| y.to_string()).collect())
        .collect();
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    // println!("{width}X{height}");

    let mut antenna_groups: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    (0..map.len()).for_each(|i| {
        (0..map[i].len()).for_each(|j| {
            if map[i][j] != "." {
                antenna_groups
                    .entry(map[i][j].clone())
                    .and_modify(|x| x.push((j as i32, i as i32)))
                    .or_insert(vec![(j as i32, i as i32)]);
            }
        });
    });

    let in_range = |value: i32, min: i32, max: i32| -> bool { value < max && value >= min };
    let mut antinodes = HashSet::new();
    for (_, antennas) in antenna_groups.iter() {
        let mut antennas_iter = antennas.clone().into_iter();
        if antennas_iter.len() > 1 {
            for i in antennas_iter.clone() {
                antinodes.insert(i);
            }
        }
        for _ in 0..antennas.len() {
            let x = antennas_iter.next().unwrap();
            // println!("({}, {})", x.0, x.1);

            for i in antennas_iter.clone() {
                let diff = (i.0 - x.0, i.1 - x.1);
                // println!("  ({}, {})", i.0, i.1);
                // println!("      ({}, {})", diff.0, diff.1);

                let mut backward = (x.0 - diff.0, x.1 - diff.1);
                let mut forward = (i.0 + diff.0, i.1 + diff.1);
                // println!("          ({}, {})", backward.0, backward.1);
                // println!("          ({}, {})", forward.0, forward.1);

                while in_range(backward.0, 0, width) && in_range(backward.1, 0, height) {
                    antinodes.insert(backward);
                    backward = (backward.0 - diff.0, backward.1 - diff.1);
                }
                while in_range(forward.0, 0, width) && in_range(forward.1, 0, height) {
                    antinodes.insert(forward);
                    forward = (forward.0 + diff.0, forward.1 + diff.1);
                }
            }
        }
    }
    println!("{}", antinodes.len());
}
