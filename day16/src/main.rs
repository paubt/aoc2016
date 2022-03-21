fn main() {
    let answer_part_one = part_one();
    let answer_part_two = part_two();

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn expand_dragon_style(mut a: Vec<bool>) -> Vec<bool> {
    let mut reverse_inverted: Vec<bool> = a.iter().cloned().rev().map(|x| !x).collect();
    a.push(false);
    let t: Vec<bool> = a.iter().chain(reverse_inverted.iter()).copied().collect();

    t
}

fn calc_check_sum(a: Vec<bool>) -> Vec<bool> {
    let mut cs: Vec<bool> = vec![];
    let mut i = a.iter();
    while let Some(x) = i.next() {
        match i.next() {
            Some(y) => cs.push(!(*x ^ *y)),
            None => return cs,
        }
    }
    cs
}

fn red_with_cs(a: Vec<bool>) -> Vec<bool> {
    let cs = calc_check_sum(a);
    if cs.len() % 2 == 0 {
        return red_with_cs(cs);
    } else {
        return cs;
    }
}

fn expand_and_reduce(mut is: Vec<bool>, size: usize) -> Vec<bool> {
    while is.len() < size {
        is = expand_dragon_style(is);
    }
    let cs: Vec<bool> = is.iter().take(size).cloned().collect();
    let r_cs = red_with_cs(cs);
    r_cs
}

fn part_one() -> String {
    let is: String = String::from("01111001100111011");
    let is: Vec<bool> = is
        .chars()
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => panic!("lel"),
        })
        .collect();
    expand_and_reduce(is, 272)
        .iter()
        .fold(String::new(), |mut acc, x| {
            match x {
                true => acc.push('1'),
                false => acc.push('0'),
            };
            acc
        })
}

fn part_two() -> String {
    let is: String = String::from("01111001100111011");
    let is: Vec<bool> = is
        .chars()
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => panic!("lel"),
        })
        .collect();
    expand_and_reduce(is, 35651584)
        .iter()
        .fold(String::new(), |mut acc, x| {
            match x {
                true => acc.push('1'),
                false => acc.push('0'),
            };
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exp_dragon_s1() {
        assert_eq!(vec![true, false, false], expand_dragon_style(vec![true]));
    }
    #[test]
    fn exp_dragon_s2() {
        assert_eq!(vec![false, false, true], expand_dragon_style(vec![false]));
    }
    #[test]
    fn exp_dragon_s3() {
        let t = assert_eq!(
            vec![true, true, true, true, true, false, false, false, false, false, false],
            expand_dragon_style(vec![true, true, true, true, true])
        );
    }
    #[test]
    fn calc_cs1() {
        assert_eq!(
            vec![true, true, false, true, false, true],
            calc_check_sum(vec![
                true, true, false, false, true, false, true, true, false, true, false, false
            ])
        );
    }

    #[test]
    fn red_cs1() {
        assert_eq!(
            vec![true, false, false],
            red_with_cs(vec![
                true, true, false, false, true, false, true, true, false, true, false, false
            ])
        );
    }
    #[test]
    fn rae1() {
        assert_eq!(
            vec![false, true, true, false, false],
            expand_and_reduce(vec![true, false, false, false, false], 20)
        );
    }
}
