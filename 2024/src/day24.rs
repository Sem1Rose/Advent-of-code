use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let mut wires: HashMap<String, bool> = HashMap::new();
    let mut operations: Vec<(char, (String, String), String)> = vec![];

    input.split("\n\n").next().unwrap().lines().for_each(|x| {
        let mut split = x.split(": ");
        wires.insert(
            split.next().unwrap().to_string(),
            split.next().unwrap() == "1",
        );
    });

    input.split("\n\n").last().unwrap().lines().for_each(|x| {
        let mut split = x.split(' ');
        let input1 = split.next().unwrap();
        let operation = match split.next().unwrap() {
            "AND" => '&',
            "OR" => '|',
            "XOR" => '^',
            _ => ' ',
        };
        let input2 = split.next().unwrap();
        split.next();

        operations.push((
            operation,
            (input1.to_string(), input2.to_string()),
            split.next().unwrap().to_string(),
        ));
    });

    let mut outputs: Vec<_> = operations
        .iter()
        .map(|x| x.2.clone())
        .filter(|x| x.starts_with('z'))
        .collect();
    outputs.sort();
    outputs.reverse();

    // for i in wires {
    //     println!("{}: {}", i.0, i.1);
    // }
    // println!();
    // for i in operations {
    //     println!("{} {} {}: {}", i.1 .0, i.0, i.1 .1, i.2);
    // }

    while !operations.is_empty() {
        let mut new_operations = vec![];
        for op in operations {
            if wires.contains_key(&op.2) {
                continue;
            }
            if wires.contains_key(&op.1 .0) && wires.contains_key(&op.1 .1) {
                let out = match op.0 {
                    '&' => wires[&op.1 .0] & wires[&op.1 .1],
                    '^' => wires[&op.1 .0] ^ wires[&op.1 .1],
                    '|' => wires[&op.1 .0] | wires[&op.1 .1],
                    _ => false,
                };
                wires.insert(op.2, out);
            } else {
                new_operations.push(op);
            }
        }

        operations = new_operations;
    }

    let mut num = 0;
    for i in outputs {
        num = (num << 1) | wires[&i] as u128;
    }

    println!("{:#b}, {0}", num);
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();
}
