use itertools::Itertools;
use std::fs;

fn main() {
    let instructions = fs::read_to_string("../data/day8/input.txt").expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn print_display(display: &Vec<Vec<bool>>) {
    print!("\n");
    display.iter().for_each(|row| {
        row.iter().for_each(|ele| match ele {
            true => print!("#"),
            false => print!(" "),
        });
        print!("\n");
    })
}

fn part_one(instructions: &str) -> u32 {
    let row_count = 6;
    let col_count = 50;
    let mut display: Vec<Vec<bool>> = vec![vec![false; col_count]; row_count];

    for ins in instructions.lines() {
        // identify the the three distinct instructions
        let ins: Vec<&str> = ins.split(" ").collect();
        // rectangle add
        if ins.len() == 2 {
            let (c, r) = ins[1]
                .split("x")
                .map(|s| s.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap();
            for row in display.iter_mut().take(r) {
                for ele in row.iter_mut().take(c) {
                    *ele = true;
                }
            }
        } else {
            let (axis, nr) = ins[2].split("=").next_tuple().unwrap();
            let nr = nr.parse::<usize>().unwrap();
            let shift_count = ins[4].parse::<usize>().unwrap();
            match axis {
                // row
                "y" => {
                    display[nr] = display[nr]
                        .iter()
                        .copied()
                        .skip(col_count - shift_count)
                        .chain(display[nr].iter().copied().take(col_count - shift_count))
                        .collect::<Vec<bool>>();
                }
                // column
                "x" => {
                    // copy the column to shift
                    let old_col = display
                        .iter()
                        .map(|row| row.iter().copied().nth(nr).unwrap())
                        .collect::<Vec<bool>>();
                    // take shift same as in the row case
                    let new_col = old_col
                        .iter()
                        .copied()
                        .skip(row_count - shift_count)
                        .chain(old_col.iter().copied().take(row_count - shift_count))
                        .collect::<Vec<bool>>();
                    // insert the new values into the display
                    for r in 0..row_count {
                        display[r][nr] = new_col[r];
                    }
                }
                _ => panic!("error in char found in shift instructions"),
            }
        }
    }
    print_display(&display);
    display.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |acc, ele| acc + *ele as u32)
    })
}

fn part_two(_: &str) -> u32 {
    0
}
