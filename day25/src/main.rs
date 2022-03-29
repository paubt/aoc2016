use std::fs;

fn main() {
    let instructions = fs::read_to_string("../data/day25/input.txt").expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn exec_inst(instructions: &str, a_init_v: i32, check_lenght: usize) -> Vec<i32> {
    let instructions: Vec<Vec<&str>> = instructions
        .lines()
        .map(|line| line.split(" ").collect())
        .collect();
    let mut a: i32 = a_init_v;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut d: i32 = 0;
    let mut pc: i32 = 0;
    let mut clock_signal: Vec<i32> = Vec::new();
    loop {
        if clock_signal.len() >= check_lenght {
            break;
        }
        match instructions.iter().nth(pc as usize) {
            Some(i) => match i.first() {
                Some(kw) => match *kw {
                    "inc" => match *i.iter().nth(1).unwrap() {
                        "a" => a += 1,
                        "b" => b += 1,
                        "c" => c += 1,
                        "d" => d += 1,
                        _ => panic!("unknown register id inc"),
                    },
                    "dec" => match *i.iter().nth(1).unwrap() {
                        "a" => a -= 1,
                        "b" => b -= 1,
                        "c" => c -= 1,
                        "d" => d -= 1,
                        _ => panic!("unknown register id dec"),
                    },
                    "cpy" => {
                        let v: i32;
                        match *i.iter().nth(1).expect("no elemts found cpy") {
                            "a" => v = a,
                            "b" => v = b,
                            "c" => v = c,
                            "d" => v = d,

                            _ => {
                                v = i
                                    .iter()
                                    .nth(1)
                                    .expect("no elemts found cpy")
                                    .parse::<i32>()
                                    .expect("parse of value source failed ");
                            }
                        }
                        match *i.iter().nth(2).expect("no elemts found to target cpy") {
                            "a" => a = v,
                            "b" => b = v,
                            "c" => c = v,
                            "d" => d = v,
                            _ => panic!("invalid copy target"),
                        }
                    }
                    "jnz" => {
                        let jv: i32;
                        match *i.iter().nth(1).expect("no elemts found in jump condition") {
                            "a" => jv = a,
                            "b" => jv = b,
                            "c" => jv = c,
                            "d" => jv = d,

                            _ => {
                                jv = i
                                    .iter()
                                    .nth(1)
                                    .expect("no elemts 1 found in jump condition")
                                    .parse::<i32>()
                                    .expect("parse of jmp cond failed ");
                            }
                        }
                        if jv != 0 {
                            let js = i
                                .iter()
                                .nth(2)
                                .expect("no elemts 2 found in jump size")
                                .parse::<i32>()
                                .expect("parse of jmp size failed ");
                            if (pc + js) < 0 {
                                break;
                            } else {
                                pc += js - 1;
                            }
                        }
                    }
                    "out" => {
                        let jv: i32;
                        match *i.iter().nth(1).expect("out arg fail") {
                            "a" => jv = a,
                            "b" => jv = b,
                            "c" => jv = c,
                            "d" => jv = d,

                            _ => {
                                jv = i
                                    .iter()
                                    .nth(1)
                                    .expect("out arg fail")
                                    .parse::<i32>()
                                    .expect("parse of out arg failed ");
                            }
                        }
                        clock_signal.push(jv);
                    }
                    _ => panic!("unkown instruction"),
                },
                None => panic!("instruction has no keyword"),
            },
            None => break,
        }
        pc += 1;
    }
    clock_signal.iter().for_each(|x| print!("{} ", x));
    println!();
    clock_signal
}

fn brute_force(instructions: &str, check_lenght: usize) -> usize {
    let mut a: i32 = 0;
    loop {
        let res: Vec<i32> = exec_inst(instructions, a, check_lenght);

        let mut last = 1;
        let mut premature_end = false;
        for x in &res {
            if (*x == 0 || *x == 1) && *x != last {
                last = *x;
            } else {
                premature_end = true;
                break;
            }
        }
        if premature_end {
            println!("pattern not found");
            res.iter().for_each(|x| print!("{} ", x));
            println!();
        } else {
            println!("pattern found with a={}", a);

            res.iter().for_each(|x| print!("{} ", x));
            println!();
            break;
        }

        if a > 1000 {
            break;
        }
        a += 1;
    }
    0
}
fn part_one(instructions: &str) -> usize {
    brute_force(instructions, 45)
}
fn part_two(instructions: &str) -> i32 {
    0
    //exec_inst(instructions, 1)
}
