use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    sync::{Arc, Mutex},
    thread,
};

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let maze: Vec<Vec<String>> = input
        .lines()
        .map(|x| x.chars().map(|y| y.to_string()).collect())
        .collect();

    let mut origin = (0, 0);
    let mut destination = (0, 0);
    let mut path = vec![];
    (0..maze.len()).for_each(|i| {
        (0..maze[i].len()).for_each(|j| {
            if maze[i][j] != "#" {
                path.push((j, i));
            }
            if maze[i][j] == "S" {
                origin = (j, i);
            } else if maze[i][j] == "E" {
                destination = (j, i);
            }
        });
    });

    // let debug_lines = 5;
    let (destination_costs, _) = dijkstra(&path, destination, origin);
    let (origin_costs, _) = dijkstra(&path, origin, destination);
    // for i in 0..maze.len() {
    //     for j in 0..maze[i].len() {
    //         if new_path.contains(&(j, i)) {
    //             print!("O");
    //         } else {
    //             print!("{}", maze[i][j]);
    //         }
    //     }
    //     println!();
    // }
    // println!("{}{}", path.len(), "\n".repeat(debug_lines));
    // println!();
    // let path_cost = path.len() - 1;
    // for i in 0..maze.len() {
    //     for j in 0..maze[i].len() {
    //         if path.contains(&(j, i)) {
    //             print!("O");
    //         } else {
    //             print!("{}", maze[i][j]);
    //         }
    //     }
    //     println!();
    // }
    // println!("{}{}", path_cost, "\n".repeat(debug_lines));
    // println!("{}", path_cost);

    let check_edge = |p: (usize, usize)| -> bool {
        p.0 == 0 || p.0 == maze[0].len() - 1 || p.1 == 0 || p.1 == maze.len() - 1
    };
    let get_neighbor_obstacles = |p: (usize, usize)| -> Vec<(usize, usize)> {
        let mut neighboring_obstacles = vec![];
        for offset in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let new_pos = (
                (p.0 as i32 + offset.0) as usize,
                (p.1 as i32 + offset.1) as usize,
            );
            if !path.contains(&new_pos) {
                neighboring_obstacles.push(new_pos);
            }
        }
        neighboring_obstacles
    };
    let get_neighbor_nodes = |p: (usize, usize)| -> Vec<(usize, usize)> {
        let mut neighboring_nodes = vec![];
        for offset in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let new_pos = (
                (p.0 as i32 + offset.0) as usize,
                (p.1 as i32 + offset.1) as usize,
            );
            if path.contains(&new_pos) {
                neighboring_nodes.push(new_pos);
            }
        }
        neighboring_nodes
    };

    let mut cheat_saves = HashMap::new();
    let mut disabled_obstacles = HashSet::new();
    for i in 0..path.len() {
        let p = path[i];
        let neighboring_obstacles = get_neighbor_obstacles(p);
        if p == destination || get_neighbor_nodes(p).contains(&destination) {
            continue;
        }

        for j in 0..neighboring_obstacles.len() {
            let obstacle = neighboring_obstacles[j];
            if check_edge(obstacle) {
                continue;
            }

            let next_neighboring_nodes = get_neighbor_nodes(obstacle);
            if next_neighboring_nodes.is_empty()
                || next_neighboring_nodes.len() == 1 && next_neighboring_nodes.contains(&p)
            {
                continue;
            }

            if disabled_obstacles.contains(&obstacle) {
                continue;
            }
            disabled_obstacles.insert(obstacle);

            let mut cheat_nodes = path.clone();
            cheat_nodes.push(obstacle);
            // for i in 0..maze.len() {
            //     for j in 0..maze[i].len() {
            //         if (j, i) == obstacle {
            //             print!("O");
            //         } else if (j, i) == next_obstacle {
            //             print!("Q");
            //         } else if (j, i) == p {
            //             print!("&");
            //         } else {
            //             print!("{}", maze[i][j]);
            //         }
            //     }
            //     println!();
            // }
            // println!("{}", "\n".repeat(debug_lines));

            // let (_, new_path) = dijkstra(&cheat_nodes, origin, destination);
            let neighbor_nodes = get_neighbor_nodes(obstacle);
            let mut lowest_cost_dest = u32::MAX;
            for node in neighbor_nodes.clone() {
                if destination_costs[&node] < lowest_cost_dest {
                    lowest_cost_dest = destination_costs[&node];
                }
            }
            let mut lowest_cost_orig = u32::MAX;
            for node in neighbor_nodes {
                if origin_costs[&node] < lowest_cost_orig {
                    lowest_cost_orig = origin_costs[&node];
                }
            }

            // println!(
            //     "{}, {}, {}, {}, {}",
            //     lowest_cost_dest,
            //     lowest_cost_orig,
            //     lowest_cost_dest + lowest_cost_orig + 2,
            //     new_path.len() - 1,
            //     path.len() - 1
            // );
            cheat_saves
                .entry(path.len() as u32 - lowest_cost_dest - lowest_cost_orig - 3)
                .and_modify(|x| *x += 1)
                .or_insert(1u32);
            // for i in 0..maze.len() {
            //     for j in 0..maze[i].len() {
            //         if (j, i) == obstacle {
            //             print!("&");
            //         } else if new_path.contains(&(j, i)) {
            //             print!("O");
            //         } else {
            //             print!("{}", maze[i][j]);
            //         }
            //     }
            //     println!();
            // }
            // println!();
            // println!(
            //     "{}{}",
            //     path.len() - new_path.len(),
            //     "\n".repeat(debug_lines)
            // );
            // println!("\t{}/{}", j, neighboring_obstacles.len());
        }
        //     let next_neighboring_obstacles = get_neighbor_obstacles(obstacle);
        //     for next_obstacle in next_neighboring_obstacles {
        //         if check_edge(next_obstacle) || get_neighbor_obstacles(next_obstacle).len() == 4 {
        //             continue;
        //         }
        //         if disabled_obstacles.contains(&(obstacle, next_obstacle))
        //             || disabled_obstacles.contains(&(next_obstacle, obstacle))
        //         {
        //             continue;
        //         }

        //         disabled_obstacles.insert((obstacle, next_obstacle));

        //         let mut cheat_nodes = path.clone();
        //         cheat_nodes.push(obstacle);
        //         cheat_nodes.push(next_obstacle);
        //         // for i in 0..maze.len() {
        //         //     for j in 0..maze[i].len() {
        //         //         if (j, i) == obstacle {
        //         //             print!("O");
        //         //         } else if (j, i) == next_obstacle {
        //         //             print!("Q");
        //         //         } else if (j, i) == p {
        //         //             print!("&");
        //         //         } else {
        //         //             print!("{}", maze[i][j]);
        //         //         }
        //         //     }
        //         //     println!();
        //         // }
        //         // println!("{}", "\n".repeat(debug_lines));

        //         let new_path = dijkstra(&cheat_nodes, starting_pos, destination);
        //         cheat_saves
        //             .entry(path.len() - new_path.len())
        //             .and_modify(|x| *x += 1)
        //             .or_insert(1u32);
        //         for i in 0..maze.len() {
        //             for j in 0..maze[i].len() {
        //                 if (j, i) == obstacle {
        //                     print!("O");
        //                 } else if (j, i) == next_obstacle {
        //                     print!("Q");
        //                 } else if new_path.contains(&(j, i)) {
        //                     print!("O");
        //                 } else {
        //                     print!("{}", maze[i][j]);
        //                 }
        //             }
        //             println!();
        //         }
        //         // println!();
        //         println!(
        //             "{}{}",
        //             path.len() - new_path.len(),
        //             "\n".repeat(debug_lines)
        //         );
        //     }
        // }
        // println!("{}/{}", i, path.len());
    }

    println!(
        "{}",
        cheat_saves
            .iter()
            .filter(|x| *x.0 >= 100)
            .fold(0, |acc, x| acc + x.1)
    );
    // for cheat in cheat_saves {
    //     println!("{}: {}", cheat.0, cheat.1);
    // }
}

fn dijkstra(
    nodes: &[(usize, usize)],
    source: (usize, usize),
    destination: (usize, usize),
) -> (HashMap<(usize, usize), u32>, Vec<(usize, usize)>) {
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

    while !Q.is_empty() {
        let min_dist_node_index = get_min_dist_node_index(&Q, &dists);
        let node = Q[min_dist_node_index];
        Q.remove(min_dist_node_index);
        if dists[&node] == u32::MAX {
            continue;
        }
        if node == destination {
            break;
        }

        let mut neighbors = vec![];
        for offset in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let new_pos = add_coords(node, offset);
            if Q.contains(&new_pos) {
                neighbors.push(new_pos);
            }
        }

        for neighbor in neighbors {
            let alt = dists[&node] + 1;
            if alt < dists[&neighbor] {
                dists.entry(neighbor).and_modify(|x| *x = alt);
                prev.entry(neighbor).and_modify(|x| *x = Some(node));
            }
        }
    }

    let mut u = Some(destination);
    let mut path = vec![];
    while u.is_some() {
        path.push(u.unwrap());
        u = prev[&u.unwrap()];
    }

    (dists, path)
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();
}
