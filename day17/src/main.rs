use std::collections::VecDeque;

fn main() {
    let answer_part_one = part_one();
    let answer_part_two = part_two();

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn part_one() -> String {
    let passcode: String = String::from("veumntbg");
    let mut open_states: VecDeque<(String, i32, i32)> = VecDeque::new();
    open_states.push_back((String::new(), 0, 0));

    while let Some((s, x, y)) = open_states.pop_front() {
        if x == 3 && y == 3 {
            return s;
        }
        let hs: String = format!("{:x}", md5::compute(format!("{}{}", passcode, s)));
        for (e, (nx, ny)) in [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
            .iter()
            .enumerate()
        {
            if *nx >= 0
                && *nx <= 3
                && *ny >= 0
                && *ny <= 3
                && i32::from_str_radix(&hs.chars().nth(e).unwrap().to_string(), 16).unwrap() > 10
            {
                let mut ns = s.clone();
                match e {
                    0 => ns.push('U'),
                    1 => ns.push('D'),
                    2 => ns.push('L'),
                    3 => ns.push('R'),
                    _ => panic!("unpanicable"),
                };
                open_states.push_back((ns, *nx, *ny));
            }
        }
    }

    String::new()
}

fn part_two() -> usize {
    let passcode: String = String::from("veumntbg");
    let mut open_states: VecDeque<(String, i32, i32)> = VecDeque::new();
    open_states.push_back((String::new(), 0, 0));
    let mut lpl: usize = 0;
    let mut lpi: String = String::new();
    while let Some((s, x, y)) = open_states.pop_front() {
        if x == 3 && y == 3 {
            if s.len() > lpl {
                lpl = s.len();
                lpi = s;
            }
        } else {
            let hs: String = format!("{:x}", md5::compute(format!("{}{}", passcode, s)));
            for (e, (nx, ny)) in [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
                .iter()
                .enumerate()
            {
                if *nx >= 0
                    && *nx <= 3
                    && *ny >= 0
                    && *ny <= 3
                    && i32::from_str_radix(&hs.chars().nth(e).unwrap().to_string(), 16).unwrap()
                        > 10
                {
                    let mut ns = s.clone();
                    match e {
                        0 => ns.push('U'),
                        1 => ns.push('D'),
                        2 => ns.push('L'),
                        3 => ns.push('R'),
                        _ => panic!("unpanicable"),
                    };
                    open_states.push_back((ns, *nx, *ny));
                }
            }
        }
    }
    lpi.len()
}
