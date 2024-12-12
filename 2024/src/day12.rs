use std::{collections::HashMap, fs::read_to_string};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let mut garden: Vec<Vec<String>> = input
        .lines()
        .map(|x| format!(".{x}.").chars().map(|y| y.to_string()).collect())
        .collect();
    garden.insert(
        0,
        ["."]
            .repeat(garden[0].len())
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );
    garden.insert(
        garden.len(),
        ["."]
            .repeat(garden[0].len())
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );

    let mut letters: Vec<Vec<Vec<String>>> = vec![];
    for character in b'A'..=b'Z' {
        let c = (character as char).to_string();
        if !garden.iter().flatten().any(|x| *x == c.to_string()) {
            continue;
        }

        let mut map = vec![];
        for (n, i) in garden.iter().enumerate() {
            map.push(vec![]);
            for j in i {
                if *j == c {
                    map[n].push(c.clone());
                } else {
                    map[n].push(".".to_string());
                }
            }
        }
        letters.push(map);
    }

    let mut separated_letters = vec![];
    for map in letters {
        let mut c = b'A';
        let mut map_separated = map.clone();
        let mut separated: Vec<Vec<bool>> = (0..map_separated.len())
            .map(|x| (0..map_separated[x].len()).map(|_| false).collect())
            .collect();
        for i in 0..map_separated.len() {
            for j in 0..map_separated[i].len() {
                if !separated[i][j] && map_separated[i][j] != "." {
                    separate(&mut map_separated, &mut separated, (i, j), c);
                    c += 1;
                }
            }
        }

        for character in b'A'..=b'z' {
            let c = (character as char).to_string();
            if !map_separated.iter().flatten().any(|x| *x == c) {
                continue;
            }

            let mut map = vec![];
            for (n, i) in map_separated.iter().enumerate() {
                map.push(vec![]);
                for j in i {
                    if *j == c {
                        map[n].push(c.clone());
                    } else {
                        map[n].push(".".to_string());
                    }
                }
            }
            separated_letters.push(map);
        }
    }

    let mut sum = 0;
    for letter_map in separated_letters {
        let area = letter_map
            .iter()
            .fold(0, |acc, x| acc + x.iter().filter(|y| **y != ".").count());
        if area == 0 {
            continue;
        }

        let mut perimeter = 0;
        for i in 0..letter_map.len() {
            for j in 0..letter_map[i].len() {
                if letter_map[i][j] != "." {
                    continue;
                }

                if i > 0 && letter_map[i - 1][j] != "." {
                    perimeter += 1;
                }
                if i < letter_map.len() - 1 && letter_map[i + 1][j] != "." {
                    perimeter += 1;
                }
                if j > 0 && letter_map[i][j - 1] != "." {
                    perimeter += 1;
                }
                if j < letter_map[i].len() - 1 && letter_map[i][j + 1] != "." {
                    perimeter += 1;
                }
            }
        }
        sum += perimeter * area;
    }
    println!("{sum}");
}

