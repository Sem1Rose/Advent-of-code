use std::fs;

pub fn part_one() {
    let input = fs::read_to_string("./src/input").unwrap();
    let shapes = input
        .split("\n\n")
        .take(input.split("\n\n").count() - 1)
        .map(|x| {
            x.split(":\n")
                .nth(1)
                .unwrap()
                .lines()
                .map(|y| {
                    y.chars()
                        .map(|z| if z == '#' { true } else { false })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let densities = shapes
        .iter()
        .map(|x| x.iter().flatten().filter(|&&y| y).count())
        .collect::<Vec<_>>();

    let regions = input
        .split("\n\n")
        .nth(input.split("\n\n").count() - 1)
        .unwrap()
        .lines()
        .map(|x| {
            <(usize, usize)>::try_from(
                <[usize; 2]>::try_from(
                    x.split(": ")
                        .nth(0)
                        .unwrap()
                        .split("x")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let region_shapes = input
        .split("\n\n")
        .nth(input.split("\n\n").count() - 1)
        .unwrap()
        .lines()
        .map(|x| {
            x.split(": ")
                .nth(1)
                .unwrap()
                .split(" ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut guaranteed = 0;
    let mut possible = 0;
    let mut impossible = 0;
    for (i, &region) in regions.iter().enumerate() {
        let area = region.0 * region.1;
        let mut min_area = 0;
        let mut max_area = 0;
        for (j, &shape) in region_shapes[i].iter().enumerate() {
            min_area += shape * densities[j];
            max_area += shape * 9;
        }

        if area < min_area {
            impossible += 1;
        } else if area >= max_area {
            guaranteed += 1;
        } else {
            possible += 1;
        }
    }

    // Unbelievable!!!!
    // this problem is just pure evil
    println!(
        "guaranteed: {guaranteed}, possible: {possible}, impossible: {impossible}\nminimum: {guaranteed}, maximum: {}",
        guaranteed + possible
    );
}

pub fn part_two() {
    let input = fs::read_to_string("./src/input").unwrap();
}
