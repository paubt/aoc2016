use std::collections::HashMap;
use std::fs;

fn main() {
    let instructions = fs::read_to_string("../data/day4/input.txt").expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn part_one(instructions: &str) -> u32 {
    let sum_id: u32 = instructions.lines().fold(0, |mut acc, l| {
        let (name, id_check) = l.rsplit_once('-').unwrap();
        let t: Vec<&str> = id_check
            .split(|c| c == '[' || c == ']')
            .collect::<Vec<&str>>();
        let mut letter: Vec<(char, u32)> = name
            .chars()
            .filter(|c| *c != '-')
            .fold(HashMap::new(), |mut acc, c| match acc.get_mut(&c) {
                Some(count) => {
                    *count = *count + 1;
                    return acc;
                }
                _ => {
                    acc.insert(c, 1);
                    return acc;
                }
            })
            .iter()
            .fold(Vec::new(), |mut acc, (k, v)| {
                acc.push((*k, *v));
                acc
            });
        // sort first by value and second lexographical by key
        letter.sort_by(
            |(k1, v1), (k2, v2)| {
                if v1 == v2 {
                    k1.cmp(k2)
                } else {
                    v2.cmp(v1)
                }
            },
        );
        let check_sum_is = letter
            .iter()
            .take(5)
            .fold(String::new(), |mut acc, (k, _)| {
                acc.push(*k);
                acc
            });
        if t[1] == check_sum_is {
            acc += t[0].parse::<u32>().unwrap();
        }
        acc
    });
    sum_id
}

fn part_two(instructions: &str) -> u32 {
    let sum_id: u32 = instructions.lines().fold(0, |mut acc, l| {
        let (name, id_check) = l.rsplit_once('-').unwrap();
        let t: Vec<&str> = id_check
            .split(|c| c == '[' || c == ']')
            .collect::<Vec<&str>>();
        let mut letter: Vec<(char, u32)> = name
            .chars()
            .filter(|c| *c != '-')
            .fold(HashMap::new(), |mut acc, c| match acc.get_mut(&c) {
                Some(count) => {
                    *count = *count + 1;
                    return acc;
                }
                _ => {
                    acc.insert(c, 1);
                    return acc;
                }
            })
            .iter()
            .fold(Vec::new(), |mut acc, (k, v)| {
                acc.push((*k, *v));
                acc
            });
        // sort first by value and second lexographical by key
        letter.sort_by(
            |(k1, v1), (k2, v2)| {
                if v1 == v2 {
                    k1.cmp(k2)
                } else {
                    v2.cmp(v1)
                }
            },
        );
        let check_sum_is = letter
            .iter()
            .take(5)
            .fold(String::new(), |mut acc, (k, _)| {
                acc.push(*k);
                acc
            });
        if t[1] == check_sum_is {
            acc += t[0].parse::<u32>().unwrap();
        }
        acc
    });
    sum_id
}
