use itertools::Itertools;
use md5::{Digest, Md5};
use std::collections::HashMap;

fn main() {
    let answer_part_one = part_one("ffykfhsq");
    let answer_part_two = part_two("ffykfhsq");

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn part_one(s: &str) -> String {
    let mut pw: String = String::new();
    let mut i: u32 = 0;
    while pw.len() < 8 {
        let mut hasher = Md5::new();
        let mut comp: String = String::from(s);
        comp.push_str(&i.to_string());
        hasher.update(comp);
        let mut buf = [0u8; 128];
        let hash = hasher.finalize();
        let hex_result = base16ct::lower::encode_str(&hash, &mut buf).unwrap();

        if hex_result[..5] == *"00000" {
            pw.push(hex_result.chars().nth(5).unwrap());
            print!("at i = {}\npw = {}\nhash = {}\n\n", i, pw, hex_result,)
        }
        i += 1;
    }
    pw
}

fn part_two(s: &str) -> String {
    let mut pw: HashMap<u32, char> = HashMap::new();
    let mut i: u32 = 0;
    while pw.len() < 8 {
        let mut hasher = Md5::new();
        let mut comp: String = String::from(s);
        comp.push_str(&i.to_string());
        hasher.update(comp);
        let mut buf = [0u8; 128];
        let hash = hasher.finalize();
        let hex_result = base16ct::lower::encode_str(&hash, &mut buf).unwrap();

        if hex_result[..5] == *"00000" {
            let i_at_5 = hex_result.chars().nth(5).unwrap().to_digit(16).unwrap();
            if i_at_5 < 8 {
                match pw.get(&i_at_5) {
                    None => {
                        pw.insert(i_at_5, hex_result.chars().nth(6).unwrap());
                    }
                    Some(_) => (),
                }
            }
            print!("at i = {}\npw = {:#?}\nhash = {}\n\n", i, pw, hex_result,)
        }
        i += 1;
    }
    pw.keys().sorted().fold(String::new(), |mut acc, k| {
        acc.push(pw[k]);
        acc
    })
}
