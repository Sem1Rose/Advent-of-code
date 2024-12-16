use std::{collections::HashMap, collections::HashSet, fs::read_to_string};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let maze: Vec<Vec<bool>> = input
        .lines()
        .rev()
        .map(|x| x.chars().map(|y| y != '#').collect())
        .collect();

    let mut nodes = vec![];
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            if maze[i][j] {
                nodes.push((j, i));
            }
        }
    }

    // let paths_costs = iterate_maze(&map, HashSet::new(), (1, 1), 0, 0);
    // for cost in paths_costs.clone() {
    //     println!("{cost}");
    // }
    // println!("{}", paths_costs.iter().min().unwrap());
    println!(
        "{}",
        dijkstra(&nodes, (1, 1), (maze[0].len() - 2, maze.len() - 2))
    );
}

fn dijkstra(nodes: &[(usize, usize)], source: (usize, usize), destination: (usize, usize)) -> u32 {
    let mut dists = HashMap::new();
    let mut prev = HashMap::new();
    let mut Q = vec![];

    for node in nodes {
        dists.insert(*node, u32::MAX);
        prev.insert(*node, None);
        Q.push(*node);
    }
    dists.entry(source).and_modify(|x| *x = 0).or_insert(0);

    let get_min_dist_node_index =
        |q: &[(usize, usize)], dist: &HashMap<(usize, usize), u32>| -> usize {
            let mut min_dist = u32::MAX;
            let mut min_dist_node = usize::MAX;

            for i in 0..q.len() {
                if dist[&q[i]] <= min_dist {
                    min_dist = dist[&q[i]];
                    min_dist_node = i;
                }
            }

            min_dist_node
        };
    let add_coords = |p: (usize, usize), d: (i32, i32)| -> (usize, usize) {
        ((p.0 as i32 + d.0) as usize, (p.1 as i32 + d.1) as usize)
    };
    let distance = |p: (usize, usize), d: (usize, usize)| -> (usize, usize) {
        (
            (p.0 as i32 - d.0 as i32).unsigned_abs() as usize,
            (p.1 as i32 - d.1 as i32).unsigned_abs() as usize,
        )
    };

    while !Q.is_empty() {
        let min_dist_node_index = get_min_dist_node_index(&Q, &dists);
        let node = Q[min_dist_node_index];
        Q.remove(min_dist_node_index);

        let mut neighbors = vec![];
        for offset in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let new_pos = add_coords(node, offset);
            if Q.contains(&new_pos) {
                neighbors.push(new_pos);
            }
        }

        for neighbor in neighbors {
            let mut edge_cost = 1;
            if prev[&node].is_some() {
                if (distance(node, prev[&node].unwrap()).0 == 0)
                    != (distance(node, neighbor).0 == 0)
                {
                    edge_cost += 1000;
                }
            } else if distance(node, neighbor).1 != 0 {
                edge_cost += 1000;
            }

            let alt = dists[&node] + edge_cost;
            if alt < dists[&neighbor] {
                dists.entry(neighbor).and_modify(|x| *x = alt);
                prev.entry(neighbor).and_modify(|x| *x = Some(node));
            }
        }
    }

    dists[&destination]
}

