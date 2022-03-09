use std::collections::HashSet;
use std::fs;

fn main() {
    let instructions = fs::read_to_string("../data/day7/input.txt").expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn part_one(s: &str) -> u32 {
    s.lines().fold(0, |mut acc, l| {
        let mut found: bool = false;

        for (i, part) in l.split(['[', ']']).enumerate() {
            let mut c_1 = '1';
            let mut c_2 = '2';
            let mut c_3 = '3';
            let mut f = false;
            for c in part.chars() {
                if c == c_3 && c_1 == c_2 && c_1 != c {
                    f = true;
                    break;
                }
                c_3 = c_2;
                c_2 = c_1;
                c_1 = c;
            }
            if f {
                if i % 2 == 0 {
                    found = true;
                } else {
                    found = false;
                    break;
                }
            }
        }
        if found {
            acc += 1;
            acc
        } else {
            acc
        }
    })
}

fn part_two(s: &str) -> u32 {
    s.lines().fold(0, |mut acc, l| {
        let mut aba_sn: HashSet<(char, char, char)> = HashSet::new();
        let mut bab_hn: HashSet<(char, char, char)> = HashSet::new();

        for (i, part) in l.split(['[', ']']).enumerate() {
            let mut c_1 = '1';
            let mut c_2 = '2';
            for c in part.chars() {
                if c == c_2 && c_1 != c {
                    // supernet -> aba
                    if i % 2 == 0 {
                        aba_sn.insert((c_2, c_1, c));
                    } else {
                        bab_hn.insert((c_2, c_1, c));
                    }
                }
                c_2 = c_1;
                c_1 = c;
            }
        }
        for (_, c_1, c) in aba_sn.iter() {
            match bab_hn.get(&(*c_1, *c, *c_1)) {
                Some(_) => {
                    acc += 1;
                    return acc;
                }
                None => (),
            }
        }
        acc
    })
}
