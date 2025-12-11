use std::fs;

// fn dijkstra(connections: &[Vec<usize>]) -> u32 {
//     let mut dists = vec![];
//     let mut prev = vec![];
//     let mut Q = vec![];

//     for node in 0..connections.len() {
//         dists.push(u32::MAX);
//         prev.push(None);
//         if node != connections.len() - 1 {
//             Q.push(node);
//         }
//     }
//     dists[0] = 0;

//     let get_min_dist_node_index = |q: &[usize], dist: &[u32]| -> usize {
//         let mut min_dist = u32::MAX;
//         let mut node = usize::MAX;

//         for i in 0..q.len() {
//             if dist[q[i]] <= min_dist {
//                 min_dist = dist[q[i]];
//                 node = i;
//             }
//         }

//         node
//     };
//     // let add_coords = |p: (usize, usize), d: (i32, i32)| -> (usize, usize) {
//     //     ((p.0 as i32 + d.0) as usize, (p.1 as i32 + d.1) as usize)
//     // };

//     let mut num_paths = 0;
//     while !Q.is_empty() {
//         if Q.iter().all(|&x| dists[x] == u32::MAX) {
//             break;
//         }
//         let next_node_index = get_min_dist_node_index(&Q, &dists);
//         let node = Q[next_node_index];
//         Q.remove(next_node_index);
//         // if node == destination {
//         //     num_paths += 1;
//         //     continue;
//         //     // break;
//         // }

//         // let mut neighbors = vec![];
//         // for offset in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
//         //     let new_pos = add_coords(node, offset);
//         //     if Q.contains(&new_pos) {
//         //         neighbors.push(new_pos);
//         //     }
//         // }

//         for &neighbor in &connections[node] {
//             let alt = dists[node] + 1;
//             if alt < dists[neighbor as usize] {
//                 dists[neighbor as usize] = alt;
//                 prev[neighbor as usize] = Some(node);
//             }
//             if neighbor == connections.len() - 1 {
//                 num_paths += 1;

//                 print!("{} ", connections.len() - 1);
//                 let mut u = Some(node);
//                 while u.is_some() {
//                     print!("{} ", u.unwrap());
//                     u = prev[u.unwrap()];
//                 }
//                 println!();
//             }
//         }
//     }

//     // let mut u = Some(destination);
//     // let mut path = vec![];
//     // while u.is_some() {
//     //     path.push(u.unwrap());
//     //     u = prev[u.unwrap()];
//     // }

//     num_paths
// }

fn bfs(
    node: usize,
    destination: usize,
    connections: &[Vec<usize>],
    history: &mut [Option<u32>],
) -> u32 {
    let mut num_paths = 0;
    for &neighbor in &connections[node] {
        if neighbor == destination {
            num_paths += 1;
            continue;
        }
        if let Some(paths) = history[neighbor] {
            num_paths += paths;
            continue;
        }
        if connections[neighbor].is_empty() {
            history[neighbor] = Some(0);
            continue;
        }

        let paths = bfs(neighbor, destination, connections, history);
        history[neighbor] = Some(paths);
        num_paths += paths;
    }
    num_paths
}

pub fn part_one() {
    let input = fs::read_to_string("./src/input").unwrap();

    let mut nodes = vec!["you".to_string()];
    for line in input.lines() {
        let node = line.split(": ").nth(0).unwrap();
        if node != "you" {
            nodes.push(node.to_string());
        }
    }
    nodes.push("out".to_string());

    let mut connections = vec![vec![]; nodes.len()];
    for line in input.lines() {
        let &[name, neighbors] = line.split(": ").collect::<Vec<_>>().as_slice() else {
            panic!("🫪")
        };

        for neighbor in neighbors.split(' ') {
            connections[nodes.iter().position(|x| *x == name).unwrap()]
                .push(nodes.iter().position(|x| *x == neighbor).unwrap());
        }
    }

    // let num_paths = dijkstra(&connections);
    // println!("{num_paths}");
    // connections[0].remove(1);
    // let num_paths = dijkstra(&connections);
    // println!("{num_paths}");

    println!(
        "{}",
        bfs(
            0,
            nodes.len() - 1,
            &connections,
            &mut vec![None; nodes.len() - 1]
        )
    );
}

