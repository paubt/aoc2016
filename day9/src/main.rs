use evalexpr::*;
use itertools::Itertools;
use std::fs;
use std::time::{Duration, Instant};

fn main() {
    let instructions = fs::read_to_string("../data/day9/input.txt").expect("read in file failed");
    let now = Instant::now();
    let answer_part_one = part_one(&instructions);
    let time_part_one = now.elapsed().as_millis();
    let now = Instant::now();
    let answer_part_two = part_two(&instructions);
    let time_part_two = now.elapsed().as_millis();
    println!(
        "\nanswer part 1: {} with {} ms needed\nanswer part 2: {} with {} ms needed",
        answer_part_one, time_part_one, answer_part_two, time_part_two
    );

    /*
    let instructions = fs::read_to_string("../data/day9/input.txt").expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
    */
}

fn part_one(instructions: &str) -> u32 {
    let mut counter: usize = 0;
    let mut iter_chars = instructions.chars();
    let mut skip_length_left: usize = 0;
    while let Some(c) = iter_chars.next() {
        if skip_length_left > 0 {
            skip_length_left -= 1;
        } else if c == '(' {
            // read numbers (chars) until hiting
            let marker: String = iter_chars
                .clone()
                .take_while(|c| *c != ')')
                .collect::<String>();

            //print!("marker = {}\n", marker);
            let (sub_s, reps) = marker
                .split('x')
                .map(|x| x.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap();
            counter += sub_s * reps;
            //print!("substring to add = {}\n", s);
            skip_length_left = sub_s + marker.len() + 1;
        } else {
            counter += 1;
        }
    }

    //print!("\n{}", decompressed);
    counter as u32 - 1
}

fn rec_decomp(instructions: &str) -> String {
    let mut decompressed: Vec<String> = Vec::new();
    let mut iter_chars = instructions.chars();
    let mut skip_length_left: usize = 0;
    let mut counter: usize = 0;

    while let Some(c) = iter_chars.next() {
        if skip_length_left > 0 {
            skip_length_left -= 1;
        } else if c == '(' {
            // read numbers (chars) until hiting
            let i = iter_chars.clone();
            let marker: String = i.take_while(|c| *c != ')').collect();
            //print!("marker = {}\n", marker);
            let (sub_s, reps) = marker
                .split('x')
                .map(|x| x.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap();
            let substring: String = iter_chars
                .clone()
                .skip(marker.len() + 1)
                .take(sub_s)
                .collect();
            let t = format!("{}*({})", reps, rec_decomp(&substring));
            decompressed.push(t);
            //print!("substring to add = {}\n", s);
            skip_length_left = sub_s + marker.len() + 1;
        } else {
            //decompressed.push(c.to_string());
            decompressed.push("1".to_string());
            counter += 1;
        }
    }
    let decompressed: String =
        decompressed
            .iter()
            .intersperse(&"+".to_string())
            .fold(String::new(), |mut acc, s| {
                acc.push_str(s);
                acc
            });
    //print!("{}\n", decompressed);
    decompressed
}

fn part_two(instructions: &str) -> u32 {
    // turn into parsable equation
    let t = rec_decomp(&instructions[..instructions.len() - 1]);
    print!("{}\n", t);
    // evaluate equation
    let e = eval_int(&t).unwrap();
    // print!("{}\n", e);
    e as u32
}
