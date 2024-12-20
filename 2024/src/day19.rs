use std::{
    fs::read_to_string,
    sync::{Arc, Mutex},
    thread,
};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let available_patterns: Vec<_> = input.split("\n\n").next().unwrap().split(", ").collect();
    let designs: Vec<_> = input.split("\n\n").last().unwrap().lines().collect();

    let mut possible_designs = 0;
    for design in designs {
        // print!("{design}\t");
        if try_design(design, &available_patterns) {
            possible_designs += 1;
            // println!("possible!");
            // } else {
            //     println!("not possible!");
        }
    }

    println!("{possible_designs}");
}

fn try_design(design: &str, available_patterns: &[&str]) -> bool {
    let mut design = design;
    while !design.is_empty() {
        let test_char = design.chars().next().unwrap();
        let possible_patterns: Vec<_> = available_patterns
            .iter()
            .filter(|x| x.len() <= design.len() && x.chars().next().unwrap() == test_char)
            .collect();

        if possible_patterns.is_empty() {
            return false;
        } else if possible_patterns.len() == 1 {
            let pattern = possible_patterns.first().unwrap();
            let pattern_len = pattern.len();

            if design[0..pattern_len] != ***pattern {
                return false;
            }

            design = &design[pattern_len..];
        } else {
            for pattern in possible_patterns {
                let pattern_len = pattern.len();

                if design[0..pattern_len] != **pattern {
                    continue;
                }

                if try_design(&design[pattern_len..], available_patterns) {
                    return true;
                }
            }
            return false;
        }
    }

    true
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let available_patterns: Vec<_> = input
        .split("\n\n")
        .next()
        .unwrap()
        .split(", ")
        .map(|x| x.to_string())
        .collect();
    let designs: Vec<_> = input
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect();
    let max_pattern_len = available_patterns
        .iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .len();

    // for pattern in available_patterns.clone() {
    //     print!("{pattern}, ");
    // }
    // println!();

    let mut no_possible_patterns = 0;
    for design in designs {
        println!("{design}");
        let all_possible_patterns = find_all_patterns(design, max_pattern_len, &available_patterns);
        // find_all_patterns("asd".to_string(), max_pattern_len, &available_patterns);
        for possible_pattern in all_possible_patterns {
            println!("\t{}\t", possible_pattern.join(","));
            //     // let mut all_patterns_exist = true;
            //     // for i in possible_pattern {
            //     //     if !available_patterns.contains(&i) {
            //     //         // println!("\n\t{i} doesn't exist!");
            //     //         all_patterns_exist = false;
            //     //         break;
            //     //     }
            //     // }
            //     // if all_patterns_exist {
            //     //     print!("works!");
            no_possible_patterns += 1;
            //     // }
            //     // println!()
        }
    }
    println!("{}", no_possible_patterns);
}

fn find_all_patterns(
    design: String,
    max_pattern_len: usize,
    available_patterns: &[String],
) -> Vec<Vec<String>> {
    if design.is_empty() {
        return vec![];
    }

    let mut all_possible_patterns = vec![];
    for i in 1..=max_pattern_len.min(design.len()) {
        if !available_patterns.contains(&design[0..i].to_string()) {
            continue;
        }

        if design[i..].is_empty() {
            all_possible_patterns.push(vec![design[0..i].to_string()]);
            continue;
        }
        let recursive_patterns =
            find_all_patterns(design[i..].to_string(), max_pattern_len, available_patterns);
        for possible_pattern in recursive_patterns {
            let mut pattern = vec![design[0..i].to_string()];
            pattern.extend(possible_pattern);
            all_possible_patterns.push(pattern);
        }
    }

    all_possible_patterns
}

// pub fn part_two() {
//     let input = read_to_string("./src/input").unwrap();

//     let available_patterns: Vec<_> = input
//         .split("\n\n")
//         .next()
//         .unwrap()
//         .split(", ")
//         .map(|x| x.to_string())
//         .collect();
//     let designs: Vec<_> = input
//         .split("\n\n")
//         .last()
//         .unwrap()
//         .lines()
//         .map(|x| x.to_string())
//         .collect();

//     println!("Starting...");
//     let possible_designs = Arc::new(Mutex::new(0));
//     let num_threads = 1;
//     let mut handles = vec![];
//     for j in 0..num_threads {
//         let cloned_possible_designs = Arc::clone(&possible_designs);
//         let cloned_designs = designs.clone();
//         let cloned_available_patterns = available_patterns.clone();
//         let min = 0;
//         let max = cloned_designs.len();
//         let diff = max - min;
//         let part = diff / num_threads;

//         let handle = thread::spawn(move || {
//             let mut possible_designs = 0;
//             // println!(
//             //     "{}: {} {} {},{}",
//             //     j,
//             //     part,
//             //     max,
//             //     (min + part * j),
//             //     (min + part * (j + 1))
//             // );
//             for i in (min + part * j)..(min + part * (j + 1)) {
//                 let design = cloned_designs[i].clone();
//                 // possible_designs += try_design2(design, &cloned_available_patterns);
//                 let designs = try_design2(0, design, &cloned_available_patterns);
//                 possible_designs += designs;
//                 println!(
//                     "{}/{} => {}:\t{}",
//                     i,
//                     cloned_designs.len(),
//                     cloned_designs[i],
//                     designs
//                 );
//             }
//             *cloned_possible_designs.lock().unwrap() += possible_designs;
//         });

//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.join();
//     }
//     // for design in designs {
//     //     // }
//     // }

//     println!("{}", possible_designs.lock().unwrap());
// }

// fn try_design2(depth: usize, d: String, available_patterns: &[String]) -> u32 {
//     let mut no_possible_designs = 0;
//     let mut design = d;
//     // println!("{}design: {design}:", "\t".repeat(depth));
//     while !design.is_empty() {
//         let test_char = design.chars().next().unwrap();
//         let possible_patterns: Vec<_> = available_patterns
//             .iter()
//             .filter(|x| x.len() <= design.len() && x.chars().next().unwrap() == test_char)
//             .collect();

//         if possible_patterns.is_empty() {
//             return 0;
//         } else if possible_patterns.len() == 1 {
//             let pattern = possible_patterns.first().unwrap();
//             let pattern_len = pattern.len();

//             if design[0..pattern_len] != **pattern {
//                 // println!("nuh uh!");
//                 return 0;
//             }

//             design = design[pattern_len..].to_string();
//             // println!(
//             //     "{}\tpattern: {pattern}:\tyuh uh => {design}",
//             //     "\t".repeat(depth)
//             // );
//         } else {
//             let mut to_try = vec![];
//             for pattern in possible_patterns.iter() {
//                 let pattern_len = pattern.len();

//                 // print!("{}\t{pattern}:\t", "\t".repeat(depth));
//                 if design[0..pattern_len] != **pattern {
//                     // println!("nuh uh!");
//                     continue;
//                 }
//                 // println!(
//                 //     "{}\tpattern: {pattern}:\tyuh uh => {}",
//                 //     "\t".repeat(depth),
//                 //     &design[pattern_len..]
//                 // );
//                 to_try.push(design[pattern_len..].to_string());
//             }

//             for new_design in to_try {
//                 if new_design.is_empty() {
//                     no_possible_designs += 1;
//                 } else {
//                     no_possible_designs += try_design2(depth + 1, new_design, available_patterns);
//                 }
//                 // println!(
//                 //     "{}\ttotal designs: => {no_possible_designs}",
//                 //     "\t".repeat(depth)
//                 // );
//             }
//             return no_possible_designs;
//         }
//     }

//     1
// }
