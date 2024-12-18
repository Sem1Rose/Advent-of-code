use std::collections::HashMap;
use std::fs::read_to_string;

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let map_size = (70, 70);
    let bytes: Vec<(usize, usize)> = input
        .lines()
        .take(1024)
        .map(|x| {
            (
                x.split(',').next().unwrap().parse().unwrap(),
                x.split(',').last().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut nodes = vec![];
    (0..=map_size.0).for_each(|i| {
        (0..=map_size.1).for_each(|j| {
            if !bytes.contains(&(j, i)) {
                nodes.push((j, i));
                //     print!(".");
                // } else {
                //     print!("#");
            }
        });
        // println!();
    });

    let path = dijkstra(&nodes, (0, 0), map_size);
    println!("{}", path.len() - 1);
}

fn dijkstra(
    nodes: &[(usize, usize)],
    source: (usize, usize),
    destination: (usize, usize),
) -> Vec<(usize, usize)> {
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

    path
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let bytes: Vec<(usize, usize)> = input
        .lines()
        .map(|x| {
            (
                x.split(',').next().unwrap().parse().unwrap(),
                x.split(',').last().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let map_size = (70, 70);

    let mut batch: Vec<_> = bytes.iter().cloned().take(2900).collect();
    let mut nodes = vec![];
    (0..=map_size.0).for_each(|i| {
        (0..=map_size.1).for_each(|j| {
            if !batch.contains(&(j, i)) {
                nodes.push((j, i));
                //     print!(".");
                // } else {
                //     print!("#");
            }
        });
        // println!();
    });
    let mut path = dijkstra(&nodes, (0, 0), map_size);

    println!("first pass");
    while path.len() > 1 {
        let blocking_index = bytes
            .iter()
            .position(|x| x == bytes.iter().find(|y| path.contains(y)).unwrap())
            .unwrap();

        batch = bytes.iter().cloned().take(blocking_index + 1).collect();
        nodes = vec![];
        (0..=map_size.0).for_each(|i| {
            (0..=map_size.1).for_each(|j| {
                if !batch.contains(&(j, i)) {
                    nodes.push((j, i));
                    //     print!(".");
                    // } else {
                    //     print!("#");
                }
            });
            // println!();
        });
        path = dijkstra(&nodes, (0, 0), map_size);

        println!(
            "{}: {},{}",
            blocking_index,
            batch.last().unwrap().0,
            batch.last().unwrap().1
        );
    }
}
