use std::{collections::HashSet, fs::read_to_string};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let maze: Vec<Vec<String>> = input
        .lines()
        .map(|x| x.chars().map(|x| x.to_string()).collect())
        .collect();

    let mut guard_pos: (_, _) = (
        (maze.iter().flatten().position(|x| x == "^").unwrap() % maze[0].len()) as usize,
        (maze.iter().flatten().position(|x| x == "^").unwrap() as f32 / maze[0].len() as f32)
            .floor() as usize,
    );
    let mut visited_cells: HashSet<(usize, usize)> = HashSet::from([guard_pos]);
    let mut facing: u32 = 2;
    loop {
        let inc_guard_pos = |pos: &(usize, usize), facing: u32| -> (usize, usize) {
            let mut new_pos = *pos;
            match facing {
                1 => new_pos.0 -= 1,
                2 => new_pos.1 -= 1,
                4 => new_pos.0 += 1,
                8 => new_pos.1 += 1,
                _ => panic!("facing was {facing}"),
            }
            new_pos
        };

        if guard_pos.0 == 0
            || guard_pos.0 == maze[0].len() - 1
            || guard_pos.1 == 0
            || guard_pos.1 == maze.len() - 1
        {
            break;
        }

        let mut new_pos = inc_guard_pos(&guard_pos, facing);
        while maze[new_pos.1][new_pos.0] == "#" {
            facing *= 2;
            if facing > 8 {
                facing = 1;
            }
            new_pos = inc_guard_pos(&guard_pos, facing);
        }

        guard_pos = new_pos;
        visited_cells.insert(guard_pos);
    }
    println!("{}", visited_cells.len());
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let maze: Vec<Vec<String>> = input
        .lines()
        .map(|x| x.chars().map(|x| x.to_string()).collect())
        .collect();

    let starting_pos: (_, _) = (
        (maze.iter().flatten().position(|x| x == "^").unwrap() % maze[0].len()) as usize,
        (maze.iter().flatten().position(|x| x == "^").unwrap() as f32 / maze[0].len() as f32)
            .floor() as usize,
    );
    let mut guard_pos = starting_pos;
    let mut infinite_loop_cells: Vec<(usize, usize)> = vec![];
    let mut visited_cells: HashSet<(usize, usize)> = HashSet::from([guard_pos]);
    let mut facing: u32 = 2;
    'main_loop: loop {
        let inc_guard_pos = |pos: &(usize, usize), facing: u32| -> Option<(usize, usize)> {
            let mut new_pos = *pos;
            match facing {
                1 => {
                    if new_pos.0 == 0 {
                        return None;
                    }
                    new_pos.0 -= 1;
                }
                2 => {
                    if new_pos.1 == 0 {
                        return None;
                    }
                    new_pos.1 -= 1;
                }
                4 => {
                    if new_pos.0 == maze[0].len() - 1 {
                        return None;
                    }
                    new_pos.0 += 1;
                }
                8 => {
                    if new_pos.1 == maze.len() - 1 {
                        return None;
                    }
                    new_pos.1 += 1;
                }
                _ => panic!("facing was {facing}"),
            }
            Some(new_pos)
        };

        let mut new_facing = facing;
        let new_pos = inc_guard_pos(&guard_pos, new_facing);
        if new_pos.is_none() {
            break;
        }
        let mut new_pos = new_pos.unwrap();
        while maze[new_pos.1][new_pos.0] == "#" {
            new_facing *= 2;
            if new_facing > 8 {
                new_facing = 1;
            }
            let n_pos = inc_guard_pos(&guard_pos, new_facing);
            if n_pos.is_none() {
                break 'main_loop;
            }
            new_pos = n_pos.unwrap();
        }

        if visited_cells.insert(new_pos) {
            let mut new_maze = maze.clone();
            new_maze[new_pos.1][new_pos.0] = "#".to_string();
            if check_loop(&new_maze, guard_pos, facing) {
                infinite_loop_cells.push(new_pos);
            }
        }

        guard_pos = new_pos;
        facing = new_facing;
    }

    println!("{}", infinite_loop_cells.len());
}

fn check_loop(maze: &[Vec<String>], starting_pos: (usize, usize), starting_facing: u32) -> bool {
    let mut facing = starting_facing;
    let mut guard_pos = starting_pos;
    let mut path = HashSet::from([(starting_facing, starting_pos)]);
    loop {
        let inc_guard_pos = |pos: &(usize, usize), facing: u32| -> Option<(usize, usize)> {
            let mut new_pos = *pos;
            match facing {
                1 => {
                    if new_pos.0 == 0 {
                        return None;
                    }
                    new_pos.0 -= 1;
                }
                2 => {
                    if new_pos.1 == 0 {
                        return None;
                    }
                    new_pos.1 -= 1;
                }
                4 => {
                    if new_pos.0 == maze[0].len() - 1 {
                        return None;
                    }
                    new_pos.0 += 1;
                }
                8 => {
                    if new_pos.1 == maze.len() - 1 {
                        return None;
                    }
                    new_pos.1 += 1;
                }
                _ => panic!("facing was {facing}"),
            }
            Some(new_pos)
        };

        let new_pos = inc_guard_pos(&guard_pos, facing);
        if new_pos.is_none() {
            return false;
        }
        let mut new_pos = new_pos.unwrap();
        while maze[new_pos.1][new_pos.0] == "#" {
            facing *= 2;
            if facing > 8 {
                facing = 1;
            }
            let n_pos = inc_guard_pos(&guard_pos, facing);
            if n_pos.is_none() {
                return false;
            }
            new_pos = n_pos.unwrap();
        }

        guard_pos = new_pos;
        if !path.insert((facing, guard_pos)) {
            return true;
        }
    }
}
