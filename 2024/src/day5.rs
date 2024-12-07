use std::collections::HashMap;
use std::fs::read_to_string;

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let rules: Vec<(u32, u32)> = input
        .lines()
        .filter(|x| x.contains('|'))
        .map(|x| {
            (
                x.split('|').next().unwrap().parse().unwrap(),
                x.split('|').next_back().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let updates: Vec<Vec<u32>> = input
        .lines()
        .filter(|x| !x.contains('|'))
        .map(|x| x.split(',').map(|y| y.parse().unwrap()).collect())
        .collect();

    let mut incorrect_order_dictionary: HashMap<u32, Vec<u32>> = HashMap::new();
    for (first, second) in rules {
        incorrect_order_dictionary
            .entry(second)
            .and_modify(|x| x.push(first))
            .or_insert(vec![first]);
    }

    let mut mid_page_num_sum = 0;
    'outer: for update in updates {
        let mut update_iter = update.clone().into_iter();
        while let Some(i) = update_iter.next() {
            if !incorrect_order_dictionary.contains_key(&i) {
                continue;
            }
            let incorrect_ordering = incorrect_order_dictionary.get(&i).unwrap();

            for j in update_iter.clone() {
                if incorrect_ordering.contains(&j) {
                    continue 'outer;
                }
            }
        }
        mid_page_num_sum += update[update.len() / 2];
    }

    println!("{}", mid_page_num_sum);
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let rules: Vec<(u32, u32)> = input
        .lines()
        .filter(|x| x.contains('|'))
        .map(|x| {
            (
                x.split('|').next().unwrap().parse().unwrap(),
                x.split('|').next_back().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let updates: Vec<Vec<u32>> = input
        .lines()
        .filter(|x| !x.contains('|'))
        .map(|x| x.split(',').map(|y| y.parse().unwrap()).collect())
        .collect();

    let mut incorrect_order_dictionary: HashMap<u32, Vec<u32>> = HashMap::new();
    for (first, second) in rules {
        incorrect_order_dictionary
            .entry(second)
            .and_modify(|x| x.push(first))
            .or_insert(vec![first]);
    }

    let mut mid_page_num_sum = 0;
    for update in updates {
        if !test_ordering(&update, &incorrect_order_dictionary) {
            mid_page_num_sum +=
                fix_ordering(&update, &incorrect_order_dictionary)[update.len() / 2];
        }
    }

    println!("{}", mid_page_num_sum);
}

fn fix_ordering(bad_update: &Vec<u32>, dictionary: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut fixed_update = bad_update.clone();

    loop {
        'outer: for i in 0..fixed_update.len() {
            if !dictionary.contains_key(&fixed_update[i]) {
                continue;
            }

            let incorrect_ordering = dictionary.get(&fixed_update[i]).unwrap();
            for j in (i + 1)..fixed_update.len() {
                if incorrect_ordering.contains(&fixed_update[j]) {
                    fixed_update.swap(i, j);
                    break 'outer;
                }
            }
        }
        if test_ordering(&fixed_update, dictionary) {
            break;
        }
    }
    fixed_update
}

fn test_ordering(update: &[u32], dictionary: &HashMap<u32, Vec<u32>>) -> bool {
    let mut update_iter = update.iter();
    while let Some(i) = update_iter.next() {
        if !dictionary.contains_key(i) {
            continue;
        }

        let incorrect_ordering = dictionary.get(i).unwrap();
        for j in update_iter.clone() {
            if incorrect_ordering.contains(j) {
                return false;
            }
        }
    }
    true
}
