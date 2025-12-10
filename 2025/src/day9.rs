use std::fs;

pub fn part_one() {
    let input = fs::read_to_string("./src/input").unwrap();
    let tiles: Vec<_> = input
        .lines()
        .map(|x| {
            <(i32, i32)>::from(
                <[i32; 2]>::try_from(x.split(',').map(|z| z.parse().unwrap()).collect::<Vec<_>>())
                    .unwrap(),
            )
        })
        .collect();

    // let mut top_right_sorted = tiles.clone();
    // top_right_sorted.sort_by(|a, b| (a.1 - a.0).cmp(&(b.1 - b.0)));

    // let mut bot_left_sorted = tiles.clone();
    // bot_left_sorted.sort_by(|a, b| (a.0 - a.1).cmp(&(b.0 - b.1)));

    // for i in top_right_sorted {
    //     println!("{:?}", i);
    // }
    // println!();
    // for i in bot_left_sorted {
    //     println!("{:?}", i);
    // }
    let area = |(x1, y1): (i32, i32), (x2, y2): (i32, i32)| {
        ((x2 - x1).abs() + 1) as u64 * ((y2 - y1).abs() + 1) as u64
    };
    let mut largest_area = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            // if i >= tiles.len() || j >= tiles.len() {
            //     break;
            // }
            let area = area(tiles[i], tiles[j]);
            // println!("{:?} {:?} {}", tiles[i], tiles[j], area);
            if area > largest_area {
                largest_area = area;
                // println!("{largest_area}");
            }
        }
    }
    println!("{largest_area}");
}

