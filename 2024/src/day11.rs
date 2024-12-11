use std::{
    collections::HashMap,
    collections::HashSet,
    fs::read_to_string,
    sync::{Arc, Mutex},
    thread,
};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let mut pebbles = input.split(' ').map(|x| x.to_string()).collect::<Vec<_>>();

    // println!("{}", pebbles.join(" "));
    for _ in 0..25 {
        pebbles = blink(&pebbles);
        // println!("{}", pebbles.join(" "));
    }

    println!("{}", pebbles.len())
}

fn blink(pebbles: &[String]) -> Vec<String> {
    let mut new_pebbles = vec![];
    for pebble in 0..pebbles.len() {
        if pebbles[pebble] == "0" {
            new_pebbles.push("1".to_string());
        } else if pebbles[pebble].len() % 2 == 0 {
            let len = pebbles[pebble].len() / 2;
            new_pebbles.push(pebbles[pebble][0..len].to_string());
            new_pebbles.push(pebbles[pebble][len..].parse::<u128>().unwrap().to_string());
        } else {
            new_pebbles.push((pebbles[pebble].parse::<u128>().unwrap() * 2024).to_string());
        }
    }

    new_pebbles
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let pebbles = input
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let depth = calc_blinks(pebbles, 39);
    let hashes = HashSet::from_iter(depth.clone());

    println!("{}", hashes.len());

    let hashmap = populate_hashmap(hashes, 36);

    let mut sum = 0;
    for i in depth {
        sum += hashmap.get(&i).unwrap();
    }

    println!("{sum}");
    // println!("{}", pebbles.join(" "));
    // println!("{}", calc_blinks(pebbles, &mut hashmap, 35, 0).len());
}

// fn blink2(pebbles: &[u128]) -> Vec<u128> {
//     let mut new_pebbles = vec![];
//     for pebble in 0..pebbles.len() {
//         if pebbles[pebble] == 0 {
//             new_pebbles.push(1);
//         } else {
//             let len = (0..)
//                 .take_while(|i| 10u128.pow(*i) <= pebbles[pebble])
//                 .count();
//             if len % 2 == 0 {
//                 new_pebbles.push(pebbles[pebble] / 10u128.pow(len as u32 / 2));
//                 new_pebbles.push(pebbles[pebble] % 10u128.pow(len as u32 / 2));
//             } else {
//                 new_pebbles.push(pebbles[pebble] * 2024);
//             }
//         }
//     }

//     new_pebbles
// }

// fn calc_blinks(pebbles: Vec<u128>, num_passes: u32) -> usize {
//     let new_pebbles_count = Arc::new(Mutex::new(0));
//     let mut handles = vec![];
//     for pebble in pebbles {
//         let count = Arc::clone(&new_pebbles_count);

//         let handle = thread::spawn(move || {
//             let mut pebbs_tree = vec![pebble];
//             for _ in 0..num_passes {
//                 let mut new_tree = vec![];
//                 for i in 0..pebbs_tree.len() {
//                     let pebb = pebbs_tree[i];
//                     if pebb == 0 {
//                         new_tree.push(1);
//                     } else {
//                         let len = (0..).take_while(|x| 10u128.pow(*x) <= pebb).count();
//                         if len % 2 == 0 {
//                             new_tree.push(pebb / 10u128.pow(len as u32 / 2));
//                             new_tree.push(pebb % 10u128.pow(len as u32 / 2));
//                         } else {
//                             new_tree.push(pebb * 2024);
//                         }
//                     }
//                 }
//                 pebbs_tree = new_tree;
//             }

//             *count.lock().unwrap() += pebbs_tree.len();
//         });

//         handles.push(handle);
//     }

//     for (j, handle) in handles.into_iter().enumerate() {
//         handle.join().unwrap();
//         println!("{j}");
//     }

//     let x = *new_pebbles_count.lock().unwrap();
//     x
// }

