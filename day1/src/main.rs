use std::collections::HashSet;
use std::fs;

fn main() {
    let instructions = fs::read_to_string("../data/day1/input.txt").expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "answer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn update_positon(mut x: i32, mut y: i32, dir: Direction, cmd: &str) -> (i32, i32, Direction) {
    // determine Direction
    let new_dir = match &cmd[0..1] {
        "L" => match dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        },
        "R" => match dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        },
        _ => panic!("Wrong direction char"),
    };
    let blocks: i32 = cmd[1..].parse::<i32>().expect(&cmd[1..]);
    match new_dir {
        Direction::North => y += blocks,
        Direction::East => x += blocks,
        Direction::South => y += -blocks,
        Direction::West => x += -blocks,
    }
    (x, y, new_dir)
}

fn sequence_of_position_updates(
    mut x: i32,
    mut y: i32,
    mut dir: Direction,
    instructions: &str,
) -> (i32, i32, Direction) {
    for cmd in instructions.lines().next().unwrap().split(", ") {
        (x, y, dir) = update_positon(x, y, dir, cmd);
    }
    (x, y, dir)
}

fn calc_taxi_distance(x: i32, y: i32, dir: Direction, instructions: &str) -> u32 {
    let start_x: i32 = x;
    let start_y: i32 = y;
    let (end_x, end_y, _) = sequence_of_position_updates(x, y, dir, instructions);
    ((start_x - end_x).abs() + (start_y - end_y).abs()) as u32
}

fn part_one(instructions: &str) -> u32 {
    calc_taxi_distance(0, 0, Direction::North, instructions)
}

fn update_positon_visited_points_list(
    mut x: i32,
    mut y: i32,
    dir: Direction,
    cmd: &str,
) -> (i32, i32, Direction, Vec<(i32, i32)>) {
    // determine Direction
    let new_dir = match &cmd[0..1] {
        "L" => match dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        },
        "R" => match dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        },
        _ => panic!("Wrong direction char"),
    };
    let mut visted_postions: Vec<(i32, i32)> = Vec::new();
    let blocks: i32 = cmd[1..].parse::<i32>().expect(&cmd[1..]);
    for _ in 0..blocks {
        match new_dir {
            Direction::North => y += 1,
            Direction::East => x += 1,
            Direction::South => y += -1,
            Direction::West => x += -1,
        }
        visted_postions.push((x, y));
    }
    (x, y, new_dir, visted_postions)
}

fn distance_two_visits_first(instructions: &str) -> u32 {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut new_positions: Vec<(i32, i32)>;
    let mut dir: Direction = Direction::North;
    for cmd in instructions.lines().next().unwrap().split(", ") {
        (x, y, dir, new_positions) = update_positon_visited_points_list(x, y, dir, cmd);
        for (x, y) in new_positions {
            if !visited_positions.insert((x, y)) {
                return (x.abs() + y.abs()) as u32;
            }
        }
    }

    panic!("the instructions dont contain a double visit");
}

fn part_two(instructions: &str) -> u32 {
    distance_two_visits_first(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_position_1() {
        let (x, y, dir) = update_positon(13, -4, Direction::North, "R2");
        assert_eq!(x, 15);
        assert_eq!(y, -4);
        assert!(matches!(dir, Direction::East))
    }
    #[test]
    fn test_update_position_2() {
        let (x, y, dir) = update_positon(0, 3, Direction::West, "L99");
        assert_eq!(x, 0);
        assert_eq!(y, -96);
        assert!(matches!(dir, Direction::South))
    }
    #[test]
    #[should_panic]
    fn test_update_position_3() {
        let _ = update_positon(0, 3, Direction::West, "D99");
    }
    #[test]
    fn test_sequence_of_instuctions_1() {
        let (x, y, dir) = sequence_of_position_updates(0, 0, Direction::North, "R2, L3");
        assert_eq!(x, 2);
        assert_eq!(y, 3);
        assert!(matches!(dir, Direction::North))
    }
    #[test]
    fn test_sequence_of_instuctions_2() {
        let (x, y, dir) = sequence_of_position_updates(0, 0, Direction::North, "R2, R2, R2");
        assert_eq!(x, 0);
        assert_eq!(y, -2);
        assert!(matches!(dir, Direction::West))
    }
    #[test]
    #[should_panic]
    fn test_sequence_of_instuctions_3() {
        let _ = sequence_of_position_updates(0, 0, Direction::North, "R2;L2, R2");
    }
    #[test]
    fn test_calc_taxi_distance_1() {
        let distance = calc_taxi_distance(0, 0, Direction::North, "R5, L5, R5, R3");
        assert_eq!(distance, 12);
    }
    #[test]
    fn test_calc_taxi_distance_2() {
        let distance = calc_taxi_distance(0, 0, Direction::North, "R2, R2, R2\n");
        assert_eq!(distance, 2);
    }
    #[test]
    fn test_calc_taxi_distance_3() {
        let distance = calc_taxi_distance(0, 0, Direction::North, "R2, L3");
        assert_eq!(distance, 5);
    }
}