/* Could work but will take forever because it's testing every single route.
fn iterate_maze(
    maze: &[Vec<bool>],
    mut visited: HashSet<(usize, usize)>,
    pos: (usize, usize),
    dir: u32,
    cost: u32,
) -> Vec<u32> {
    let add_coords = |p: (usize, usize), d: (i32, i32)| -> (usize, usize) {
        ((p.0 as i32 + d.0) as usize, (p.1 as i32 + d.1) as usize)
    };
    let inc_dir = |d: u32, inc: i32| -> u32 {
        let mut new_d = d as i32 + inc;
        if new_d < 0 {
            new_d = 3;
        } else if new_d > 3 {
            new_d = 0;
        }
        new_d as u32
    };
    let get_dir_offset = |d: u32| -> (i32, i32) {
        match d {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => (0, 0),
        }
    };

    // let readable_dir = |d: u32| -> String {
    //     match d {
    //         0 => "Right".to_string(),
    //         1 => "Up".to_string(),
    //         2 => "Left".to_string(),
    //         3 => "Down".to_string(),
    //         _ => "WHAT THE FUCK".to_string(),
    //     }
    // };

    // println!("\n({}, {}), {}, {}", pos.0, pos.1, readable_dir(dir), cost);
    let mut position = pos;
    let mut direction = dir;
    let mut cost = cost;
    loop {
        // println!(
        //     "\t({}, {}), {}, {}",
        //     position.0,
        //     position.1,
        //     readable_dir(direction),
        //     cost
        // );
        if position == (maze[0].len() - 2, maze.len() - 2) {
            return vec![cost];
        }
        if !visited.insert(position) {
            return vec![u32::MAX];
        }

        // println!("\tpossible routes: ");
        let mut possible_routes = vec![];
        for i in -1..=1 {
            let new_dir = inc_dir(direction, i);
            let new_pos = add_coords(position, get_dir_offset(new_dir));
            if maze[new_pos.1][new_pos.0] {
                possible_routes.push((if i == 0 { 1 } else { 1001 }, new_dir, new_pos));
                // println!(
                //     "\t\t({}, {}), {}, {}",
                //     new_pos.0,
                //     new_pos.1,
                //     readable_dir(new_dir),
                //     if i == 0 { 1 } else { 1001 }
                // );
            }
        }

        possible_routes.sort();

        if possible_routes.is_empty() {
            return vec![u32::MAX];
        } else if possible_routes.len() == 1 {
            let (cost_add, new_dir, new_pos) = possible_routes.first().unwrap();
            position = *new_pos;
            direction = *new_dir;
            cost += cost_add;
        } else {
            let mut costs = vec![];
            for (cost_add, new_dir, new_pos) in possible_routes {
                costs.extend(iterate_maze(
                    maze,
                    visited.clone(),
                    new_pos,
                    new_dir,
                    cost + cost_add,
                ));
            }
            return costs;
        }
        // std::thread::sleep(std::time::Duration::from_millis(500));
    }
}*/

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let maze_debug: Vec<Vec<_>> = input
        .lines()
        .rev()
        .map(|x| x.chars().map(|y| y.to_string()).collect())
        .collect();
    let maze: Vec<Vec<bool>> = input
        .lines()
        .rev()
        .map(|x| x.chars().map(|y| y != '#').collect())
        .collect();

    let mut nodes = vec![];
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            if maze[i][j] {
                nodes.push((j, i));
            }
        }
    }

    // let paths_costs = iterate_maze(&map, HashSet::new(), (1, 1), 0, 0);
    // for cost in paths_costs.clone() {
    //     println!("{cost}");
    // }
    // println!("{}", paths_costs.iter().min().unwrap());
    let mut best_paths_tiles = HashSet::new();
    let best_path_costs = dijkstra2(
        &maze_debug,
        &nodes,
        (1, 1),
        (maze[0].len() - 2, maze.len() - 2),
        true,
    );
    let best_cost = best_path_costs[0].1;
    println!("{best_cost}");
    best_paths_tiles.extend(best_path_costs.iter().map(|x| x.0));

    let distance = |p: (usize, usize), d: (usize, usize)| -> (usize, usize) {
        (
            (p.0 as i32 - d.0 as i32).unsigned_abs() as usize,
            (p.1 as i32 - d.1 as i32).unsigned_abs() as usize,
        )
    };
    for i in 0..(best_path_costs.len() - 1) {
        let p = best_path_costs[i].0;

        let mut neighbors = vec![];
        for offset in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let new_pos = (
                (p.0 as i32 + offset.0) as usize,
                (p.1 as i32 + offset.1) as usize,
            );
            if !best_paths_tiles.contains(&new_pos) && nodes.contains(&new_pos) {
                neighbors.push(new_pos);
            }
        }

        for n in neighbors {
            let mut n_cost = 0;
            n_cost += 1;
            if i > 0 {
                if (distance(p, best_path_costs[i - 1].0).0 == 0) != (distance(p, n).0 == 0) {
                    n_cost += 1000;
                }
            } else if distance(p, n).1 != 0 {
                n_cost += 1000;
            }
            let new_path = dijkstra2(
                &maze_debug,
                &nodes,
                n,
                (maze[0].len() - 2, maze.len() - 2),
                false,
            );
            let new_cost = new_path[0].1 + n_cost + best_path_costs[i].1;

            println!(
                "{}, {}, {}, {}",
                new_path[0].1, n_cost, best_path_costs[i].1, new_cost
            );
            if new_cost == best_cost {
                best_paths_tiles.extend(new_path.iter().map(|x| x.0));
            }
        }
    }
    // let lowest_cost = dijkstra2(
    //     &maze_debug,
    //     &nodes,
    //     (1, 1),
    //     (maze[0].len() - 2, maze.len() - 2),
    //     &mut best_paths_tiles,
    // );
    // println!("{}", lowest_cost);
    // for _ in 0..1 {
    //     let mut new_best_tiles = best_paths_tiles.clone();
    //     let new_cost = dijkstra2(
    //         &maze_debug,
    //         &nodes,
    //         (1, 1),
    //         (maze[0].len() - 2, maze.len() - 2),
    //         &mut new_best_tiles,
    //     );

    //     println!("{new_cost}");
    //     if new_cost == lowest_cost {
    //         best_paths_tiles.extend(new_best_tiles);
    //     }
    // }
    println!("{}", best_paths_tiles.len());
}

