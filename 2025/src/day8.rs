use std::fs;

struct Vec3(f64, f64, f64);
impl Vec3 {
    fn dist(&self, other: &Vec3) -> f64 {
        (self.0 - other.0).powi(2) + (self.1 - other.1).powi(2) + (self.2 - other.2).powi(2)
    }
}
impl<P> From<P> for Vec3
where
    P: Iterator<Item = f64>,
{
    fn from(mut value: P) -> Self {
        Self(
            value.next().unwrap(),
            value.next().unwrap(),
            value.next().unwrap(),
        )
    }
}

pub fn part_one() {
    let input = fs::read_to_string("./src/input").unwrap();
    let jboxes: Vec<Vec3> = input
        .lines()
        .map(|x| x.split(',').map(|x| x.parse().unwrap()).into())
        .collect();

    let mut dists = vec![];
    for i in 0..jboxes.len() - 1 {
        dists.push(vec![]);
        for j in i + 1..jboxes.len() {
            dists[i].push(jboxes[i].dist(&jboxes[j]));
        }
    }
    let get_smallest_dist_jboxes = |dists: &mut [Vec<f64>]| {
        let mut smallest_dist = f64::MAX;
        let mut smallest_jboxes = (0, 0);
        for i in 0..dists.len() {
            for j in 0..dists[i].len() {
                if dists[i][j] < smallest_dist {
                    smallest_dist = dists[i][j];
                    smallest_jboxes = (i, j);
                }
            }
        }

        dists[smallest_jboxes.0][smallest_jboxes.1] = f64::MAX;
        (smallest_jboxes.0, smallest_jboxes.1 + smallest_jboxes.0 + 1)
    };

    let mut circuits: Vec<Vec<usize>> = vec![];
    let get_jbox_circuit = |circuits: &[Vec<usize>], jbox: usize| {
        for i in 0..circuits.len() {
            if circuits[i].contains(&jbox) {
                return Some(i);
            }
        }

        None
    };

    for _ in 0..1000 {
        let (i, j) = get_smallest_dist_jboxes(&mut dists);
        let i_circuit = get_jbox_circuit(&circuits, i);
        let j_circuit = get_jbox_circuit(&circuits, j);
        // while !i_circuit.is_none() && !j_circuit.is_none() && i_circuit == j_circuit {
        //     (i, j) = get_smallest_dist_jboxes(&mut dists);
        //     i_circuit = get_jbox_circuit(&circuits, i);
        //     j_circuit = get_jbox_circuit(&circuits, j);
        // }

        if !i_circuit.is_none() && !j_circuit.is_none() && i_circuit == j_circuit {
            continue;
        }

        // println!("{i} {j} {i_circuit:?} {j_circuit:?}");
        if i_circuit.is_some() && j_circuit.is_some() {
            let i_circuit = i_circuit.unwrap();
            let j_circuit = j_circuit.unwrap();
            let c = circuits.remove(i_circuit.max(j_circuit));
            circuits[i_circuit.min(j_circuit)].extend_from_slice(&c);
        } else if let Some(circuit) = i_circuit {
            circuits[circuit].push(j);
        } else if let Some(circuit) = j_circuit {
            circuits[circuit].push(i);
        } else {
            circuits.push(vec![i, j]);
        }

        // for circuit in &circuits {
        //     for i in circuit {
        //         print!("{i} ");
        //     }
        //     println!();
        // }

        // println!();
    }

    circuits.sort_by(|a, b| a.len().cmp(&b.len()));
    circuits.reverse();

    println!(
        "{}",
        circuits[0].len() * circuits[1].len() * circuits[2].len()
    );
}

pub fn part_two() {
    let input = fs::read_to_string("./src/input").unwrap();
    let jboxes: Vec<Vec3> = input
        .lines()
        .map(|x| x.split(',').map(|x| x.parse().unwrap()).into())
        .collect();

    let mut dists = vec![];
    for i in 0..jboxes.len() - 1 {
        dists.push(vec![]);
        for j in i + 1..jboxes.len() {
            dists[i].push(jboxes[i].dist(&jboxes[j]));
        }
    }
    let get_smallest_dist_jboxes = |dists: &mut [Vec<f64>]| {
        let mut smallest_dist = f64::MAX;
        let mut smallest_jboxes = (0, 0);
        for i in 0..dists.len() {
            for j in 0..dists[i].len() {
                if dists[i][j] < smallest_dist {
                    smallest_dist = dists[i][j];
                    smallest_jboxes = (i, j);
                }
            }
        }

        dists[smallest_jboxes.0][smallest_jboxes.1] = f64::MAX;
        (smallest_jboxes.0, smallest_jboxes.1 + smallest_jboxes.0 + 1)
    };

    let mut circuits: Vec<Vec<usize>> = vec![];
    let get_jbox_circuit = |circuits: &[Vec<usize>], jbox: usize| {
        for i in 0..circuits.len() {
            if circuits[i].contains(&jbox) {
                return Some(i);
            }
        }

        None
    };

    loop {
        let (i, j) = get_smallest_dist_jboxes(&mut dists);
        let i_circuit = get_jbox_circuit(&circuits, i);
        let j_circuit = get_jbox_circuit(&circuits, j);

        if !i_circuit.is_none() && !j_circuit.is_none() && i_circuit == j_circuit {
            continue;
        }

        if i_circuit.is_some() && j_circuit.is_some() {
            let i_circuit = i_circuit.unwrap();
            let j_circuit = j_circuit.unwrap();
            let c = circuits.remove(i_circuit.max(j_circuit));
            circuits[i_circuit.min(j_circuit)].extend_from_slice(&c);
        } else if let Some(circuit) = i_circuit {
            circuits[circuit].push(j);
        } else if let Some(circuit) = j_circuit {
            circuits[circuit].push(i);
        } else {
            circuits.push(vec![i, j]);
        }

        if circuits.len() == 1 && circuits[0].len() == jboxes.len() {
            return println!("{}", jboxes[i].0 * jboxes[j].0);
        }
    }
}
