fn main() {
    let answer_part_one = part_one();
    let answer_part_two = part_two();

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn part_one() -> u32 {
    let elf_number_input = 3014603;
    let mut ig_elfs: Vec<u32> = Vec::new();
    for e in 1..elf_number_input + 1 {
        ig_elfs.push(e);
    }

    while ig_elfs.len() > 1 {
        if ig_elfs.len() % 2 != 0 {
            let last_elf = ig_elfs.pop().unwrap();
            ig_elfs = ig_elfs.iter().copied().step_by(2).collect::<Vec<u32>>();
            ig_elfs.insert(0, last_elf);
        } else {
            ig_elfs = ig_elfs.iter().copied().step_by(2).collect::<Vec<u32>>();
        }
    }

    *ig_elfs.first().unwrap()
}

fn part_two() -> u32 {
    let elf_number_input = 3014603;
    let mut ig_elfs: Vec<u32> = Vec::new();
    for e in 1..elf_number_input + 1 {
        ig_elfs.push(e);
    }

    while ig_elfs.len() > 1 {
        ig_elfs.remove(ig_elfs.len() / 2);
        let t = ig_elfs.remove(0);
        ig_elfs.push(t);
    }

    *ig_elfs.first().unwrap()
}
