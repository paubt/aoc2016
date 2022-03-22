use itertools::Itertools;
use std::fs;

fn main() {
    let instructions = fs::read_to_string("../data/day20/input.txt").expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}
fn part_one(s: &str) -> u32 {
    let mut iv: Vec<(u32, u32)> = s
        .lines()
        .map(|l| {
            l.split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    iv.sort_by(|(a, _), (b, _)| a.cmp(b));
    //iv.iter().for_each(|(a, b)| println!("{} - {}", a, b));

    let mut furthest_last_intervall_end: u32 = 0;
    for (start, end) in iv.iter() {
        if furthest_last_intervall_end + 1 < *start {
            return furthest_last_intervall_end + 1;
        } else {
            if furthest_last_intervall_end < *end {
                furthest_last_intervall_end = *end;
            }
        }
    }
    panic!("no found")
}

fn part_two(s: &str) -> u64 {
    let mut iv: Vec<(u64, u64)> = s
        .lines()
        .map(|l| {
            l.split('-')
                .map(|x| x.parse::<u32>().unwrap() as u64)
                .collect_tuple()
                .unwrap()
        })
        .collect();
    iv.sort_by(|(a, _), (b, _)| a.cmp(b));
    //iv.iter().for_each(|(a, b)| println!("{} - {}", a, b));

    let mut furthest_last_intervall_end: u64 = 0;
    let mut allowed_ip_sum: u64 = 0;
    for (start, end) in iv.iter() {
        if furthest_last_intervall_end + 1 < *start {
            allowed_ip_sum += (*start - 1) - furthest_last_intervall_end;
            furthest_last_intervall_end = *end;
        } else {
            if furthest_last_intervall_end < *end {
                furthest_last_intervall_end = *end;
            }
        }
    }
    allowed_ip_sum
}
