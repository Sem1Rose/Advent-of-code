use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let connection: Vec<_> = line.split('-').collect();
        connections
            .entry(connection[0].to_string())
            .and_modify(|x| x.push(connection[1].to_string()))
            .or_insert(vec![connection[1].to_string()]);
        connections
            .entry(connection[1].to_string())
            .and_modify(|x| x.push(connection[0].to_string()))
            .or_insert(vec![connection[0].to_string()]);
    }

    println!("ass");

    let mut visited: Vec<HashSet<String>> = vec![];
    for (level0, c) in connections.clone() {
        if !level0.starts_with("t") {
            continue;
        }
        for level1 in c {
            for level2 in &connections.clone()[&level1] {
                for l0 in &connections.clone()[level2] {
                    if l0 == &level0 {
                        let chain = HashSet::from_iter(vec![
                            level0.clone(),
                            level1.clone(),
                            level2.clone(),
                        ]);
                        if !visited.iter().any(|x| x == &chain) {
                            visited.push(chain);
                        }
                    }
                }
            }
        }
    }

    // println!("ass");

    // let mut count = 0;
    // for v in visited.clone() {
    //     if v.iter().any(|x| x.starts_with('t')) {
    //         // println!("{:?}", v);
    //         count += 1;
    //     }
    // }

    // println!("{count}");
    println!("{}", visited.len());
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();
}
