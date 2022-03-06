use itertools::Itertools;
use std::fs;

fn main() {
    let instructions = fs::read_to_string("../data/day3/input.txt").expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "answer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn part_one(instructions: &str) -> u32 {
    instructions.lines().fold(0, |mut acc, x| {
        let s: Vec<u32> = x
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        if ((s[0] + s[1]) > s[2]) && ((s[1] + s[2]) > s[0]) && ((s[2] + s[0]) > s[1]) {
            acc += 1;
        }
        acc
    })
}

fn part_two(instructions: &str) -> u32 {
    instructions
        .lines()
        .chunks(3)
        .into_iter()
        .fold(0, |mut acc, x| {
            let x = x
                .into_iter()
                .map(|l| {
                    l.split(' ')
                        .filter(|e| !e.is_empty())
                        .map(|e| e.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>();
            // check triangle first to third column
            if ((x[0][0] + x[1][0]) > x[2][0])
                && (x[1][0] + x[2][0] > x[0][0])
                && (x[0][0] + x[2][0] > x[1][0])
            {
                acc += 1;
            }
            if ((x[0][1] + x[1][1]) > x[2][1])
                && (x[1][1] + x[2][1] > x[0][1])
                && (x[0][1] + x[2][1] > x[1][1])
            {
                acc += 1;
            }
            if ((x[0][2] + x[1][2]) > x[2][2])
                && (x[1][2] + x[2][2] > x[0][2])
                && (x[0][2] + x[2][2] > x[1][2])
            {
                acc += 1;
            }
            acc
        })
}
