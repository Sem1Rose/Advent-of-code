use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();
    let codes = input.lines().collect::<Vec<_>>();

    let locked_door_keypad: HashMap<char, (i32, i32)> = HashMap::from_iter([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]);

    let robots_keypad: HashMap<char, (i32, i32)> = HashMap::from_iter([
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    let locked_door_bot_pos = locked_door_keypad[&'A'];
    let bot_pos = robots_keypad[&'A'];

    let mut complexities = 0;
    for code in codes {
        let first_sequences = get_permutations(get_bot_movements(
            code,
            &locked_door_keypad,
            locked_door_bot_pos,
        ));

        let mut filtered_sequences = vec![];
        for i in first_sequences {
            if test_sequence(&i, &locked_door_keypad, locked_door_bot_pos) {
                filtered_sequences.push(i);
            }
        }

        println!("first sequences: {}", filtered_sequences.len());
        for i in filtered_sequences.clone() {
            println!("\t{}", i);
        }
        println!();

        let mut second_sequences = vec![];
        for seq in filtered_sequences {
            let sequence = get_permutations(get_bot_movements(&seq, &robots_keypad, bot_pos));

            for i in sequence {
                if test_sequence(&i, &robots_keypad, bot_pos) {
                    second_sequences.push(i);
                }
            }
        }

        println!("second sequences: {}", second_sequences.len());
        for i in second_sequences.clone() {
            println!("\t{}", i);
        }
        println!();

        let mut third_sequences = vec![];
        for seq in second_sequences {
            let sequence = get_permutations(get_bot_movements(&seq, &robots_keypad, bot_pos));

            for i in sequence {
                if test_sequence(&i, &robots_keypad, bot_pos) {
                    third_sequences.push(i);
                }
            }
        }

        println!("third sequences: {}", third_sequences.len());
        for i in third_sequences.clone() {
            println!("\t{}", i);
        }
        println!();

        let shortest_sequence = third_sequences
            .iter()
            .min_by(|x, y| x.len().cmp(&y.len()))
            .unwrap()
            .len();

        // println!(
        //     "{}\n{}\n{}\n{}",
        //     get_bot_movements(
        //         get_bot_movements(
        //             get_bot_movements(code, &locked_door_keypad, locked_door_bot_pos).as_str(),
        //             &robots_keypad,
        //             first_bot_pos
        //         )
        //         .as_str(),
        //         &robots_keypad,
        //         second_bot_pos
        //     ),
        //     get_bot_movements(
        //         get_bot_movements(code, &locked_door_keypad, locked_door_bot_pos).as_str(),
        //         &robots_keypad,
        //         first_bot_pos
        //     ),
        //     get_bot_movements(code, &locked_door_keypad, locked_door_bot_pos),
        //     code,
        // );

        // println!(
        //     "{}, {}",
        //     sequence.len(),
        //     code.chars()
        //         .filter(|x| x.is_ascii_digit())
        //         .fold(String::new(), |acc, x| format!("{}{}", acc, x))
        // );

        complexities += shortest_sequence
            * code
                .chars()
                .filter(|x| x.is_ascii_digit())
                .fold(String::new(), |acc, x| format!("{}{}", acc, x))
                .parse::<usize>()
                .unwrap();
        println!();
    }

    println!("{}", complexities);
    // let perms = get_permutations("v<>^Av<>^AAA".to_string());
    // println!(
    //     "v<>^Av<>^AAA, >{}, <{}, v{}, ^{} -> {}",
    //     "v<>^Av<>^AAA".chars().filter(|x| *x == '>').count(),
    //     "v<>^Av<>^AAA".chars().filter(|x| *x == '<').count(),
    //     "v<>^Av<>^AAA".chars().filter(|x| *x == 'v').count(),
    //     "v<>^Av<>^AAA".chars().filter(|x| *x == '^').count(),
    //     perms.len()
    // );
    // for perm in perms {
    //     println!(
    //         "{perm}, {}, {}, {}, {}",
    //         perm.chars().filter(|x| *x == '>').count(),
    //         perm.chars().filter(|x| *x == '<').count(),
    //         perm.chars().filter(|x| *x == 'v').count(),
    //         perm.chars().filter(|x| *x == '^').count(),
    //     );
    // }
}

fn get_bot_movements(
    code: &str,
    keypad_hashmap: &HashMap<char, (i32, i32)>,
    pos: (i32, i32),
) -> String {
    let signed_distance = |p: (i32, i32), d: (i32, i32)| -> (i32, i32) { (p.0 - d.0, p.1 - d.1) };

    let mut sequence = String::new();
    let mut position = pos;
    for press in code.chars() {
        let destination = keypad_hashmap[&press];

        let (horiz, vert) = signed_distance(destination, position);
        let horiz_sign = horiz > 0;
        let vert_sign = vert > 0;

        if vert.abs() > horiz.abs() {
            sequence = format!(
                "{}{}{}{}A",
                sequence,
                if vert_sign { "v" } else { "^" }.repeat(if vert == 0 {
                    0
                // } else if vert.abs() == 1 {
                //     vert.abs() as usize
                } else {
                    (vert.abs() - 1).unsigned_abs() as usize
                }),
                if horiz_sign { ">" } else { "<" }.repeat(horiz.unsigned_abs() as usize),
                if vert_sign { "v" } else { "^" }.repeat(if vert == 0 { 0 } else { 1 })
            );
        } else {
            sequence = format!(
                "{}{}{}{}A",
                sequence,
                if horiz_sign { ">" } else { "<" }.repeat(if horiz == 0 {
                    0
                // } else if horiz.abs() == 1 {
                //     horiz.abs() as usize
                } else {
                    (horiz.abs() - 1).unsigned_abs() as usize
                }),
                if vert_sign { "v" } else { "^" }.repeat(vert.unsigned_abs() as usize),
                if horiz_sign { ">" } else { "<" }.repeat(if horiz == 0 { 0 } else { 1 })
            );
        }
        position = destination;
    }

    sequence
}

fn get_permutations(sequence: String) -> Vec<String> {
    let mut permutations: Vec<Vec<String>> = vec![];

    // let mut permutations = vec![];
    for seq in sequence.split('A') {
        if seq.is_empty() {
            let mut new_permutations = vec![];
            for s in permutations.clone() {
                let mut new_s = s.clone();
                new_s.push("".to_string());
                new_permutations.push(new_s);
            }
            // out = new_out;
            // permutations.extend(permutation.drain());
            permutations = new_permutations;
            continue;
        }

        if (seq.contains('v') || seq.contains('^')) && (seq.contains('>') || seq.contains('<')) {
            let mut permutation = HashSet::new();
            permute_str(&mut permutation, seq.chars().collect(), 0);

            if permutations.is_empty() {
                for perm in permutation {
                    permutations.push(vec![perm]);
                }
                continue;
            }
            let mut new_permutations = vec![];
            for s in permutations.clone() {
                for perm in permutation.clone() {
                    let mut new_s = s.clone();
                    new_s.push(perm);
                    new_permutations.push(new_s);
                }
            }
            // out = new_out;
            // permutations.extend(permutation.drain());
            permutations = new_permutations;
        } else {
            if permutations.is_empty() {
                permutations.push(vec![seq.to_string()]);
                continue;
            }

            let mut new_permutations = vec![];
            for s in permutations.clone() {
                // new_out.push();
                let mut new_s = s.clone();
                new_s.push(seq.to_string());
                new_permutations.push(new_s);
            }
            permutations = new_permutations;
            // permutations.push(seq.to_string());
        }
    }

    let mut out = HashSet::new();
    for perm in permutations {
        out.insert(perm.join("A"));
    }

    out.drain().collect()
}

fn permute_str(hashset: &mut HashSet<String>, s: Vec<char>, idx: usize) {
    if idx == s.len() - 1 {
        hashset.insert(String::from_iter(s));
        return;
    }

    for i in idx..s.len() {
        let mut swapped = s.clone();
        swapped.swap(idx, i);
        permute_str(hashset, swapped, idx + 1);
    }
}

fn test_sequence(
    sequence: &str,
    keypad_hashmap: &HashMap<char, (i32, i32)>,
    pos: (i32, i32),
) -> bool {
    let mut position = pos;
    let add_pos = |a: (i32, i32), b: (i32, i32)| -> (i32, i32) { (a.0 + b.0, a.1 + b.1) };
    let get_dir = |c: char| -> (i32, i32) {
        match c {
            'v' => (0, 1),
            '>' => (1, 0),
            '^' => (0, -1),
            '<' => (-1, 0),
            _ => (0, 0),
        }
    };

    for c in sequence.chars() {
        let new_pos = add_pos(position, get_dir(c));
        if !keypad_hashmap.iter().any(|x| *x.1 == new_pos) {
            return false;
        }
        position = new_pos;
    }

    true
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();
}
