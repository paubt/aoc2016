use std::fs;

fn main() {
    let instructions = fs::read_to_string("../data/day2/input.txt").expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "answer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn part_one(instructions: &str) -> u32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut code: String = String::new();
    for c in instructions.chars() {
        match c {
            'U' => {
                if y < 1 {
                    y += 1
                }
            }
            'D' => {
                if y > -1 {
                    y -= 1
                }
            }
            'L' => {
                if x > -1 {
                    x -= 1
                }
            }
            'R' => {
                if x < 1 {
                    x += 1
                }
            }
            '\n' => match (x, y) {
                (-1, 1) => code.push('1'),
                (0, 1) => code.push('2'),
                (1, 1) => code.push('3'),
                (-1, 0) => code.push('4'),
                (0, 0) => code.push('5'),
                (1, 0) => code.push('6'),
                (-1, -1) => code.push('7'),
                (0, -1) => code.push('8'),
                (1, -1) => code.push('9'),
                _ => panic!("position should be possible"),
            },
            _ => panic!("unexpected char"),
        }
    }
    code.parse::<u32>().unwrap()
}

fn part_two(instructions: &str) -> String {
    let mut x: i32 = -2;
    let mut y: i32 = 0;
    let mut code: String = String::new();
    for c in instructions.chars() {
        match c {
            'U' => {
                if x == 0 {
                    if y < 2 {
                        y += 1;
                    }
                } else if (x == 1) || (x == -1) {
                    if y < 1 {
                        y += 1;
                    }
                }
            }
            'D' => {
                if x == 0 {
                    if y > -2 {
                        y -= 1;
                    }
                } else if (x == 1) || (x == -1) {
                    if y > -1 {
                        y -= 1;
                    }
                }
            }
            'L' => {
                if y == 0 {
                    if x > -2 {
                        x -= 1;
                    }
                } else if (y == 1) || (y == -1) {
                    if x > -1 {
                        x -= 1;
                    }
                }
            }
            'R' => {
                if y == 0 {
                    if x < 2 {
                        x += 1;
                    }
                } else if (y == 1) || (y == -1) {
                    if x < 1 {
                        x += 1;
                    }
                }
            }
            '\n' => match (x, y) {
                (0, 2) => code.push('1'),
                (-1, 1) => code.push('2'),
                (0, 1) => code.push('3'),
                (1, 1) => code.push('4'),
                (-2, 0) => code.push('5'),
                (-1, 0) => code.push('6'),
                (0, 0) => code.push('7'),
                (1, 0) => code.push('8'),
                (2, 0) => code.push('9'),
                (-1, -1) => code.push('A'),
                (0, -1) => code.push('B'),
                (1, -1) => code.push('C'),
                (0, -2) => code.push('D'),
                _ => panic!("position should be possible"),
            },
            _ => panic!("unexpected char"),
        }
    }
    code
}
