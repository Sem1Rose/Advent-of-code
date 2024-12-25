use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let mut keys = vec![];
    let mut locks = vec![];
    for kl in input.split("\n\n") {
        let mut lines = kl.lines();
        let columns = vec![
            lines
                .clone()
                .map(|x| x.chars().nth(0).unwrap())
                .filter(|x| *x == '#')
                .count()
                - 1,
            lines
                .clone()
                .map(|x| x.chars().nth(1).unwrap())
                .filter(|x| *x == '#')
                .count()
                - 1,
            lines
                .clone()
                .map(|x| x.chars().nth(2).unwrap())
                .filter(|x| *x == '#')
                .count()
                - 1,
            lines
                .clone()
                .map(|x| x.chars().nth(3).unwrap())
                .filter(|x| *x == '#')
                .count()
                - 1,
            lines
                .clone()
                .map(|x| x.chars().nth(4).unwrap())
                .filter(|x| *x == '#')
                .count()
                - 1,
        ];

        if lines.next().unwrap().contains('.') {
            keys.push(columns);
        } else {
            locks.push(columns);
        }
    }

    // println!("Keys: ");
    // for i in keys {
    //     println!("\t{:#?}", i);
    // }

    // println!("Locks: ");
    // for i in locks {
    //     println!("\t{:#?}", i);
    // }

    let add = |a: &Vec<usize>, b: &Vec<usize>| -> Vec<usize> {
        let mut out = vec![];
        for i in 0..a.len() {
            out.push(a[i] + b[i]);
        }
        out
    };

    let mut key_lock_pairs = vec![];
    for lock in locks {
        for key in keys.clone() {
            key_lock_pairs.push(add(&lock, &key));
        }
    }

    println!(
        "{}",
        key_lock_pairs
            .iter()
            .filter(|x| !x.iter().any(|y| *y > 5))
            .count()
    );
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();
}