fn populate_hashmap(pebbles: HashSet<u64>, depth: u32) -> HashMap<u64, u64> {
    let hashmap = Arc::new(Mutex::new(HashMap::<u64, u64>::new()));
    let mut handles = vec![];
    for (i, pebble) in pebbles.clone().into_iter().enumerate() {
        let h_map = Arc::clone(&hashmap);

        let handle = thread::spawn(move || {
            let mut tree = vec![pebble];
            for _ in 0..depth {
                tree = tree
                    .iter()
                    .flat_map(|x| {
                        if *x == 0 {
                            vec![1]
                        } else {
                            let len = (0..).take_while(|y| 10u64.pow(*y) <= *x).count();
                            if len & 1 == 0 {
                                vec![x / 10u64.pow(len as u32 / 2), x % 10u64.pow(len as u32 / 2)]
                            } else {
                                vec![*x * 2024]
                            }
                        }
                    })
                    .collect();
            }

            h_map.lock().unwrap().insert(pebble, tree.len() as u64);
            println!("{i}, {pebble}");
            drop(tree);
        });
        handles.push(handle);

        while handles.len() >= 16 {
            thread::sleep(std::time::Duration::from_millis(100));
            for i in 0..handles.len() {
                if handles[i].is_finished() {
                    handles.remove(i);
                    break;
                }
            }
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let x = hashmap.lock().unwrap().clone();
    x
}

fn calc_blinks(pebbles: Vec<u64>, depth: u32) -> Vec<u64> {
    let final_tree = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];
    for i in 0..pebbles.len() {
        let pebble = pebbles[i];
        let f_tree = Arc::clone(&final_tree);

        let handle = thread::spawn(move || {
            let mut tree = vec![pebble];
            for j in 0..depth {
                println!("{i} => {j}: {}", tree.len());
                tree = tree
                    .iter()
                    .flat_map(|x| {
                        if *x == 0 {
                            vec![1]
                        } else {
                            let len = (0..).take_while(|y| 10u64.pow(*y) <= *x).count();
                            if len & 1 == 0 {
                                vec![x / 10u64.pow(len as u32 / 2), x % 10u64.pow(len as u32 / 2)]
                            } else {
                                vec![*x * 2024]
                            }
                        }
                    })
                    .collect();
            }

            f_tree.lock().unwrap().extend(tree);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let x = final_tree.lock().unwrap().clone();
    x
}

// fn calc_blinks(pebbles: Vec<u64>, num_passes: u32) -> usize {
//     let mut new_pebbles_count = 0;
//     let mut cache: HashMap<u64, (u32, Vec<u64>)> = HashMap::new();
//     for i in 0..pebbles.len() {
//         let pebble = pebbles[i];
//         // let counter = Arc::clone(&new_pebbles_count);

//         let mut pebbs_tree = vec![pebble];
//         for j in 0..num_passes {
//             println!("{i} => {j}: {}", pebbs_tree.len());
//             let new_tree: Vec<Vec<u64>> = pebbs_tree
//                 .iter()
//                 .map(|x| {
//                     if cache.contains_key(x) {
//                         return cache.get(x).unwrap().1.to_vec().clone();
//                     }
//                     if *x == 0 {
//                         vec![1]
//                     } else {
//                         let len = (0..).take_while(|y| 10u64.pow(*y) <= *x).count();
//                         if len & 1 == 0 {
//                             vec![x / 10u64.pow(len as u32 / 2), x % 10u64.pow(len as u32 / 2)]
//                         } else {
//                             vec![*x * 2024]
//                         }
//                     }
//                 })
//                 .collect();

//             for i in 0..pebbs_tree.len() {
//                 if cache.contains_key(i) && cache.get(i).unwrap().0 +
//                 cache
//                     .entry(pebbs_tree[i])
//                     .or_insert_with(|| (j, new_tree[i].clone()));
//             }

//             pebbs_tree = new_tree.iter().flatten().copied().collect();
//             // for i in 0..pebbs_tree.len() {
//             //     let pebb = pebbs_tree[i];
//             //     if pebb == 0 {
//             //         new_tree.push(1);
//             //     } else {
//             //         let len = (0..).take_while(|x| 10u64.pow(*x) <= pebb).count();
//             //         if len % 2 == 0 {
//             //             new_tree.push(pebb / 10u64.pow(len as u32 / 2));
//             //             new_tree.push(pebb % 10u64.pow(len as u32 / 2));
//             //         } else {vec
//             //             new_tree.push(pebb * 2024);
//             //         }
//             //     }
//             // }
//         }

//         new_pebbles_count += pebbs_tree.len();
//     }

//     new_pebbles_count
// }