// fn bfs2(
//     depth: usize,
//     node: usize,
//     destination: usize,
//     connections: &[Vec<usize>],
//     history: &mut [Option<Vec<Vec<usize>>>],
// ) -> Vec<Vec<usize>> {
//     let mut paths = vec![];
//     for (i, &neighbor) in connections[node].iter().enumerate() {
//         if neighbor == destination {
//             paths.push(vec![destination, node]);
//             continue;
//         }
//         if let Some(_paths) = &history[neighbor] {
//             paths.extend(_paths.clone().into_iter().map(|mut x| {
//                 x.push(node);
//                 x
//             }));
//             continue;
//         }
//         if connections[neighbor].is_empty() {
//             history[neighbor] = Some(vec![]);
//             continue;
//         }

//         let _paths = bfs2(depth + 1, neighbor, destination, connections, history);
//         paths.extend(_paths.clone().into_iter().map(|mut x| {
//             x.push(node);
//             x
//         }));
//         history[neighbor] = Some(_paths);
//         println!("{dep")
//         if depth < 5 {
//             println!("{}/{}", i + 1, connections[node].len());
//         }
//     }

//     paths
// }

fn bfs2(
    depth: usize,
    node: usize,
    // fft: Option<usize>,
    // dac: Option<usize>,
    destination: usize,
    connections: &[Vec<usize>],
    history: &mut [Option<u32>],
) -> u32 {
    let mut num_paths = 0;
    // let found_fft = fft.is_none() || node == fft.unwrap();
    // let found_dac = dac.is_none() || node == dac.unwrap();
    for (i, &neighbor) in connections[node].iter().enumerate() {
        // for &neighbor in &connections[node] {
        if neighbor == destination {
            // if found_dac && found_fft {
            num_paths += 1;
            // }
            continue;
        }
        // if let Some(paths) = history[neighbor] {
        //     num_paths += paths;
        //     continue;
        // }
        if connections[neighbor].is_empty() {
            history[neighbor] = Some(0);
            continue;
        }

        let paths = bfs2(
            depth + 1,
            neighbor,
            // if found_fft { None } else { fft },
            // if found_dac { None } else { dac },
            destination,
            connections,
            history,
        );
        history[neighbor] = Some(paths);
        num_paths += paths;
        println!("{depth}");
        // if depth < 5 {
        //     println!("{}/{}", i + 1, connections[node].len());
        // }
    }
    num_paths
}

pub fn part_two() {
    let input = fs::read_to_string("./src/input").unwrap();
    let mut nodes = vec!["svr".to_string()];
    let mut fft = 0;
    let mut dac = 0;
    for line in input.lines() {
        let node = line.split(": ").nth(0).unwrap();
        if node != "svr" {
            nodes.push(node.to_string());
        }
        if node == "fft" {
            fft = nodes.len() - 1;
        } else if node == "dac" {
            dac = nodes.len() - 1;
        }
    }
    nodes.push("out".to_string());

    let mut connections = vec![vec![]; nodes.len()];
    for line in input.lines() {
        let &[name, neighbors] = line.split(": ").collect::<Vec<_>>().as_slice() else {
            panic!("🫪")
        };

        for neighbor in neighbors.split(' ') {
            connections[nodes.iter().position(|x| *x == name).unwrap()]
                .push(nodes.iter().position(|x| *x == neighbor).unwrap());
        }
    }

    println!(
        "{:?}",
        bfs2(
            0,
            fft,
            // Some(fft),
            // Some(dac),
            // nodes.len() - 1,
            dac,
            &connections,
            &mut vec![None; nodes.len()]
        ) // .iter()
          // .filter(|x| x.contains(&fft) && x.contains(&dac))
          // .count()
    );
}
