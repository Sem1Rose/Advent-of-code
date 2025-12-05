use std::fs;

pub fn part_one() {
    let input = fs::read_to_string("./src/input").unwrap();
    let ranges = input.split("\n\n").collect::<Vec<_>>()[0]
        .lines()
        .map(|line| {
            <[u64; 2]>::try_from(
                line.split('-')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    let ids = input.split("\n\n").collect::<Vec<_>>()[1]
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut num_valid_ids = 0;
    for id in ids {
        for range in &ranges {
            if id >= range[0] && id <= range[1] {
                num_valid_ids += 1;
                break;
            }
        }
    }

    println!("{num_valid_ids}");
}

pub fn part_two() {
    let input = fs::read_to_string("./src/input").unwrap();
    let mut ranges = input.split("\n\n").collect::<Vec<_>>()[0]
        .lines()
        .map(|line| {
            <[u64; 2]>::try_from(
                line.split('-')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    ranges.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut new_ranges: Vec<[u64; 2]> = vec![];
    for range in &ranges {
        let pos = new_ranges.iter().position(|x| range[0] <= x[1]);
        if let Some(pos) = pos {
            new_ranges[pos][1] = new_ranges[pos][1].max(range[1]);
        } else {
            new_ranges.push(*range);
        }
    }

    let mut valid_ids = 0;
    for range in &new_ranges {
        valid_ids += range[1] - range[0] + 1;
    }

    println!("{}", valid_ids);
}
