use std::{collections::HashSet, fs::read_to_string};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let replacement_dictionary: Vec<(&str, &str)> = input
        .lines()
        .filter(|x| x.contains("=>"))
        .map(|x| {
            (
                x.split(" => ").collect::<Vec<&str>>()[0],
                x.split(" => ").collect::<Vec<&str>>()[1],
            )
        })
        .collect();
    let molecule = input
        .lines()
        .filter(|x| !x.contains("=>"))
        .collect::<Vec<&str>>()[0];
    let mut generated_molecules: Vec<String> = vec![];

    for (pattern, replacement) in replacement_dictionary {
        let molecule_clone = molecule.to_string();
        let matches = molecule.match_indices(pattern);

        for (index, mat) in matches {
            let mut x = molecule_clone
                .chars()
                .map(|z| z.to_string())
                .collect::<Vec<String>>();
            for _ in 0..mat.chars().count() {
                x.remove(index);
            }

            for i in replacement.chars().rev() {
                x.insert(index, i.to_string());
            }

            generated_molecules.push(x.concat());
        }
    }

    let set: HashSet<_> = generated_molecules.drain(..).collect(); // dedup
    println!("{}", set.len());
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|x| x.split(' ').map(|y| y.parse().expect(y)).collect())
        .collect();

    fn count_errors(report: &[i32]) -> u32 {
        let mut errors = 0;

        let mut truncated = report.iter();
        let first_item = truncated.next().unwrap();
        let increasing = truncated
            .clone()
            .fold((0, first_item), |(acc, last), x| {
                (
                    match x.cmp(last) {
                        std::cmp::Ordering::Less => acc - 1,
                        std::cmp::Ordering::Equal => acc,
                        std::cmp::Ordering::Greater => acc + 1,
                    },
                    x,
                )
            })
            .0
            >= 0;

        truncated.fold(first_item, |acc, x| {
            if x == acc || (x - acc > 0) != increasing || x.abs_diff(*acc) > 3 {
                errors += 1;
            }
            x
        });

        errors
    }

    let mut safe_reports_count = 0;
    for report in reports {
        if count_errors(&report) == 0 {
            safe_reports_count += 1;
            continue;
        }

        for i in 0..report.len() {
            let mut cloned = report.clone();
            cloned.remove(i);
            if count_errors(&cloned) == 0 {
                safe_reports_count += 1;
                break;
            }
        }
    }

    println!("{}", safe_reports_count);
}