pub fn part_two() {
    let input = fs::read_to_string("./src/input").unwrap();
    let tiles: Vec<_> = input
        .lines()
        .map(|x| {
            <[i32; 2]>::try_from(x.split(',').map(|z| z.parse().unwrap()).collect::<Vec<_>>())
                .unwrap()
        })
        .collect();

    let area = |[x1, y1]: [i32; 2], [x2, y2]: [i32; 2]| {
        ((x2 - x1).abs() + 1) as u64 * ((y2 - y1).abs() + 1) as u64
    };
    let mut largest_area = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let area = area(tiles[i], tiles[j]);

            let mut x = j + 1;
            if x == tiles.len() {
                x = 0;
            }
            let calc_quadrant = |[x1, y1]: [i32; 2], [ax, ay]: [i32; 2], [bx, by]: [i32; 2]| {
                let left = ax.min(bx);
                let right = ax.max(bx);
                let top = ay.min(by);
                let bottom = ay.max(by);
                // println!(
                //     "{},{} {},{} {},{} = {} {} {} {}",
                //     x1, y1, ax, ay, bx, by, left, right, top, bottom
                // );

                let mut quadrant = [0, 0];
                if x1 <= left {
                    quadrant[0] = -1;
                } else if x1 >= right {
                    quadrant[0] = 1;
                }
                if y1 <= top {
                    quadrant[1] = 1;
                } else if y1 >= bottom {
                    quadrant[1] = -1;
                }

                quadrant
            };
            let mut prev_quadrant = calc_quadrant(tiles[j], tiles[i], tiles[j]);
            // println!("{i} {j} {prev_quadrant:?}");
            let mut valid = true;
            while x != j {
                let current_quadrant = calc_quadrant(tiles[x], tiles[i], tiles[j]);
                // println!("{x} {current_quadrant:?}");
                if current_quadrant == [0, 0] {
                    valid = false;
                    break;
                }
                if prev_quadrant[1] == 0 && current_quadrant[0] != prev_quadrant[0] {
                    valid = false;
                    break;
                } else if prev_quadrant[1] == 0 && current_quadrant[0] != prev_quadrant[0] {
                    valid = false;
                    break;
                } else if prev_quadrant[0] == 0 && current_quadrant[1] != prev_quadrant[1] {
                    valid = false;
                    break;
                } else if prev_quadrant[0] == 0 && current_quadrant[1] != prev_quadrant[1] {
                    valid = false;
                    break;
                }

                x += 1;
                if x == tiles.len() {
                    x = 0;
                }
                prev_quadrant = current_quadrant;
            }

            if valid && area > largest_area {
                largest_area = area;
                // println!("{largest_area}");
            }
            // println!();
        }
    }
    println!("{largest_area}");

    // let width = tiles.iter().max_by(|a, b| a[0].cmp(&b[0])).unwrap()[0];
    // let height = tiles.iter().max_by(|a, b| a[1].cmp(&b[1])).unwrap()[1];
    // println!("{width}x{height}");
    // let mut map = vec![vec![false; (width + 1) as usize]; (height + 1) as usize];
    // for i in 0..tiles.len() {
    //     let [x, y] = tiles[i];
    //     map[y as usize][x as usize] = true;
    //     let [next_x, next_y];
    //     if i == tiles.len() - 1 {
    //         [next_x, next_y] = tiles[0];
    //     } else {
    //         [next_x, next_y] = tiles[i + 1];
    //     }

    //     if next_x == x {
    //         for j in (y + 1).min(next_y + 1)..next_y.max(y) {
    //             map[j as usize][x as usize] = true;
    //         }
    //     } else {
    //         for i in (x + 1).min(next_x + 1)..next_x.max(x) {
    //             map[y as usize][i as usize] = true;
    //         }
    //     }
    // }

    // let first_node;
    // if (tiles[0][0] as usize) < map[tiles[0][1] as usize].len() - 1
    //     && map[tiles[0][1] as usize][tiles[0][0] as usize + 1] != 0
    // {
    //     if (tiles[0][1] as usize) < map.len() - 1
    //         && map[tiles[0][1] as usize + 1][tiles[0][0] as usize] != 0
    //     {
    //         first_node = (tiles[0][0] + 1, tiles[0][1] + 1);
    //     // } else if map[tiles[0][1] as usize - 1][tiles[0][0] as usize] != 0 {
    //     } else {
    //         first_node = (tiles[0][0] + 1, tiles[0][1] - 1);
    //     }
    // // } else if map[tiles[0][1] as usize][tiles[0][0] as usize - 1] != 0 {
    // } else {
    //     if (tiles[0][1] as usize) < map.len() - 1
    //         && map[tiles[0][1] as usize + 1][tiles[0][0] as usize] != 0
    //     {
    //         first_node = (tiles[0][0] - 1, tiles[0][1] + 1);
    //     // } else if map[tiles[0][1] as usize - 1][tiles[0][0] as usize] != 0 {
    //     } else {
    //         first_node = (tiles[0][0] - 1, tiles[0][1] - 1);
    //     }
    // }

    // println!("{:?} {:?}", tiles[0], first_node);

    // span filling algorithm thanks wikipedia
    // let mut s: VecDeque<[isize; 4]> = VecDeque::from(vec![
    //     [first_node.0, first_node.0, first_node.1, 1],
    //     [first_node.0, first_node.0, first_node.1 - 1, -1],
    // ]);
    // while !s.is_empty() {
    //     let [mut x1, x2, y, dy] = s.pop_back().unwrap();
    //     let mut x = x1;
    //     if map[y as usize][x as usize] == 0 || map[y as usize][x as usize] == 3 {
    //         while map[y as usize][x as usize - 1] == 0 || map[y as usize][x as usize - 1] == 3 {
    //             map[y as usize][x as usize - 1] = 3;
    //             x -= 1;
    //             if x == 0 {
    //                 break;
    //             }
    //         }
    //         if x < x1 {
    //             s.push_front([x, x1 - 1, y - dy, -dy]);
    //         }
    //     }
    //     while x1 <= x2 {
    //         while map[y as usize][x1 as usize] == 0 {
    //             map[y as usize][x1 as usize] = 3;
    //             x1 += 1;
    //             if x1 as usize >= map[y as usize].len() {
    //                 break;
    //             }
    //         }
    //         if x1 > x {
    //             s.push_front([x, x1 - 1, y + dy, dy]);
    //         }
    //         if x1 - 1 > x2 {
    //             s.push_front([x2 + 1, x1 - 1, y - dy, -dy]);
    //         }
    //         x1 += 1;
    //         while x1 < x2 && map[y as usize][x1 as usize] != 0 {
    //             x1 += 1;
    //             if x1 as usize >= map[y as usize].len() {
    //                 break;
    //             }
    //         }
    //         x = x1;
    //     }
    // }
    // for i in &map {
    //     for j in i {
    //         print!("{j}");
    //     }
    //     println!();
    // }
    // println!();

    // for j in 1..map.len() - 1 {
    //     let mut up = map[j - 1][0];
    //     let mut cur = map[j][0];
    //     let mut dow = map[j + 1][0];
    //     for i in 1..map[j].len() - 1 {
    //         if map[j - 1][i] {
    //             up = !up;
    //         }
    //         if map[j + 1][i] {
    //             dow = !dow;
    //         }
    //         if map[j][i] {
    //             cur = !cur;
    //             continue;
    //         }
    //         if up && dow {
    //             cur = true;
    //         }
    //         if cur {
    //             map[j][i] = true;
    //         }
    //     }
    // }
    // for i in &map {
    //     for j in i {
    //         print!("{}", if *j { 1 } else { 0 });
    //     }
    //     println!();
    // }
    // println!();

    // let get_submap = |[x1, y1]: [i32; 2], [x2, y2]: [i32; 2], map: &[Vec<i32>]| {
    //     map[(y1.min(y2)) as usize..((y2 + 1).max(y1 + 1)) as usize]
    //         .iter()
    //         .cloned()
    //         .map(|x| x[(x1.min(x2)) as usize..((x2 + 1).max(x1 + 1)) as usize].to_vec())
    //         .collect::<Vec<Vec<_>>>()
    // };
    // let valid = |submap: Vec<Vec<i32>>| !submap.iter().flatten().any(|x| *x == 0);

    // let submap = get_submap(tiles[4], tiles[6], &map);
    // for i in submap {
    //     for j in i {
    //         print!("{j}");
    //     }
    //     println!();
    // }
}