fn separate(
    map: &mut Vec<Vec<String>>,
    separated: &mut Vec<Vec<bool>>,
    pos: (usize, usize),
    c: u8,
) {
    if separated[pos.0][pos.1] || map[pos.0][pos.1] == "." {
        return;
    }
    separated[pos.0][pos.1] = true;
    map[pos.0][pos.1] = (c as char).to_string();

    if pos.0 > 0 {
        separate(map, separated, (pos.0, pos.1 - 1), c);
    }
    if pos.0 < map[0].len() - 1 {
        separate(map, separated, (pos.0, pos.1 + 1), c);
    }
    if pos.1 > 0 {
        separate(map, separated, (pos.0 - 1, pos.1), c);
    }
    if pos.1 < map.len() - 1 {
        separate(map, separated, (pos.0 + 1, pos.1), c);
    }
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let mut garden: Vec<Vec<String>> = input
        .lines()
        .map(|x| format!(".{x}.").chars().map(|y| y.to_string()).collect())
        .collect();
    garden.insert(
        0,
        ["."]
            .repeat(garden[0].len())
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );
    garden.insert(
        garden.len(),
        ["."]
            .repeat(garden[0].len())
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );

    let mut letters: Vec<Vec<Vec<String>>> = vec![];
    for character in b'A'..=b'Z' {
        let c = (character as char).to_string();
        if !garden.iter().flatten().any(|x| *x == c.to_string()) {
            continue;
        }

        let mut map = vec![];
        for (n, i) in garden.iter().enumerate() {
            map.push(vec![]);
            for j in i {
                if *j == c {
                    map[n].push(c.clone());
                } else {
                    map[n].push(".".to_string());
                }
            }
        }
        letters.push(map);
    }

    let mut separated_letters = vec![];
    for map in letters {
        let mut c = b'A';
        let mut map_separated = map.clone();
        let mut separated: Vec<Vec<bool>> = (0..map_separated.len())
            .map(|x| (0..map_separated[x].len()).map(|_| false).collect())
            .collect();
        for i in 0..map_separated.len() {
            for j in 0..map_separated[i].len() {
                if !separated[i][j] && map_separated[i][j] != "." {
                    separate(&mut map_separated, &mut separated, (i, j), c);
                    c += 1;
                }
            }
        }

        for character in b'A'..=b'z' {
            let c = (character as char).to_string();
            if !map_separated.iter().flatten().any(|x| *x == c) {
                continue;
            }

            let mut map = vec![];
            for (n, i) in map_separated.iter().enumerate() {
                map.push(vec![]);
                for j in i {
                    if *j == c {
                        map[n].push(c.clone());
                    } else {
                        map[n].push(".".to_string());
                    }
                }
            }
            separated_letters.push(map);
        }
    }

    let mut sum = 0;
    for letter_map in separated_letters {
        let area = letter_map
            .iter()
            .fold(0, |acc, x| acc + x.iter().filter(|y| **y != ".").count());
        if area == 0 {
            continue;
        }

        let mut no_sides = 0;
        {
            let mut heights = HashMap::new();
            let mut in_map = false;
            for j in 0..letter_map[0].len() {
                for i in 0..letter_map.len() {
                    if letter_map[i][j] == "." {
                        in_map = false;
                        continue;
                    }
                    if !in_map {
                        heights
                            .entry(i)
                            .and_modify(|x: &mut Vec<usize>| x.push(j))
                            .or_insert(vec![j]);
                    }
                    in_map = true;
                }
            }
            for i in 0..letter_map.len() {
                if !heights.contains_key(&i) {
                    continue;
                }
                let h = heights.get(&i).unwrap();
                let mut side = false;
                for j in 0..letter_map[i].len() {
                    if h.contains(&j) {
                        if !side {
                            no_sides += 1;
                            side = !side;
                        }
                    } else {
                        side = false;
                    }
                }
            }
        }
        {
            let mut heights = HashMap::new();
            let mut in_map = false;
            for j in 0..letter_map[0].len() {
                for i in (0..letter_map.len()).rev() {
                    if letter_map[i][j] == "." {
                        in_map = false;
                        continue;
                    }
                    if !in_map {
                        heights
                            .entry(i)
                            .and_modify(|x: &mut Vec<usize>| x.push(j))
                            .or_insert(vec![j]);
                    }
                    in_map = true;
                }
            }
            for i in 0..letter_map.len() {
                if !heights.contains_key(&i) {
                    continue;
                }
                let h = heights.get(&i).unwrap();
                let mut side = false;
                for j in 0..letter_map[i].len() {
                    if h.contains(&j) {
                        if !side {
                            no_sides += 1;
                            side = !side;
                        }
                    } else {
                        side = false;
                    }
                }
            }
        }
        {
            let mut depths = HashMap::new();
            let mut in_map = false;
            for i in 0..letter_map.len() {
                for j in 0..letter_map[0].len() {
                    if letter_map[i][j] == "." {
                        in_map = false;
                        continue;
                    }
                    if !in_map {
                        depths
                            .entry(j)
                            .and_modify(|x: &mut Vec<usize>| x.push(i))
                            .or_insert(vec![i]);
                    }
                    in_map = true;
                }
            }
            for j in 0..letter_map[0].len() {
                if !depths.contains_key(&j) {
                    continue;
                }
                let h = depths.get(&j).unwrap();
                let mut side = false;
                for i in 0..letter_map.len() {
                    if h.contains(&i) {
                        if !side {
                            no_sides += 1;
                            side = !side;
                        }
                    } else {
                        side = false;
                    }
                }
            }
        }
        {
            let mut depths = HashMap::new();
            let mut in_map = false;
            for i in 0..letter_map.len() {
                for j in (0..letter_map[0].len()).rev() {
                    if letter_map[i][j] == "." {
                        in_map = false;
                        continue;
                    }
                    if !in_map {
                        depths
                            .entry(j)
                            .and_modify(|x: &mut Vec<usize>| x.push(i))
                            .or_insert(vec![i]);
                    }
                    in_map = true;
                }
            }
            for j in 0..letter_map[0].len() {
                if !depths.contains_key(&j) {
                    continue;
                }
                let h = depths.get(&j).unwrap();
                let mut side = false;
                for i in 0..letter_map.len() {
                    if h.contains(&i) {
                        if !side {
                            no_sides += 1;
                            side = !side;
                        }
                    } else {
                        side = false;
                    }
                }
            }
        }
        sum += no_sides * area;
    }
    println!("{sum}");
}
