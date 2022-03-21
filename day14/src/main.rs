fn main() {
    let answer_part_one = part_one_vec();
    let answer_part_two = part_two();

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn part_one_vec() -> usize {
    let salt = "yjdafjpo";
    let mut key_part_counter: usize = 0;
    let mut master_index: usize = 0;
    let mut poss_previous_vec: Vec<(usize, char)> = Vec::new();
    loop {
        // calc hash input
        let hi = format!("{}{}", salt, master_index);
        // hash it 2016 times
        let h: String = format!("{:x}", md5::compute(hi));
        // delete the possible previouse recorded options in the vector which have
        // a index further away than 1000 steps
        poss_previous_vec.retain(|(pi, _)| master_index - pi < 1000);
        let mut pair3_found: bool = false;
        let mut char_of_first_pair3: char = 'L';
        // check for all 5 in a rows
        let mut last_char: char = 'g';
        let mut last_char_same_counter: u32 = 0;
        'cloop: for c in h.chars() {
            if c == last_char {
                // found first 3 char same snippet
                if last_char_same_counter == 2 && pair3_found == false {
                    pair3_found = true;
                    char_of_first_pair3 = c;
                    last_char_same_counter += 1;
                }
                // found a 5 char same snippet
                else if last_char_same_counter == 4 {
                    let mut tdv: Vec<usize> = Vec::new();
                    // this could be logic error cause we need to check reverse iter
                    poss_previous_vec.sort_by(|(ai, _), (bi, _)| ai.partial_cmp(bi).unwrap());
                    for (e, (pi, pc)) in poss_previous_vec.iter().enumerate() {
                        // found one
                        // if 64th end
                        // else note his index in the vec (e) to delete next
                        if *pc == last_char {
                            //print!("{} {}\n", pi, c);
                            //print!("{}", h);
                            key_part_counter += 1;
                            if key_part_counter == 64 {
                                return *pi;
                            } else {
                                tdv.push(e);
                            }
                        }
                    }
                    for i in tdv.iter().rev() {
                        poss_previous_vec.swap_remove(*i);
                    }
                    // do this to prevent another same char to trigger a found
                    // hopefully incresing efficency
                    last_char = 'g';
                    last_char_same_counter = 0;
                } else {
                    last_char_same_counter += 1;
                    continue 'cloop;
                }
            }
            // new char start new possible streak
            else {
                last_char = c;
                last_char_same_counter = 1;
            }
        }
        if pair3_found {
            poss_previous_vec.push((master_index, char_of_first_pair3));
        }
        master_index += 1;
    }
}

fn part_two() -> usize {
    let salt = "yjdafjpo";
    let mut key_part_counter: usize = 0;
    let mut master_index: usize = 0;
    let mut poss_previous_vec: Vec<(usize, char)> = Vec::new();
    loop {
        // calc hash input
        let hi = format!("{}{}", salt, master_index);
        // hash it 2016 times

        let mut h: String = format!("{:x}", md5::compute(hi));

        for _ in 0..2016 {
            h = format!("{:x}", md5::compute(h))
        }
        // delete the possible previouse recorded options in the vector which have
        // a index further away than 1000 steps
        poss_previous_vec.retain(|(pi, _)| master_index - pi < 1000);
        let mut pair3_found: bool = false;
        let mut char_of_first_pair3: char = 'L';
        // check for all 5 in a rows
        let mut last_char: char = 'g';
        let mut last_char_same_counter: u32 = 0;
        'cloop: for c in h.chars() {
            if c == last_char {
                // found first 3 char same snippet
                if last_char_same_counter == 2 && pair3_found == false {
                    pair3_found = true;
                    char_of_first_pair3 = c;
                    last_char_same_counter += 1;
                }
                // found a 5 char same snippet
                else if last_char_same_counter == 4 {
                    let mut tdv: Vec<usize> = Vec::new();
                    // this could be logic error cause we need to check reverse iter
                    poss_previous_vec.sort_by(|(ai, _), (bi, _)| ai.partial_cmp(bi).unwrap());
                    for (e, (pi, pc)) in poss_previous_vec.iter().enumerate() {
                        // found one
                        // if 64th end
                        // else note his index in the vec (e) to delete next
                        if *pc == last_char {
                            key_part_counter += 1;
                            if key_part_counter == 65 {
                                return *pi;
                            } else {
                                tdv.push(e);
                            }
                        }
                    }
                    for i in tdv.iter().rev() {
                        poss_previous_vec.swap_remove(*i);
                    }
                    // do this to prevent another same char to trigger a found
                    // hopefully incresing efficency
                    last_char = 'g';
                    last_char_same_counter = 0;
                } else {
                    last_char_same_counter += 1;
                    continue 'cloop;
                }
            }
            // new char start new possible streak
            else {
                last_char = c;
                last_char_same_counter = 1;
            }
        }
        if pair3_found {
            poss_previous_vec.push((master_index, char_of_first_pair3));
        }
        master_index += 1;
    }
}
