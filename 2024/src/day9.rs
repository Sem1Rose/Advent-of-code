use std::fs::read_to_string;

#[derive(Clone, Copy, PartialEq)]
struct DiskFile {
    free_space: bool,
    id: u128,
    size: u128,
}

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let mut disk_map = vec![];

    let mut n = false;
    let mut id = 0;
    for i in input
        .chars()
        .map(|x| x.to_string().parse::<u128>().unwrap())
    {
        disk_map.push(DiskFile {
            free_space: n,
            id,
            size: i,
        });
        if n {
            id += 1;
        }
        n = !n;
    }
    if disk_map.last().unwrap().free_space {
        disk_map.remove(disk_map.len() - 1);
    }

    // for i in disk_map.clone() {
    //     print!(
    //         "{}",
    //         (if i.free_space {
    //             ".".to_string()
    //         } else {
    //             i.id.to_string()
    //         })
    //         .repeat(i.size as usize)
    //     );
    // }
    // println!();

    let mut free_space_index_check = disk_map.iter().position(|x| x.free_space);
    while free_space_index_check.is_some() {
        let mut free_space_index = free_space_index_check.unwrap() + 1;
        let mut last_file_index = disk_map.iter().rposition(|x| !x.free_space).unwrap() + 1;
        let mut new_file_index = free_space_index - 1;

        if last_file_index <= free_space_index {
            disk_map.remove(free_space_index - 1);
            break;
        }

        if free_space_index > 1
            && disk_map[free_space_index - 1].id == disk_map[last_file_index - 1].id
        {
            free_space_index -= 1;
            last_file_index -= 1;
            new_file_index -= 1;
        } else {
            disk_map.insert(
                new_file_index,
                DiskFile {
                    free_space: false,
                    id: disk_map[last_file_index - 1].id,
                    size: 0,
                },
            );
        }

        while disk_map[free_space_index].size > 0 {
            if disk_map[last_file_index].size == 0 {
                disk_map.remove(last_file_index);

                last_file_index = disk_map.iter().rposition(|x| !x.free_space).unwrap() + 1;
                new_file_index = free_space_index;
                free_space_index += 1;

                if last_file_index <= free_space_index {
                    free_space_index -= 1;
                    break;
                }
                disk_map.insert(
                    new_file_index,
                    DiskFile {
                        free_space: false,
                        id: disk_map[last_file_index - 1].id,
                        size: 0,
                    },
                );
            }

            disk_map[new_file_index].size += 1;
            disk_map[free_space_index].size -= 1;
            disk_map[last_file_index].size -= 1;

            // println!(
            //     "{}{}: {}\n{}{}: {}\n{}{}: {}\n",
            //     if disk_map[free_space_index].free_space {
            //         "-"
            //     } else {
            //         "+"
            //     },
            //     free_space_index,
            //     disk_map[free_space_index]
            //         .id
            //         .to_string()
            //         .repeat(disk_map[free_space_index].size as usize),
            //     if disk_map[last_file_index].free_space {
            //         "-"
            //     } else {
            //         "+"
            //     },
            //     last_file_index,
            //     disk_map[last_file_index]
            //         .id
            //         .to_string()
            //         .repeat(disk_map[last_file_index].size as usize),
            //     if disk_map[new_file_index].free_space {
            //         "-"
            //     } else {
            //         "+"
            //     },
            //     new_file_index,
            //     disk_map[new_file_index]
            //         .id
            //         .to_string()
            //         .repeat(disk_map[new_file_index].size as usize),
            // );
        }
        disk_map.remove(free_space_index);
        if disk_map.last().unwrap().free_space {
            disk_map.remove(disk_map.len() - 1);
        }

        free_space_index_check = disk_map.iter().position(|x| x.free_space);
    }

    // for i in disk_map {
    //     print!(
    //         "{}",
    //         (if i.free_space {
    //             ".".to_string()
    //         } else {
    //             i.id.to_string()
    //         })
    //         .repeat(i.size as usize)
    //     );
    // }
    // println!();

    let mut sum = 0;
    let mut index = 0;
    (0..disk_map.len()).for_each(|i| {
        for _ in 0..disk_map[i].size {
            sum += disk_map[i].id * index;
            index += 1;
        }
    });

    println!("{sum}")
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let mut disk_map = vec![];

    let mut n = false;
    let mut id = 0;
    for i in input
        .chars()
        .map(|x| x.to_string().parse::<u128>().unwrap())
    {
        disk_map.push(DiskFile {
            free_space: n,
            id,
            size: i,
        });
        if n {
            id += 1;
        }
        n = !n;
    }
    while disk_map.last().unwrap().free_space {
        disk_map.remove(disk_map.len() - 1);
    }
    while disk_map.iter().any(|x| x.size == 0) {
        disk_map.remove(disk_map.iter().position(|x| x.size == 0).unwrap());
    }

    // for i in disk_map.clone() {
    //     print!(
    //         "{}",
    //         (if i.free_space {
    //             ".".to_string()
    //         } else {
    //             i.id.to_string()
    //         })
    //         .repeat(i.size as usize)
    //     );
    // }
    // println!("\n");

    (1..=(disk_map.iter().rfind(|x| !x.free_space).unwrap().id))
        .rev()
        .for_each(|id| {
            let index = disk_map
                .iter()
                .position(|x| !x.free_space && x.id == id)
                .unwrap();
            let size = disk_map[index].size;

            // println!("{}: {} {}", id, index, size);

            for j in 0..index {
                if disk_map[j].free_space && disk_map[j].size >= size {
                    disk_map[j].size -= size;

                    disk_map.insert(
                        j,
                        DiskFile {
                            free_space: false,
                            id,
                            size,
                        },
                    );
                    disk_map.insert(
                        index + 1,
                        DiskFile {
                            free_space: true,
                            id: 0,
                            size,
                        },
                    );
                    disk_map.remove(index + 2);

                    if disk_map[j + 1].size == 0 {
                        disk_map.remove(j + 1);
                    }

                    // while disk_map.last().unwrap().free_space {
                    //     disk_map.remove(disk_map.len() - 1);
                    // }
                    break;
                }
            }
            // for k in disk_map.clone() {
            //     print!(
            //         "{}",
            //         (if k.free_space {
            //             ".".to_string()
            //         } else {
            //             k.id.to_string()
            //         })
            //         .repeat(k.size as usize)
            //     );
            // }
            // println!();
        });

    // for i in disk_map {
    //     print!(
    //         "{}",
    //         (if i.free_space {
    //             ".".to_string()
    //         } else {
    //             i.id.to_string()
    //         })
    //         .repeat(i.size as usize)
    //     );
    // }
    // println!();

    let mut sum = 0;
    let mut index = 0;
    (0..disk_map.len()).for_each(|i| {
        for _ in 0..disk_map[i].size {
            if !disk_map[i].free_space {
                sum += disk_map[i].id * index;
            }
            index += 1;
        }
    });

    println!("{sum}")
}
