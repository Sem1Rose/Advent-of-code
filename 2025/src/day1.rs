use std::{fs, ops::AddAssign};

#[derive(Debug, Clone, Copy)]
struct u100(i32);
impl AddAssign for u100 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        while self.0 >= 100 {
            self.0 -= 100;
        }
        while self.0 < 0 {
            self.0 += 100;
        }
    }
}
// u100(0)+u100(-5)==u100(95), 1
// u100(0)+u100(872)==u100(72), 7
impl u100 {
    pub fn add_check0(&mut self, rhs: Self) -> i32 {
        let mut zeros = 0;

        let mut x = rhs.0;
        while x.abs() > 100 {
            x -= 100 * x.signum();
            zeros += 1;
        }
        let was_zero = self.0 == 0;
        self.0 += x;
        // self.0 += rhs.0;
        // if self.0 - rhs.0 == 0 && self.0 < 0 {
        //     zeros -= 1;
        // }
        // if self.0 == 0 {
        //     zeros += 1;
        // }
        // // zeros += (self.0 / 100).abs();
        if !was_zero && self.0 == 0 {
            zeros += 1;
        }
        if self.0 >= 100 {
            self.0 -= 100;
            zeros += 1;
        } else if self.0 < 0 {
            if !was_zero {
                zeros += 1;
            }
            self.0 += 100;
        }

        // zeros.max(0)
        zeros
    }
}

pub fn part_one() {
    let input = fs::read_to_string("./src/input").unwrap();
    let mut dial = u100(50);

    let instructions = input
        .lines()
        .map(|instruction| {
            u100(
                instruction[1..].parse::<i32>().unwrap()
                    * if instruction.chars().nth(0).unwrap() == 'L' {
                        -1
                    } else {
                        1
                    },
            )
        })
        .collect::<Vec<_>>();

    let mut pass = 0;
    for instruction in instructions {
        println!("{:?} {:?}", dial, instruction);
        dial += instruction;
        if dial.0 == 0 {
            pass += 1;
        }
    }

    println!("{pass}");
}

pub fn part_two() {
    let input = fs::read_to_string("./src/input").unwrap();
    let mut dial = u100(50);

    let instructions = input
        .lines()
        .map(|instruction| {
            u100(
                instruction[1..].parse::<i32>().unwrap()
                    * if instruction.chars().nth(0).unwrap() == 'L' {
                        -1
                    } else {
                        1
                    },
            )
        })
        .collect::<Vec<_>>();

    let mut pass = 0;
    for instruction in instructions {
        print!("{:?}+{:?}=", dial, instruction);
        let p = dial.add_check0(instruction);
        println!("={:?}, {p}", dial);
        pass += p;
    }

    println!("{pass}");
}
