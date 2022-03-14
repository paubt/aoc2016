use std::collections::{HashSet, VecDeque};
use std::io::{stdin, Read};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let instructions = "lel".to_string();
    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

#[derive(EnumIter, Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Floor {
    One,
    Two,
    There,
    Four,
}

#[derive(Debug, Clone)]
enum ElementType {
    Generator,
    Microchip,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Element {
    id: u8,
    generator: Floor,
    microchip: Floor,
}

impl Element {
    pub fn new(id: u8, generator: Floor, microchip: Floor) -> Self {
        Element {
            id,
            generator,
            microchip,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    step_counter: u32,
    elevator: Floor,
    elements: Vec<Element>,
}

impl State {
    pub fn new(step_counter: u32, elevator: Floor, elements: Vec<Element>) -> Self {
        State {
            step_counter,
            elevator,
            elements,
        }
    }
}

fn print_state(s: &mut State) {
    s.elements.sort_by(|a, b| a.id.cmp(&b.id));
    print!("steps: {}\n     ", s.step_counter);
    s.elements.iter().for_each(|ele| print!("G M "));
    println!();
    for f in Floor::iter().rev() {
        match f {
            Floor::Four => print!("F{} ", 4),
            Floor::There => print!("F{} ", 3),
            Floor::Two => print!("F{} ", 2),
            Floor::One => print!("F{} ", 1),
        }
        if f == s.elevator {
            print!("e ")
        } else {
            print!(". ")
        }
        for ele in s.elements.iter() {
            if f == ele.generator {
                print!("{} ", ele.id)
            } else {
                print!(". ")
            }

            if f == ele.microchip {
                print!("{} ", ele.id)
            } else {
                print!(". ")
            }
        }
        println!();
    }
}

fn gen_combinations<T>(set: Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut output: Vec<Vec<T>> = Vec::new();
    for (idex, t_1) in set.iter().enumerate() {
        // add the element solo
        output.push(vec![t_1.clone()]);
        // and add the combi's with the preceding t's
        for t_2 in set.iter().skip(idex + 1) {
            output.push(vec![t_1.clone(), t_2.clone()]);
        }
    }
    output
}

fn print_floor(floor: &Vec<(u8, ElementType)>) {
    if floor.len() > 0 {
        floor
            .iter()
            .take(floor.len() - 1)
            .for_each(|(id, t)| match t {
                ElementType::Generator => print!("{}G, ", id),
                ElementType::Microchip => print!("{}M, ", id),
            });

        match floor.iter().rev().nth(0).unwrap().1 {
            ElementType::Generator => println!("{}G", floor.iter().rev().nth(0).unwrap().0),
            ElementType::Microchip => println!("{}M", floor.iter().rev().nth(0).unwrap().0),
        }
    }
}

fn print_options(options: &Vec<Vec<(u8, ElementType)>>) {
    options.iter().for_each(|v| {
        print!("(");
        v.iter().take(v.len() - 1).for_each(|(id, t)| match t {
            ElementType::Generator => print!("{}G|", id),
            ElementType::Microchip => print!("{}M|", id),
        });
        match v.iter().rev().nth(0).unwrap().1 {
            ElementType::Generator => print!("{}G), ", v.iter().rev().nth(0).unwrap().0),
            ElementType::Microchip => print!("{}M), ", v.iter().rev().nth(0).unwrap().0),
        }
    })
}

fn floor_up(f: &Floor) -> Option<Floor> {
    match f {
        Floor::One => Some(Floor::Two),
        Floor::Two => Some(Floor::There),
        Floor::There => Some(Floor::Four),
        Floor::Four => None,
    }
}

fn floor_down(f: &Floor) -> Option<Floor> {
    match f {
        Floor::Four => Some(Floor::There),
        Floor::There => Some(Floor::Two),
        Floor::Two => Some(Floor::One),
        Floor::One => None,
    }
}

fn check_if_state_viable(s: &State) -> bool {
    // to keep track of the floors
    let mut f_1_g: bool = false;
    let mut f_2_g: bool = false;
    let mut f_3_g: bool = false;
    let mut f_4_g: bool = false;
    let mut f_1_um: bool = false;
    let mut f_2_um: bool = false;
    let mut f_3_um: bool = false;
    let mut f_4_um: bool = false;
    //check each element if it has confict with previouse ones
    for ele in &s.elements {
        match ele.generator {
            Floor::One => {
                if f_1_um == true {
                    return false;
                } else {
                    f_1_g = true;
                }
            }
            Floor::Two => {
                if f_2_um == true {
                    return false;
                } else {
                    f_2_g = true;
                }
            }
            Floor::There => {
                if f_3_um == true {
                    return false;
                } else {
                    f_3_g = true;
                }
            }
            Floor::Four => {
                if f_4_um == true {
                    return false;
                } else {
                    f_4_g = true;
                }
            }
        }
        // case when floor # of microchip and generator is equal => dont add microchip as
        // unprotected cause the generator shields it
        // we use the oposit statement to catch the cases where the microchip is unprotected
        if ele.generator != ele.microchip {
            match ele.microchip {
                Floor::One => {
                    if f_1_g == true {
                        return false;
                    } else {
                        f_1_um = true;
                    }
                }
                Floor::Two => {
                    if f_2_g == true {
                        return false;
                    } else {
                        f_2_um = true;
                    }
                }
                Floor::There => {
                    if f_3_g == true {
                        return false;
                    } else {
                        f_3_um = true;
                    }
                }
                Floor::Four => {
                    if f_4_g == true {
                        return false;
                    } else {
                        f_4_um = true;
                    }
                }
            }
        }
    }
    true
}

fn expand_state(s: &State) -> Vec<State> {
    // create the subset's of things to move with 1 or 2 count
    // from the floor where the elevator currently is
    // first list of the elemens with mc or gn identifier
    let floor: Vec<(u8, ElementType)> = s.elements.iter().fold(Vec::new(), |mut acc, ele| {
        if ele.generator == s.elevator {
            acc.push((ele.id, ElementType::Generator));
        }
        if ele.microchip == s.elevator {
            acc.push((ele.id, ElementType::Microchip));
        }
        acc
    });
    let options = gen_combinations(floor);
    let mut viable_states: Vec<State> = Vec::new();
    // try the options with elevator up
    if s.elevator != Floor::Four {
        // for each option create a new state and alter the in the option specified elements
        for o in &options {
            let mut new_state = s.clone();
            new_state.step_counter += 1;
            new_state.elevator = floor_up(&new_state.elevator).unwrap();
            for (id, t) in o {
                let one_floor_up = new_state.elevator.clone();
                new_state
                    .elements
                    .iter_mut()
                    .filter(|ele| ele.id == *id)
                    .for_each(|mut ele| match t {
                        ElementType::Generator => ele.generator = one_floor_up,
                        ElementType::Microchip => ele.microchip = one_floor_up,
                    });
            }
            if check_if_state_viable(&new_state) {
                viable_states.push(new_state);
            }
        }
    }
    // try the options with elevator down
    if s.elevator != Floor::One {
        // for each option create a new state and alter the in the option specified elements
        for o in &options {
            let mut new_state = s.clone();
            new_state.step_counter += 1;
            new_state.elevator = floor_down(&new_state.elevator).unwrap();
            for (id, t) in o {
                let one_floor_up = new_state.elevator.clone();
                new_state
                    .elements
                    .iter_mut()
                    .filter(|ele| ele.id == *id)
                    .for_each(|mut ele| match t {
                        ElementType::Generator => ele.generator = one_floor_up,
                        ElementType::Microchip => ele.microchip = one_floor_up,
                    });
            }
            if check_if_state_viable(&new_state) {
                viable_states.push(new_state);
            }
        }
    }

    viable_states
}

fn check_win_condition(s: &State) -> bool {
    s.elements.iter().clone().fold(true, |acc, ele| {
        if ele.generator == Floor::Four && ele.microchip == Floor::Four {
            acc
        } else {
            false
        }
    })
}

fn calc_min_steps<const ELECOUNTER: usize>(initial_cond: State) -> u32 {
    // hashset, containing elevator position and element position as key to check we already
    // visited a specific constelation of
    let mut visied_states: HashSet<(Floor, [Element; ELECOUNTER])> = HashSet::new();
    let mut c: u32 = 0;
    let mut open_states: VecDeque<State> = VecDeque::from(vec![initial_cond]);
    while open_states.len() > 0 {
        let mut os = open_states.pop_front().unwrap();
        let mut new_states = expand_state(&os);
        for ns in new_states.iter_mut() {
            if check_win_condition(&ns) {
                return ns.step_counter;
            } else {
                /*
                if ns.step_counter > c {
                    c += 1;
                    println!("{}", c);
                }
                */
                // check if it is allready visited
                let mut a: [Element; ELECOUNTER] =
                    [Element::new(69, Floor::Four, Floor::Four); ELECOUNTER];
                //let mut a: [Element; 2] = [Element::new(69, Four, Four); 2];
                for i in 0..ELECOUNTER {
                    a[i] = ns.elements[i].clone();
                }
                if visied_states.insert((ns.elevator, a)) {
                    open_states.push_back(ns.to_owned());
                }
                //print_state(ns);
                //stdin().read(&mut [0]).unwrap();
            }
        }
    }
    // rec function returns option with if it managed to end in the objectiv constellation
    // recusion base case is found or max step_size is reached

    panic!("no solution found")
}

fn part_one(s: &str) -> u32 {
    use Floor::*;
    // create the first object
    let mut initial_cond = State::new(
        0,
        One,
        vec![
            Element::new(0, One, One),
            Element::new(1, One, Two),
            Element::new(2, One, Two),
            Element::new(3, There, There),
            Element::new(4, There, There),
        ],
        /*
        vec![Element::new(0, Two, One), Element::new(1, There, One)],
        */
    );

    calc_min_steps::<5>(initial_cond)
}

fn part_two(s: &str) -> u32 {
    use Floor::*;
    // create the first object
    let mut initial_cond = State::new(
        0,
        One,
        vec![
            Element::new(0, One, One),
            Element::new(1, One, Two),
            Element::new(2, One, Two),
            Element::new(3, There, There),
            Element::new(4, There, There),
            Element::new(5, One, One),
            Element::new(6, One, One),
        ],
        /*
        vec![Element::new(0, Two, One), Element::new(1, There, One)],
        */
    );
    calc_min_steps::<7>(initial_cond)
}
