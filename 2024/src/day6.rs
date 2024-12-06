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
    // println!("{}, {}", guard_pos.0, guard_pos.1);
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
        /*println!(
            "{}, {} -> {} -> {}",
            guard_pos.0,
            guard_pos.1,
            facing,
            visited_cells.len()
        );*/
    }
    println!("{}", visited_cells.len());
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let maze: Vec<Vec<String>> = input
        .lines()
        .map(|x| x.chars().map(|x| x.to_string()).collect())
        .collect();

    /*
    let mut guard_pos = starting_pos;
    let mut possible_endless_loops: Vec<(usize, usize)> = vec![];
    let mut facing: u32 = 2;
    let mut visited_cells: Vec<(u32, (usize, usize))> = vec![(facing, guard_pos)];
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

        let rotate = |f: &u32| -> u32 {
            let mut facing = *f;
            facing *= 2;
            if facing > 8 {
                facing = 1;
            }
            facing
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
            facing = rotate(&facing);
            new_pos = inc_guard_pos(&guard_pos, facing);
        }

        'infinite_loops: {
            // if new_pos == starting_pos {
            //     break 'infinite_loops;
            // }
            let rotated_facing = rotate(&facing);
            match facing {
                1 => 'one: {
                    // ()
                    let mut perpendicular_routs = visited_cells
                        .iter()
                        .filter(|x| x.0 == rotated_facing && x.1 .0 == new_pos.0)
                        .collect::<Vec<_>>();
                    if perpendicular_routs.is_empty() {
                        /*println!(
                            "{} = ({}, {}) -> {}",
                            facing, new_pos.0, new_pos.1, rotated_facing,
                        );*/
                        break 'one;
                    }
                    perpendicular_routs.sort_by(|a, b| {
                        a.1 .1
                            .abs_diff(new_pos.1)
                            .partial_cmp(&b.1 .1.abs_diff(new_pos.1))
                            .unwrap()
                    });
                    let closest_route = perpendicular_routs[0];
                    /*println!(
                        "{} = ({}, {}) -> {} = ({}, {})",
                        facing,
                        new_pos.0,
                        new_pos.1,
                        rotated_facing,
                        closest_route.1 .0,
                        closest_route.1 .1
                    ));*/

                    let mut blocked = false;
                    for i in closest_route.1 .1..=new_pos.1 {
                        if maze[i][new_pos.0] == "#" {
                            blocked = true;
                            break;
                        }
                    }
                    if blocked {
                        break 'one;
                    }

                    possible_endless_loops.push(inc_guard_pos(&new_pos, facing));
                }
                2 => {
                    // ()
                    'circle: {
                        let mut perpendicular_routs = visited_cells
                            .iter()
                            .filter(|x| x.0 == rotated_facing && x.1 .1 == new_pos.1)
                            .collect::<Vec<_>>();
                        if perpendicular_routs.is_empty() {
                            /*println!(
                                "{} = ({}, {}) -> {}",
                                facing, new_pos.0, new_pos.1, rotated_facing,
                            );*/
                            break 'circle;
                        }
                        perpendicular_routs.sort_by(|a, b| {
                            a.1 .0
                                .abs_diff(new_pos.0)
                                .partial_cmp(&b.1 .0.abs_diff(new_pos.0))
                                .unwrap()
                        });
                        let closest_route = perpendicular_routs[0];
                        /*println!(
                            "{} = ({}, {}) -> {} = ({}, {})",
                            facing,
                            new_pos.0,
                            new_pos.1,
                            rotated_facing,
                            closest_route.1 .0,
                            closest_route.1 .1
                        );*/

                        let mut blocked = false;
                        for i in new_pos.0..=closest_route.1 .0 {
                            if maze[new_pos.1][i] == "#" {
                                blocked = true;
                                break;
                            }
                        }
                        if !blocked {
                            possible_endless_loops.push(inc_guard_pos(&new_pos, facing));
                            break 'circle;
                        }
                    }

                    'line: {
                        /*println!(
                            "{} = ({}, {}) -> {}",
                            facing, new_pos.0, new_pos.1, rotated_facing,
                        );*/
                        if new_pos.0 == 0 || new_pos.0 >= maze[0].len() - 1 {
                            break 'line;
                        }
                        if maze[new_pos.1][new_pos.0 + 1] != "#" {
                            break 'line;
                        }

                        let rotated_facing = rotate(&rotated_facing);
                        let mut reverse_routs = visited_cells
                            .iter()
                            .filter(|x| {
                                x.0 == rotated_facing && x.1 .0 == new_pos.0 && x.1 .1 >= new_pos.1
                            })
                            .collect::<Vec<_>>();
                        if reverse_routs.is_empty() {
                            /*println!(
                                "{} = ({}, {}) -> {}",
                                facing, new_pos.0, new_pos.1, rotated_facing,
                            );*/
                            break 'line;
                        }
                        reverse_routs.sort_by(|a, b| {
                            a.1 .1
                                .abs_diff(new_pos.1)
                                .partial_cmp(&b.1 .1.abs_diff(new_pos.1))
                                .unwrap()
                        });
                        let closest_route = reverse_routs[0];
                        /*println!(
                            "{} = ({}, {}) -> {} = ({}, {})",
                            facing,
                            new_pos.0,
                            new_pos.1,
                            rotated_facing,
                            closest_route.1 .0,
                            closest_route.1 .1
                        );*/

                        let mut blocked = false;
                        for i in new_pos.1..=closest_route.1 .1 {
                            if maze[i][new_pos.0] == "#" {
                                blocked = true;
                                break;
                            }
                        }
                        if !blocked {
                            possible_endless_loops.push(inc_guard_pos(&new_pos, facing));
                        }
                    }
                }
                4 => 'four: {
                    // ()
                    let mut perpendicular_routs = visited_cells
                        .iter()
                        .filter(|x| x.0 == rotated_facing && x.1 .0 == new_pos.0)
                        .collect::<Vec<_>>();
                    if perpendicular_routs.is_empty() {
                        /*println!(
                            "{} = ({}, {}) -> {}",
                            facing, new_pos.0, new_pos.1, rotated_facing,
                        );*/
                        break 'four;
                    }
                    perpendicular_routs.sort_by(|a, b| {
                        a.1 .1
                            .abs_diff(new_pos.1)
                            .partial_cmp(&b.1 .1.abs_diff(new_pos.1))
                            .unwrap()
                    });
                    let closest_route = perpendicular_routs[0];
                    /*println!(
                        "{} = ({}, {}) -> {} = ({}, {})",
                        facing,
                        new_pos.0,
                        new_pos.1,
                        rotated_facing,
                        closest_route.1 .0,
                        closest_route.1 .1
                    );*/

                    let mut blocked = false;
                    for i in new_pos.1..=closest_route.1 .1 {
                        if maze[i][new_pos.0] == "#" {
                            blocked = true;
                            break;
                        }
                    }
                    if blocked {
                        break 'four;
                    }

                    possible_endless_loops.push(inc_guard_pos(&new_pos, facing));
                }
                8 => {
                    // ()
                    'circle: {
                        let mut perpendicular_routs = visited_cells
                            .iter()
                            .filter(|x| x.0 == rotated_facing && x.1 .1 == new_pos.1)
                            .collect::<Vec<_>>();
                        if perpendicular_routs.is_empty() {
                            /*println!(
                                "{} = ({}, {}) -> {}",
                                facing, new_pos.0, new_pos.1, rotated_facing,
                            );*/
                            break 'circle;
                        }
                        perpendicular_routs.sort_by(|a, b| {
                            a.1 .0
                                .abs_diff(new_pos.0)
                                .partial_cmp(&b.1 .0.abs_diff(new_pos.0))
                                .unwrap()
                        });
                        let closest_route = perpendicular_routs[0];
                        /*println!(
                            "{} = ({}, {}) -> {} = ({}, {})",
                            facing,
                            new_pos.0,
                            new_pos.1,
                            rotated_facing,
                            closest_route.1 .0,
                            closest_route.1 .1
                        );*/

                        let mut blocked = false;
                        for i in closest_route.1 .0..=new_pos.0 {
                            if maze[new_pos.1][i] == "#" {
                                blocked = true;
                                break;
                            }
                        }
                        if !blocked {
                            possible_endless_loops.push(inc_guard_pos(&new_pos, facing));
                            break 'circle;
                        }
                    }

                    'line: {
                        /*println!(
                            "{} = ({}, {}) -> {}",
                            facing, new_pos.0, new_pos.1, rotated_facing,
                        );*/
                        if new_pos.0 == 0 || new_pos.0 >= maze[0].len() - 1 {
                            break 'line;
                        }
                        if maze[new_pos.1][new_pos.0 - 1] != "#" {
                            break 'line;
                        }

                        let rotated_facing = rotate(&rotated_facing);
                        let mut reverse_routs = visited_cells
                            .iter()
                            .filter(|x| {
                                x.0 == rotated_facing && x.1 .0 == new_pos.0 && x.1 .1 <= new_pos.1
                            })
                            .collect::<Vec<_>>();
                        if reverse_routs.is_empty() {
                            /*println!(
                                "{} = ({}, {}) -> {}",
                                facing, new_pos.0, new_pos.1, rotated_facing,
                            );*/
                            break 'line;
                        }
                        reverse_routs.sort_by(|a, b| {
                            a.1 .1
                                .abs_diff(new_pos.1)
                                .partial_cmp(&b.1 .1.abs_diff(new_pos.1))
                                .unwrap()
                        });
                        let closest_route = reverse_routs[0];
                        /*println!(
                            "{} = ({}, {}) -> {} = ({}, {})",
                            facing,
                            new_pos.0,
                            new_pos.1,
                            rotated_facing,
                            closest_route.1 .0,
                            closest_route.1 .1
                        );*/

                        let mut blocked = false;
                        for i in closest_route.1 .1..=new_pos.1 {
                            if maze[i][new_pos.0] == "#" {
                                blocked = true;
                                break;
                            }
                        }
                        if !blocked {
                            possible_endless_loops.push(inc_guard_pos(&new_pos, facing));
                        }
                    }
                }
                _ => (),
            }
        }

        guard_pos = new_pos;
        visited_cells.push((facing, guard_pos));
        /*println!(
            "({}, {}) -> {} -> {}",
            guard_pos.0,
            guard_pos.1,
            facing,
            visited_cells.len()
        );*/
    }

    println!("{}", possible_endless_loops.len());
    // for i in possible_endless_loops {
    //     println!("({}, {})", i.0, i.1);
    // }
    */

    // brute-force since the other method didn't work 😤
    let starting_pos: (_, _) = (
        (maze.iter().flatten().position(|x| x == "^").unwrap() % maze[0].len()) as usize,
        (maze.iter().flatten().position(|x| x == "^").unwrap() as f32 / maze[0].len() as f32)
            .floor() as usize,
    );
    let mut guard_pos = starting_pos;
    // let mut mazes: Vec<((usize, usize), u32, Vec<Vec<String>>)> = vec![];
    let mut infinite_loop_cells: Vec<(usize, usize)> = vec![];
    let mut visited_cells: HashSet<(usize, usize)> = HashSet::from([guard_pos]);
    // let infinite_loops_count = Arc::new(Mutex::new(0));
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
            // mazes.push((guard_pos, facing, new_maze));
            if check_loop(&new_maze, guard_pos, facing) {
                infinite_loop_cells.push(new_pos);
                // println!("{}", infinite_loop_cells.len());
            }
        }

        guard_pos = new_pos;
        facing = new_facing;
        /*println!(
            "{}, {} -> {} -> {}",
            guard_pos.0,
            guard_pos.1,
            facing,
            visited_cells.len()
        );*/
    }

    // println!("{}", mazes.len());
    // // let sixteenth: u32 = mazes.len() as u32 / 32;
    // // for j in 0..33 {
    // let mut handles = vec![];
    // let num_finished = Arc::new(Mutex::new(0));
    // // for i in (j * sixteenth)..((j + 1) * sixteenth) {
    // for maze in mazes {
    //     // if i as usize > mazes.len() {
    //     //     break;
    //     // }
    //     // let maze = mazes[i as usize].clone();
    //     let counter = Arc::clone(&infinite_loops_count);
    //     let counter2 = Arc::clone(&num_finished);
    //     let handle = thread::spawn(move || {
    //         if check_loop(&maze.2, maze.0, maze.1) {
    //             *counter.lock().unwrap() += 1;
    //         } else {
    //             *counter2.lock().unwrap() += 1;
    //             println!("{}", *counter2.lock().unwrap());
    //         }
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // // }

    // println!("{}", *infinite_loops_count.lock().unwrap());
    println!("{}", infinite_loop_cells.len());
    // for i in infinite_loop_cells {
    //     println!("({}, {})", i.0, i.1);
    // }
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
            // println!("out of bounds");
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
                // println!("out of bounds");
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
