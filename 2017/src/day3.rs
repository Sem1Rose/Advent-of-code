fn get_ring(num: u32) -> u32 {
    let ceiled = (num as f32).sqrt().ceil() as u32;

    if ceiled % 2 == 0 {
        ceiled + 1
        // ceiled / 2
    } else {
        ceiled
        // (ceiled - 1) / 2
    }
}

pub fn part_one() {
    let num = 312051;
    let ring = get_ring(num);
    let half_ring = (ring as f32 / 2.0).floor() as u32;

    let region = ((ring * ring - num) as f32 / half_ring as f32).floor() as u32;
    let step = if region % 2 == 0 {
        half_ring - (ring * ring - num) % half_ring
    } else {
        (ring * ring - num) % half_ring
    };
    println!("{}", half_ring + step);
}

pub fn part_two() {
    // Bruteforce begins
    let num = 312051;
    let mut map: Vec<Vec<u32>> = vec![vec![0; 21]; 21];
    let mut x = 10;
    let mut y = 10;
    map[x][y] = 1;
    x += 1;

    let mut first = -1;

    let mut i = 2;
    loop {
        if first != -1 {
            break;
        }
        let ring = get_ring(i);
        let s = sum(&mut map, x, y, i);
        i += 1;
        if s > num {
            first = s as i32;
        }
        map[x][y] = s as u32;

        for _ in 1..(ring - 1) {
            if first != -1 {
                break;
            }
            y -= 1;
            let s = sum(&mut map, x, y, i);
            if s > num {
                first = s as i32;
            }
            map[x][y] = s as u32;
            i += 1;
        }
        for _ in 1..ring {
            if first != -1 {
                break;
            }
            x -= 1;
            let s = sum(&mut map, x, y, i);
            if s > num {
                first = s as i32;
            }
            map[x][y] = s as u32;
            i += 1;
        }
        for _ in 1..ring {
            if first != -1 {
                break;
            }
            y += 1;
            let s = sum(&mut map, x, y, i);
            if s > num {
                first = s as i32;
            }
            map[x][y] = s as u32;
            i += 1;
        }
        for _ in 1..ring {
            if first != -1 {
                break;
            }
            x += 1;
            let s = sum(&mut map, x, y, i);
            if s > num {
                first = s as i32;
            }
            map[x][y] = s as u32;
            i += 1;
        }
        x += 1;
        i += 1;

        if first != -1 {
            break;
        }
    }

    println!("{first}");
}

fn sum(map: &mut [Vec<u32>], x: usize, y: usize, i: u32) -> u32 {
    let mut s = 0;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            s += map[(x as i32 + j) as usize][(y as i32 + i) as usize];
        }
    }
    s
}
