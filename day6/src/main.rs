use std::collections::HashMap;
use std::fs;

fn main() {
    let instructions = fs::read_to_string("../data/day6/input.txt").expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn part_one(s: &str) -> String {
    let word_size = s.lines().next().unwrap().len();
    s.lines()
        .fold(vec![HashMap::new(); word_size], |mut acc, l| {
            for (i, c) in l.chars().enumerate() {
                match acc[i].get_mut(&c) {
                    Some(x) => *x += 1,
                    None => {
                        acc[i].insert(c, 1);
                    }
                };
            }
            acc
        })
        .iter()
        .fold(String::new(), |mut acc, hm| {
            let (k, _) = hm.iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();
            acc.push(*k);
            acc
        })
}

fn part_two(s: &str) -> String {
    let word_size = s.lines().next().unwrap().len();
    s.lines()
        .fold(vec![HashMap::new(); word_size], |mut acc, l| {
            for (i, c) in l.chars().enumerate() {
                match acc[i].get_mut(&c) {
                    Some(x) => *x += 1,
                    None => {
                        acc[i].insert(c, 1);
                    }
                };
            }
            acc
        })
        .iter()
        .fold(String::new(), |mut acc, hm| {
            let (k, _) = hm.iter().min_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();
            acc.push(*k);
            acc
        })
}
