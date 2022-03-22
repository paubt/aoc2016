fn main() {
    let answer_part_one = part_one();
    let answer_part_two = part_two();

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn create_next_row(mut os: Vec<bool>) -> Vec<bool> {
    let mut ns: Vec<bool> = Vec::new();
    os.insert(0, false);
    os.push(false);
    for i in 1..os.len() - 1 {
        if (os[i - 1] && os[i] && !os[i + 1])
            || (!os[i - 1] && os[i] && os[i + 1])
            || (os[i - 1] && !os[i] && !os[i + 1])
            || (!os[i - 1] && !os[i] && os[i + 1])
        {
            ns.push(true);
        } else {
            ns.push(false);
        }
    }
    ns
}

fn part_one() -> u32 {
    // trap is true
    // no trap is false
    let is = ".^^^.^.^^^^^..^^^..^..^..^^..^.^.^.^^.^^....^.^...^.^^.^^.^^..^^..^.^..^^^.^^...^...^^....^^.^^^^^^^";
    let mut s: Vec<bool> = is
        .chars()
        .map(|c| match c {
            '.' => false,
            '^' => true,
            _ => panic!("unknown char in is"),
        })
        .collect();
    let mut stc: u32 = 0;
    for _ in 0..40 {
        stc += s.iter().fold(0, |acc, x| match x {
            true => acc,
            false => acc + 1,
        });
        s = create_next_row(s);
    }
    stc
}

fn part_two() -> u32 {
    // trap is true
    // no trap is false
    let is = ".^^^.^.^^^^^..^^^..^..^..^^..^.^.^.^^.^^....^.^...^.^^.^^.^^..^^..^.^..^^^.^^...^...^^....^^.^^^^^^^";
    let mut s: Vec<bool> = is
        .chars()
        .map(|c| match c {
            '.' => false,
            '^' => true,
            _ => panic!("unknown char in is"),
        })
        .collect();
    let mut stc: u32 = 0;
    for _ in 0..400000 {
        stc += s.iter().fold(0, |acc, x| match x {
            true => acc,
            false => acc + 1,
        });
        s = create_next_row(s);
    }
    stc
}
