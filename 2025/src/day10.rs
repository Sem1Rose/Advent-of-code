use std::fs;

fn recurse(depth: u8, buttons: &[Vec<usize>], lights: &[bool]) -> bool {
    if depth == 0 {
        return lights.iter().all(|x| !*x);
    }

    for i in 0..buttons.len() {
        let mut new_lights = lights.to_vec();
        let mut new_buttons = buttons.to_vec();
        let button = new_buttons.remove(i);
        for light in button {
            new_lights[light] = !new_lights[light];
        }

        if recurse(depth - 1, &new_buttons, &new_lights) {
            return true;
        }
    }

    false
}

pub fn part_one() {
    let input = fs::read_to_string("./src/input").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let splits = line.split(' ').collect::<Vec<_>>();
        let lights = splits[0].to_string()[1..splits[0].len() - 1]
            .chars()
            .map(|x| if x == '#' { true } else { false })
            .collect::<Vec<_>>();
        let buttons = splits[1..splits.len() - 1]
            .iter()
            .map(|x| {
                x.to_string()[1..x.len() - 1]
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut depth = 1;
        loop {
            if recurse(depth, &buttons, &lights) {
                sum += depth as u32;
                break;
            }
            depth += 1;
        }
    }
    println!("{sum}");
}

fn recurse2(buttons: &[Vec<usize>], jolts: &[u32]) -> Option<u32> {
    if jolts.iter().all(|x| *x == 0) {
        return Some(0);
    }

    for i in 0..buttons.len() {
        let mut new_buttons = buttons.to_vec();
        let button = new_buttons.remove(i);
        let max = jolts[*button.iter().min_by_key(|&&x| jolts[x]).unwrap()];
        for n in 0..=max {
            let mut new_jolts = jolts.to_vec();
            let mut cont = false;
            for jolt in &button {
                if new_jolts[*jolt] < n {
                    cont = true;
                    break;
                }
                new_jolts[*jolt] -= n;
            }
            if cont {
                break;
            }

            if let Some(sum) = recurse2(&buttons, &new_jolts) {
                return Some(n + sum);
            }
        }
    }

    None
}

pub fn part_two() {
    let input = fs::read_to_string("./src/input").unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let splits = line.split(' ').collect::<Vec<_>>();
        let buttons = splits[1..splits.len() - 1]
            .iter()
            .map(|x| {
                x.to_string()[1..x.len() - 1]
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let jolts = splits[splits.len() - 1].to_string()[1..splits[splits.len() - 1].len() - 1]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();

        sum += recurse2(&buttons, &jolts).unwrap();
        println!("{sum}");
    }
}
