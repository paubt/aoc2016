use std::fs;

fn main() {
    let instructions = fs::read_to_string("../data/day21/input.txt").expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn swap_position(x: usize, y: usize, mut s: Vec<char>) -> Vec<char> {
    s.swap(x, y);
    s
}

fn swap_letter(l1: char, l2: char, mut s: Vec<char>) -> Vec<char> {
    for c in s.iter_mut() {
        if *c == l1 {
            *c = l2;
        } else if *c == l2 {
            *c = l1;
        }
    }
    s
}

// direction: true==left and false==right
fn rotate_lr_x_pos(direction: bool, amount: usize, s: Vec<char>) -> Vec<char> {
    let amount = amount % s.len();
    // left
    if direction {
        s.iter()
            .skip(amount)
            .chain(s.iter().take(amount))
            .copied()
            .collect()
    }
    // right
    else {
        s.iter()
            .skip(s.len() - amount)
            .chain(s.iter().take(s.len() - amount))
            .copied()
            .collect()
    }
}

fn rev_rotate_lr_x_pos(direction: bool, amount: usize, s: Vec<char>) -> Vec<char> {
    rotate_lr_x_pos(!direction, amount, s)
}

fn rotate_b_pos_l(l: char, s: Vec<char>) -> Vec<char> {
    let (index, _): (usize, &char) = s.iter().enumerate().find(|(_, c)| **c == l).unwrap();
    let mut amount = index + 1;
    if index >= 4 {
        amount = (amount + 1) % s.len();
    }
    rotate_lr_x_pos(false, amount, s)
}

fn rev_rotate_b_pos_l(l: char, s: Vec<char>) -> Vec<char> {
    let (index, _): (usize, &char) = s.iter().enumerate().find(|(_, c)| **c == l).unwrap();
    if index == 0 {
        rotate_lr_x_pos(true, s.len() + 1, s)
    } else if index % 2 == 0 {
        rotate_lr_x_pos(true, 6 + ((index / 2) - 1), s)
    } else {
        rotate_lr_x_pos(true, (index / 2) + 1, s)
    }
}

fn reverse_positions(from: usize, to: usize, s: Vec<char>) -> Vec<char> {
    //note that x should be lower or equal to y
    if from > to {
        let t = from;
        let from = to;
        let to = from;
    }
    let to = to + 1;
    assert!(from <= to);
    s.iter()
        .take(from)
        .chain(
            s.iter()
                .skip(from)
                .take(to - from)
                .rev()
                .chain(s.iter().skip(to)),
        )
        .copied()
        .collect()
}

fn move_pos_x_to_y(x: usize, y: usize, mut s: Vec<char>) -> Vec<char> {
    if y == x {
        s
    } else {
        let c = s.remove(x);
        s.insert(y, c);
        s
    }
}

fn part_one(instrucions: &str) -> String {
    let mut is: Vec<char> = "abcdefgh".chars().collect();

    for instrucion in instrucions.lines() {
        //is.iter().for_each(|c| print!("{}", c));
        let instrucion: Vec<&str> = instrucion.split(' ').collect();
        match (instrucion[0], instrucion[1]) {
            ("swap", "position") => {
                is = swap_position(
                    instrucion[2].parse::<usize>().unwrap(),
                    instrucion[5].parse::<usize>().unwrap(),
                    is,
                )
            }
            ("swap", "letter") => {
                is = swap_letter(
                    instrucion[2].parse::<char>().unwrap(),
                    instrucion[5].parse::<char>().unwrap(),
                    is,
                )
            }
            ("reverse", _) => {
                is = reverse_positions(
                    instrucion[2].parse::<usize>().unwrap(),
                    instrucion[4].parse::<usize>().unwrap(),
                    is,
                )
            }
            ("rotate", "left") => {
                is = rotate_lr_x_pos(true, instrucion[2].parse::<usize>().unwrap(), is)
            }
            ("rotate", "right") => {
                is = rotate_lr_x_pos(false, instrucion[2].parse::<usize>().unwrap(), is)
            }

            ("rotate", "based") => is = rotate_b_pos_l(instrucion[6].parse::<char>().unwrap(), is),

            ("move", _) => {
                is = move_pos_x_to_y(
                    instrucion[2].parse::<usize>().unwrap(),
                    instrucion[5].parse::<usize>().unwrap(),
                    is,
                )
            }
            _ => (),
        }
        //print!(" -> ");
        //is.iter().for_each(|c| print!("{}", c));
        //println!();
    }
    is.iter().fold(String::new(), |mut acc, c| {
        acc.push(*c);
        acc
    })
}

fn part_two(instrucions: &str) -> String {
    let mut is: Vec<char> = "fbgdceah".chars().collect();

    for instrucion in instrucions.lines().rev() {
        //is.iter().for_each(|c| print!("{}", c));
        let instrucion: Vec<&str> = instrucion.split(' ').collect();
        match (instrucion[0], instrucion[1]) {
            ("swap", "position") => {
                is = swap_position(
                    instrucion[2].parse::<usize>().unwrap(),
                    instrucion[5].parse::<usize>().unwrap(),
                    is,
                )
            }
            ("swap", "letter") => {
                is = swap_letter(
                    instrucion[2].parse::<char>().unwrap(),
                    instrucion[5].parse::<char>().unwrap(),
                    is,
                )
            }
            ("reverse", _) => {
                is = reverse_positions(
                    instrucion[2].parse::<usize>().unwrap(),
                    instrucion[4].parse::<usize>().unwrap(),
                    is,
                )
            }
            ("rotate", "left") => {
                is = rev_rotate_lr_x_pos(true, instrucion[2].parse::<usize>().unwrap(), is)
            }
            ("rotate", "right") => {
                is = rev_rotate_lr_x_pos(false, instrucion[2].parse::<usize>().unwrap(), is)
            }

            ("rotate", "based") => {
                is = rev_rotate_b_pos_l(instrucion[6].parse::<char>().unwrap(), is)
            }

            ("move", _) => {
                is = move_pos_x_to_y(
                    instrucion[5].parse::<usize>().unwrap(),
                    instrucion[2].parse::<usize>().unwrap(),
                    is,
                )
            }
            _ => (),
        }
        //print!(" -> ");
        //is.iter().for_each(|c| print!("{}", c));
        //println!();
    }
    is.iter().fold(String::new(), |mut acc, c| {
        acc.push(*c);
        acc
    })
}

#[cfg(test)]
mod tests {
    mod reverse {
        use super::super::*;

        #[test]
        fn rotate_based_on_xright1() {
            assert_eq!(
                String::from("abcd").chars().collect::<Vec<char>>(),
                rev_rotate_lr_x_pos(
                    false,
                    1,
                    String::from("dabc").chars().collect::<Vec<char>>()
                )
            );
        }
        #[test]
        fn rotate_based_on_xleft1() {
            assert_eq!(
                String::from("abcd").chars().collect::<Vec<char>>(),
                rev_rotate_lr_x_pos(true, 2, String::from("cdab").chars().collect::<Vec<char>>())
            );
        }

        #[test]
        fn rev_rotate_based_on_letter1() {
            assert_eq!(
                String::from("abdec").chars().collect::<Vec<char>>(),
                rev_rotate_b_pos_l('b', String::from("ecabd").chars().collect::<Vec<char>>())
            );
        }

        #[test]
        fn rev_rotate_based_on_letter2() {
            assert_eq!(
                String::from("abcdefgh").chars().collect::<Vec<char>>(),
                rev_rotate_b_pos_l('h', String::from("habcdefg").chars().collect::<Vec<char>>())
            );
        }
    }

    mod inorder {
        use super::super::*;

        #[test]
        fn swap_pos_1() {
            assert_eq!(
                String::from("adcbe").chars().collect::<Vec<char>>(),
                swap_position(1, 3, String::from("abcde").chars().collect::<Vec<char>>())
            );
        }
        #[test]
        fn swap_letter_1() {
            assert_eq!(
                String::from("accbcdb").chars().collect::<Vec<char>>(),
                swap_letter(
                    'b',
                    'c',
                    String::from("abbcbdc").chars().collect::<Vec<char>>()
                )
            );
        }
        #[test]
        fn rotate_based_on_xright1() {
            assert_eq!(
                String::from("dabc").chars().collect::<Vec<char>>(),
                rotate_lr_x_pos(
                    false,
                    1,
                    String::from("abcd").chars().collect::<Vec<char>>()
                )
            );
        }
        #[test]
        fn rotate_based_on_xleft1() {
            assert_eq!(
                String::from("cdab").chars().collect::<Vec<char>>(),
                rotate_lr_x_pos(true, 2, String::from("abcd").chars().collect::<Vec<char>>())
            );
        }
        #[test]
        fn rotate_based_on_letter1() {
            assert_eq!(
                String::from("ecabd").chars().collect::<Vec<char>>(),
                rotate_b_pos_l('b', String::from("abdec").chars().collect::<Vec<char>>())
            );
        }

        #[test]
        fn rotate_based_on_letter2() {
            assert_eq!(
                String::from("decab").chars().collect::<Vec<char>>(),
                rotate_b_pos_l('d', String::from("ecabd").chars().collect::<Vec<char>>())
            );
        }

        #[test]
        fn reverse_pos_1() {
            assert_eq!(
                String::from("abcde").chars().collect::<Vec<char>>(),
                reverse_positions(0, 4, String::from("edcba").chars().collect::<Vec<char>>())
            );
        }
        #[test]
        fn reverse_pos_2() {
            assert_eq!(
                String::from("abedc").chars().collect::<Vec<char>>(),
                reverse_positions(2, 4, String::from("abcde").chars().collect::<Vec<char>>())
            );
        }
        #[test]
        fn move_pos1() {
            assert_eq!(
                String::from("bdeac").chars().collect::<Vec<char>>(),
                move_pos_x_to_y(1, 4, String::from("bcdea").chars().collect::<Vec<char>>())
            );
        }
        #[test]
        fn move_pos2() {
            assert_eq!(
                String::from("abdec").chars().collect::<Vec<char>>(),
                move_pos_x_to_y(3, 0, String::from("bdeac").chars().collect::<Vec<char>>())
            );
        }

        #[test]
        fn move_pos3() {
            assert_eq!(
                String::from("bdeac").chars().collect::<Vec<char>>(),
                move_pos_x_to_y(2, 2, String::from("bdeac").chars().collect::<Vec<char>>())
            );
        }
    }
}
