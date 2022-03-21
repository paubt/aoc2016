fn main() {
    // no: 185303
    let answer_part_one = part_one();
    let answer_part_two = part_two();

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn part_one() -> u32 {
    let tp1 = 13;
    let tp2 = 5;
    let tp3 = 17;
    let tp4 = 3;
    let tp5 = 7;
    let tp6 = 19;

    let sp1 = 11;
    let sp2 = 0;
    let sp3 = 11;
    let sp4 = 0;
    let sp5 = 2;
    let sp6 = 17;

    let mut t = 0;
    loop {
        let c = ((t + 1 + sp1) % tp1)
            + ((t + 2 + sp2) % tp2)
            + ((t + 3 + sp3) % tp3)
            + ((t + 4 + sp4) % tp4)
            + ((t + 5 + sp5) % tp5)
            + ((t + 6 + sp6) % tp6);
        if c == 0 {
            break t;
        }
        t += 1;
    }
}

fn part_two() -> u32 {
    let tp1 = 13;
    let tp2 = 5;
    let tp3 = 17;
    let tp4 = 3;
    let tp5 = 7;
    let tp6 = 19;
    let tp7 = 11;

    let sp1 = 11;
    let sp2 = 0;
    let sp3 = 11;
    let sp4 = 0;
    let sp5 = 2;
    let sp6 = 17;
    let sp7 = 0;

    let mut t = 0;
    loop {
        let c = ((t + 1 + sp1) % tp1)
            + ((t + 2 + sp2) % tp2)
            + ((t + 3 + sp3) % tp3)
            + ((t + 4 + sp4) % tp4)
            + ((t + 5 + sp5) % tp5)
            + ((t + 6 + sp6) % tp6)
            + ((t + 7 + sp7) % tp7);
        if c == 0 {
            break t;
        }
        t += 1;
    }
}
