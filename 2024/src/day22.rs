use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let secrets = input.lines();
    let mut new_secrets = vec![];
    for secret in secrets.clone() {
        let mut num = secret.parse::<u64>().unwrap();

        for _ in 0..2000 {
            num = ((num << 6) ^ num) & 16777215;
            num = ((num >> 5) ^ num) & 16777215;
            num = ((num << 11) ^ num) & 16777215;
        }
        new_secrets.push(num);
    }

    // for i in secrets.zip(new_secrets) {
    //     println!("{}:\t{}", i.0, i.1);
    // }

    println!("{}", new_secrets.iter().sum::<u64>());
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();
}