fn dijkstra2(
    maze: &[Vec<String>],
    nodes: &[(usize, usize)],
    source: (usize, usize),
    destination: (usize, usize),
    start_cost: bool,
) -> Vec<((usize, usize), u32)> {
    let mut dists = HashMap::new();
    let mut prev = HashMap::new();
    let mut Q = vec![];

    for node in nodes {
        dists.insert(*node, u32::MAX);
        prev.insert(*node, None);
        Q.push(*node);
    }
    dists.entry(source).and_modify(|x| *x = 0).or_insert(0);

    let get_min_dist_node_index =
        |q: &[(usize, usize)], dist: &HashMap<(usize, usize), u32>| -> usize {
            let mut min_dist = u32::MAX;
            let mut min_dist_node = usize::MAX;

            for i in 0..q.len() {
                if dist[&q[i]] < min_dist {
                    min_dist = dist[&q[i]];
                    min_dist_node = i;
                }
            }

            min_dist_node
        };
    let add_coords = |p: (usize, usize), d: (i32, i32)| -> (usize, usize) {
        ((p.0 as i32 + d.0) as usize, (p.1 as i32 + d.1) as usize)
    };
    let distance = |p: (usize, usize), d: (usize, usize)| -> (usize, usize) {
        (
            (p.0 as i32 - d.0 as i32).unsigned_abs() as usize,
            (p.1 as i32 - d.1 as i32).unsigned_abs() as usize,
        )
    };

    for i in maze {
        for j in i {
            print!("{j}");
        }
        println!();
    }
    println!("\n\n\n\n\n");
    while !Q.is_empty() {
        // let mut min_dist_node_indices = get_min_dist_node_index(&Q, &dists).into_iter();
        let mut min_dist_node_index = get_min_dist_node_index(&Q, &dists);
        // while visited.contains(&Q[min_dist_node_index]) {
        //     let next = min_dist_node_indices.next();
        //     if next.is_none() {
        //         break;
        //     }
        //     min_dist_node_index = next.unwrap();
        // }
        let node = Q[min_dist_node_index];
        Q.remove(min_dist_node_index);

        let mut maze_clone = maze.to_vec().clone();
        for i in Q.clone() {
            maze_clone[i.1][i.0] = "+".to_string();
        }
        for i in maze_clone {
            for j in i {
                print!("{j}");
            }
            println!();
        }
        println!("\n\n\n\n\n");

        let mut neighbors = vec![];
        for offset in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let new_pos = add_coords(node, offset);
            if Q.contains(&new_pos) {
                neighbors.push(new_pos);
            }
        }

        for neighbor in neighbors.clone() {
            let mut edge_cost = 1;
            if prev[&node].is_some() {
                if (distance(node, prev[&node].unwrap()).0 == 0)
                    != (distance(node, neighbor).0 == 0)
                {
                    edge_cost += 1000;
                }
            } else if start_cost && distance(node, neighbor).1 != 0 {
                edge_cost += 1000;
            }
            // if neighbors.len() > 1 {
            //     let mut b = false;
            //     for i in neighbors.clone() {
            //         if !visited.contains(&i) {
            //             b = true;
            //             break;
            //         }
            //     }
            //     if b && visited.contains(&neighbor) {
            //         edge_cost += 1000;
            //     }
            // }

            let alt = dists[&node] + edge_cost;
            // println!("{alt}");
            // if alt == dists[&neighbor] {
            //     let mut u = Some(node);
            //     while u.is_some() {
            //         visited.insert(u.unwrap());
            //         u = prev[&u.unwrap()];
            //     }
            // }
            if alt < dists[&neighbor] {
                // visited.clear();
                dists.entry(neighbor).and_modify(|x| *x = alt);
                prev.entry(neighbor).and_modify(|x| *x = Some(node));

                // let mut u = Some(node);
                // while u.is_some() {
                //     visited.insert(u.unwrap());
                //     u = prev[&u.unwrap()];
                // }
            }
        }
    }

    let mut u = Some(destination);
    let mut path = vec![];
    while u.is_some() {
        // visited.insert(u.unwrap());
        path.push(u.unwrap());
        u = prev[&u.unwrap()];
    }

    let mut x = vec![];
    for i in path.clone() {
        x.push(dists[&i]);
    }

    path.iter().map(|x| *x).zip(x).collect::<Vec<_>>()
}
