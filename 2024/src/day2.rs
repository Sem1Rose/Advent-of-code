use std::fs::read_to_string;

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|x| x.split(' ').map(|y| y.parse().expect(y)).collect())
        .collect();

    let mut safe_reports_count = 0;
    for report in reports {
        let mut increasing = false;
        let mut not_safe = false;
        if report[1] > report[0] {
            increasing = true;
        }

        let mut report = report.iter();
        let first_item = report.next().unwrap();
        report.fold(first_item, |acc, x| {
            if (x - acc > 0) != increasing || x.abs_diff(*acc) > 3 || x.abs_diff(*acc) < 1 {
                not_safe = true;
            }
            x
        });

        if !not_safe {
            safe_reports_count += 1;
        }
    }

    println!("{}", safe_reports_count);
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
