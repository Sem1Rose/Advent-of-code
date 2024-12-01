use std::fs::read_to_string;

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();
    let lists = input.lines().map(|x| {
        x.split(' ')
            .flat_map(|y| y.parse::<u32>())
            .collect::<Vec<u32>>()
    });

    let mut right = lists.clone().map(|x| x[0]).collect::<Vec<u32>>();
    let mut left = lists.map(|x| x[1]).collect::<Vec<u32>>();
    right.sort();
    left.sort();

    let sum = right
        .iter()
        .zip(left)
        .fold(0, |x, (y, z)| x + y.abs_diff(z));

    println!("{sum}");
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let (left, right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|x| {
            x.split(' ')
                .flat_map(|y| y.parse::<u32>())
                .collect::<Vec<u32>>()
        })
        .map(|x| (x[0], x[1]))
        .collect::<Vec<(u32, u32)>>()
        .into_iter()
        .unzip();

    let sum = left.iter().fold(0, |x, y| {
        x + (y * right.iter().filter(|z| *z == y).count() as u32)
    });

    println!("{sum}");
}
