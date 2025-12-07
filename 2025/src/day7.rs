use std::{collections::HashSet, fs};

pub fn part_one() {
    let input = fs::read_to_string("./src/input").unwrap();
    let map = input
        .lines()
        .filter(|x| !x.chars().all(|x| x == '.'))
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // for i in &map {
    //     for x in i {
    //         print!("{x}");
    //     }
    //     println!();
    // }

    let mut beams = HashSet::from([(0usize, map[0].iter().position(|x| *x == 'S').unwrap())]);
    let mut found_splitters = HashSet::new();
    let mut splits = 0;
    while !beams.is_empty() {
        let mut new_beams = HashSet::new();
        for beam in beams {
            // if beam.0 == map.len() - 1 {
            //     continue;
            // }

            if !found_splitters.insert((beam.0 + 1, beam.1)) {
                continue;
            }
            splits += 1;
            let mut split = vec![];
            if beam.1 < map[0].len() - 1 {
                split.push((beam.0 + 1, beam.1 + 1));
            }
            if beam.1 > 0 {
                split.push((beam.0 + 1, beam.1 - 1));
            }
            for new_beam in split.iter_mut() {
                // println!(">{:?}", new_beam);
                while new_beam.0 < map.len() - 1 {
                    if map[new_beam.0 + 1][new_beam.1] == '^' {
                        break;
                    }
                    new_beam.0 += 1;
                }

                if new_beam.0 != map.len() - 1 {
                    // println!("+{:?}", new_beam);
                    new_beams.insert(*new_beam);
                }
            }
        }
        beams = new_beams;
    }

    println!("{splits}");
}

pub fn part_two() {
    let input = fs::read_to_string("./src/input").unwrap();
    let map = input
        .lines()
        .filter(|x| !x.chars().all(|x| x == '.'))
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut scores = vec![vec![]; map.len()];
    for i in (0..map.len()).rev() {
        for j in 0..map[0].len() {
            let score = if i == map.len() - 1 {
                if map[i][j] == '^' { 2 } else { 1 }
            } else {
                if map[i][j] == '^' {
                    (if j == 0 { 0 } else { scores[i + 1][j - 1] })
                        + if j == map[0].len() - 1 {
                            0u64
                        } else {
                            scores[i + 1][j + 1]
                        }
                } else {
                    scores[i + 1][j]
                }
            };
            scores[i].push(score);
        }
    }

    // scores.reverse();
    // for i in &scores {
    //     for x in i {
    //         print!(",{x:^4}")
    //     }
    //     println!();
    // }
    println!(
        "{}",
        scores[0][map[0].iter().position(|x| *x == 'S').unwrap()]
    );

    // BRUTE FORCE!!!!
    // let mut beams = vec![(0usize, map[0].iter().position(|x| *x == 'S').unwrap())];
    // let mut worlds = 1;
    // while !beams.is_empty() {
    //     let mut new_beams = vec![];
    //     for beam in beams {
    //         worlds -= 1;

    //         let mut split = vec![];
    //         if beam.1 < map[0].len() - 1 {
    //             split.push((beam.0 + 1, beam.1 + 1));
    //             worlds += 1;
    //         }
    //         if beam.1 > 0 {
    //             split.push((beam.0 + 1, beam.1 - 1));
    //             worlds += 1;
    //         }
    //         for new_beam in split.iter_mut() {
    //             while new_beam.0 < map.len() - 1 {
    //                 if map[new_beam.0 + 1][new_beam.1] == '^' {
    //                     break;
    //                 }
    //                 new_beam.0 += 1;
    //             }

    //             if new_beam.0 != map.len() - 1 {
    //                 new_beams.push(*new_beam);
    //             }
    //         }
    //     }
    //     beams = new_beams;
    // }
}
