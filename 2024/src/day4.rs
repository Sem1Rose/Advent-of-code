use regex::Regex;
use std::fs::read_to_string;

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let puzzle: Vec<_> = input
        .lines()
        .map(|x| x.chars().map(|y| y.to_string()).collect::<Vec<_>>())
        .collect();
    let bottom = rotate_cw(&puzzle);
    let top_left_diagonal = get_diagonal(&puzzle);
    let bottom_right_diagonal = get_diagonal(&bottom);

    let get_num_matches = |vector: &[Vec<String>]| -> u32 {
        let mut num_matches = 0;
        let re = Regex::new(r"XMAS").unwrap();

        // println!();
        for i in vector {
            // println!("{} {}", &i.concat(), re.captures_iter(&i.concat()).count());
            num_matches += re.captures_iter(&i.concat()).count();
        }
        // println!();
        for i in vector {
            //     println!(
            //         "{} {}",
            //         &reverse(i.clone()).concat(),
            //         re.captures_iter(&reverse(i.clone()).concat()).count()
            //     );
            num_matches += re.captures_iter(&reverse(i.clone()).concat()).count();
        }

        num_matches as u32
    };

    println!(
        "{}",
        get_num_matches(&puzzle)
            + get_num_matches(&bottom)
            + get_num_matches(&top_left_diagonal)
            + get_num_matches(&bottom_right_diagonal)
    );
}

fn reverse<T>(mut vector: Vec<T>) -> Vec<T> {
    vector.reverse();
    vector
}

fn rotate_cw<T: Clone>(vector: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut rotated: Vec<Vec<T>> = vec![];

    for i in 0..vector.len() {
        rotated.push(vec![]);
        for j in 0..vector[0].len() {
            rotated[i].push(vector[vector.len() - j - 1][i].clone());
        }
    }

    rotated
}

fn get_diagonal<T: Clone + std::fmt::Display>(vector: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut diagonalized: Vec<Vec<T>> = vec![];

    for i in 0..vector.len() {
        diagonalized.push(vec![]);
        for j in 0..=i {
            diagonalized[i].push(vector[i - j][j].clone());
        }
    }

    for j in 1..vector.len() {
        diagonalized.push(vec![]);
        for i in (j..vector.len()).rev() {
            diagonalized[vector.len() + j - 1].push(vector[i][j + vector.len() - i - 1].clone());
        }
    }

    diagonalized
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();
    let puzzle: Vec<_> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect();

    let mut x_mas_es_count = 0;
    for x in 0..4 {
        let first_char = if x & 1 == 0 { 'M' } else { 'S' };
        let second_char = if x & 2 == 0 { 'M' } else { 'S' };
        let third_char = if x & 2 == 0 { 'S' } else { 'M' };
        let fourth_char = if x & 1 == 0 { 'S' } else { 'M' };
        for i in 0..=(puzzle.len() - 3) {
            for j in 0..=(puzzle[0].len() - 3) {
                if puzzle[i][j] == first_char
                    && puzzle[i][j + 2] == second_char
                    && puzzle[i + 1][j + 1] == 'A'
                    && puzzle[i + 2][j] == third_char
                    && puzzle[i + 2][j + 2] == fourth_char
                {
                    x_mas_es_count += 1;
                }
            }
        }
    }

    println!("{}", x_mas_es_count);
}
