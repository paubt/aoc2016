use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let instructions = fs::read_to_string("../data/day10/input.txt").expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

#[derive(Debug, Clone, Copy)]
enum BOLink {
    Output(u32),
    Bot(u32),
}

#[derive(Debug, Clone)]
struct Bot {
    micro_chip_list: Vec<u32>,
    low: Option<BOLink>,
    high: Option<BOLink>,
}

impl Bot {
    pub fn new(micro_chip_list: Vec<u32>, low: Option<BOLink>, high: Option<BOLink>) -> Self {
        Self {
            micro_chip_list,
            low,
            high,
        }
    }
}

fn create_hm(instructions: &str) -> HashMap<u32, Bot> {
    // map for the bots with key == id and value == (low, high and current chips holded)
    instructions.lines().fold(HashMap::new(), |mut acc, line| {
        // value added to list of bot
        if line.chars().nth(0).unwrap() == 'v' {
            let (bot_id, mc_value) = line
                .split(' ')
                .rev()
                .step_by(4)
                .map(|x| x.parse::<u32>().unwrap())
                .next_tuple()
                .unwrap();
            // bot already in bot_map
            if let Some(b) = acc.get_mut(&bot_id) {
                b.micro_chip_list.push(mc_value);
            } else {
                acc.insert(bot_id, Bot::new(vec![mc_value], None, None));
            }
        }
        // bot pass on instructions
        else {
            let (h, l, bot_id) = line
                .split(' ')
                .rev()
                .step_by(5)
                .map(|x| x.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            let (low_target_type, high_target_type) =
                line.split(' ').skip(5).step_by(5).collect_tuple().unwrap();
            if let Some(b) = acc.get_mut(&bot_id) {
                b.low = match low_target_type {
                    "output" => Some(BOLink::Output(l)),
                    "bot" => Some(BOLink::Bot(l)),
                    _ => panic!("not foumnd"),
                };
                b.high = match high_target_type {
                    "output" => Some(BOLink::Output(h)),
                    "bot" => Some(BOLink::Bot(h)),
                    _ => panic!("not foumnd"),
                };
            } else {
                acc.insert(
                    bot_id,
                    Bot::new(
                        Vec::new(),
                        match low_target_type {
                            "output" => Some(BOLink::Output(l)),
                            "bot" => Some(BOLink::Bot(l)),
                            _ => panic!("ins p1"),
                        },
                        match high_target_type {
                            "output" => Some(BOLink::Output(h)),
                            "bot" => Some(BOLink::Bot(h)),
                            _ => panic!("ins p1"),
                        },
                    ),
                );
            }
        }
        acc
    })
}

fn part_one(instructions: &str) -> u32 {
    // map for the bots with key == id and value == (low, high and current chips holded)
    let mut bot_map: HashMap<u32, Bot> = create_hm(instructions);
    //dbg!(bot_map.clone());
    let mut output_map: HashMap<u32, u32> = HashMap::new();

    let mut update: bool = true;
    while update {
        // create update vector
        // ((low_target_: BOLink, low_value: u32),
        // (high_target: BOLink, high_value: u32))
        let target_update_vec: Vec<((BOLink, u32), (BOLink, u32))> = bot_map
            .iter_mut()
            .filter(|(_, b)| b.micro_chip_list.len() > 1)
            .fold(Vec::new(), |mut acc, (b_id, b)| {
                // get the target values and id's
                let mut low_value = b.micro_chip_list.remove(0);
                let mut high_value = b.micro_chip_list.remove(0);
                // here we need a switch when we low is really high
                if low_value > high_value {
                    let t = low_value;
                    low_value = high_value;
                    high_value = t;
                }
                // we check here asked in part 1 instructions
                // optimization: compare added value x+y -> compare one = x or y
                if low_value + high_value == 78 {
                    if low_value == 17 || high_value == 17 {
                        print!("bot of interst is: {}", b_id);
                    }
                }

                let low_target = b.low.unwrap();
                let high_target = b.high.unwrap();
                acc.push(((low_target, low_value), (high_target, high_value)));
                acc
            });
        // set the loop condition to "Continue going"
        if target_update_vec.len() > 0 {
            update = true
        } else {
            update = false
        };
        //dbg!(target_update_vec.clone());
        // use the target vec to update the
        target_update_vec
            .iter()
            .for_each(|((l_t, l_v), (h_t, h_v))| {
                // do the low part
                match l_t {
                    BOLink::Bot(b_id) => bot_map.get_mut(b_id).unwrap().micro_chip_list.push(*l_v),
                    BOLink::Output(o_id) => {
                        let e = output_map.entry(*o_id).or_insert(0);
                        *e = *l_v
                    }
                }
                // annnnd high part
                match h_t {
                    BOLink::Bot(b_id) => bot_map.get_mut(b_id).unwrap().micro_chip_list.push(*h_v),
                    BOLink::Output(o_id) => {
                        let e = output_map.entry(*o_id).or_insert(0);
                        *e = *h_v
                    }
                }
            });
        //dbg!(bot_map.clone());
        //dbg!(Someoutput_map.clone());
        //print!("\n\n\n");
    }
    //dbg!(bot_map);
    //dbg!(output_map);
    0
}

fn part_two(instructions: &str) -> u32 {
    // map for the bots with key == id and value == (low, high and current chips holded)
    let mut bot_map: HashMap<u32, Bot> = create_hm(instructions);
    //dbg!(bot_map.clone());
    let mut output_map: HashMap<u32, u32> = HashMap::new();

    let mut update: bool = true;
    while update {
        // create update vector
        // ((low_target_: BOLink, low_value: u32),
        // (high_target: BOLink, high_value: u32))
        let target_update_vec: Vec<((BOLink, u32), (BOLink, u32))> = bot_map
            .iter_mut()
            .filter(|(_, b)| b.micro_chip_list.len() > 1)
            .fold(Vec::new(), |mut acc, (_, b)| {
                // get the target values and id's
                let mut low_value = b.micro_chip_list.remove(0);
                let mut high_value = b.micro_chip_list.remove(0);
                // here we need a switch when we low is really high
                if low_value > high_value {
                    let t = low_value;
                    low_value = high_value;
                    high_value = t;
                }
                let low_target = b.low.unwrap();
                let high_target = b.high.unwrap();
                acc.push(((low_target, low_value), (high_target, high_value)));
                acc
            });
        // set the loop condition to "Continue going"
        if target_update_vec.len() > 0 {
            update = true
        } else {
            update = false
        };
        //dbg!(target_update_vec.clone());
        // use the target vec to update the
        target_update_vec
            .iter()
            .for_each(|((l_t, l_v), (h_t, h_v))| {
                // do the low part
                match l_t {
                    BOLink::Bot(b_id) => bot_map.get_mut(b_id).unwrap().micro_chip_list.push(*l_v),
                    BOLink::Output(o_id) => {
                        let e = output_map.entry(*o_id).or_insert(0);
                        *e = *l_v
                    }
                }
                // annnnd high part
                match h_t {
                    BOLink::Bot(b_id) => bot_map.get_mut(b_id).unwrap().micro_chip_list.push(*h_v),
                    BOLink::Output(o_id) => {
                        let e = output_map.entry(*o_id).or_insert(0);
                        *e = *h_v
                    }
                }
            });
        //dbg!(bot_map.clone());
        //dbg!(Someoutput_map.clone());
        //print!("\n\n\n");
    }
    //dbg!(output_map);
    output_map
        .iter()
        .filter(|(o_id, _)| **o_id < 3)
        .fold(1, |acc, (_, o_vl)| acc * o_vl)
}
