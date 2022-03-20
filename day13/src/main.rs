use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

fn main() {
    let answer_part_one = match part_one() {
        Some(t) => t.to_string(),
        None => "no solution found".to_string(),
    };
    let answer_part_two = part_two();

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

enum LocationType {
    Wall,
    OpenSpace,
}

fn calc_location_type(x: i32, y: i32, pi: i32) -> Option<LocationType> {
    if x < 0 || y < 0 {
        None
    } else {
        let t: i32 = x * x + 3 * x + 2 * x * y + y + y * y;
        let t: i32 = t + pi;
        let t: u32 = t.count_ones();
        if t % 2 == 0 {
            Some(LocationType::OpenSpace)
        } else {
            Some(LocationType::Wall)
        }
    }
}

// aka manhattan distance
fn calc_prio(from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> Option<u32> {
    if from_x < 0 || from_y < 0 || to_x < 0 || to_y < 0 {
        None
    } else {
        Some(((from_x - to_x).abs() + (from_y - to_y).abs()) as u32)
    }
}

fn part_one() -> Option<u32> {
    // init values
    let puzzle_input: i32 = 1352;
    let start_x: i32 = 1;
    let start_y: i32 = 1;
    let target_x: i32 = 31;
    let target_y: i32 = 39;
    // datastructures for the "A*" search
    let mut visited_locations: HashMap<(i32, i32), (LocationType, u32)> = HashMap::new();
    let mut open_locations: PriorityQueue<(i32, i32, u32), Reverse<u32>> = PriorityQueue::new();
    // add the starting location to the queue and visited positions
    let prio: u32 = calc_prio(start_x, start_y, target_x, target_y).unwrap();
    open_locations.push((start_x, start_y, 0), Reverse(prio));
    visited_locations.insert((start_x, start_y), (LocationType::OpenSpace, 0));

    while let Some(((x, y, step_count), _)) = open_locations.pop() {
        // check the 4 adjacent locations and add them to the queue
        'pnl: for (px, py) in [(x, y - 1), (x, y + 1), (x + 1, y), (x - 1, y)] {
            // invalid location
            if px < 0 || py < 0 {
                continue 'pnl;
            }
            // the location was already visited
            if visited_locations.contains_key(&(px, py)) {
                continue 'pnl;
            }
            // new location
            else {
                // is the target location
                if px == target_x && py == target_y {
                    return Some(step_count + 1);
                }
                // else add it if its a open space
                let nlt: LocationType = match calc_location_type(px, py, puzzle_input) {
                    Some(t) => t,
                    None => panic!("probably invalid location coordinates1"),
                };

                match nlt {
                    LocationType::Wall => {
                        visited_locations.insert((px, py), (LocationType::Wall, step_count + 1));
                    }
                    LocationType::OpenSpace => {
                        let np: u32 = calc_prio(px, py, target_x, target_y)
                            .expect("probably invalid location coordinates2");
                        open_locations.push((px, py, step_count + 1), Reverse(np));
                        visited_locations
                            .insert((px, py), (LocationType::OpenSpace, step_count + 1));
                    }
                }
            }
        }
    }
    None
}

fn part_two() -> u32 {
    // init values
    let puzzle_input: i32 = 1352;
    let start_x: i32 = 1;
    let start_y: i32 = 1;
    let target_x: i32 = 31;
    let target_y: i32 = 39;
    // datastructures for the "A*" search
    let mut visited_locations: HashMap<(i32, i32), (LocationType, u32)> = HashMap::new();
    let mut open_locations: PriorityQueue<(i32, i32, u32), Reverse<u32>> = PriorityQueue::new();
    // add the starting location to the queue and visited positions
    let prio: u32 = calc_prio(start_x, start_y, target_x, target_y).unwrap();
    open_locations.push((start_x, start_y, 0), Reverse(prio));
    visited_locations.insert((start_x, start_y), (LocationType::OpenSpace, 0));

    while let Some(((x, y, step_count), _)) = open_locations.pop() {
        //
        if step_count == 50 {
            continue;
        }
        // check the 4 adjacent locations and add them to the queue
        'pnl: for (px, py) in [(x, y - 1), (x, y + 1), (x + 1, y), (x - 1, y)] {
            // invalid location
            if px < 0 || py < 0 {
                continue 'pnl;
            }
            // the location was already visited
            if visited_locations.contains_key(&(px, py)) {
                continue 'pnl;
            }
            // new location
            else {
                // add it if its a open space to queue
                // ether way add it to visited locations
                let nlt: LocationType = match calc_location_type(px, py, puzzle_input) {
                    Some(t) => t,
                    None => panic!("probably invalid location coordinates1"),
                };

                match nlt {
                    LocationType::Wall => {
                        visited_locations.insert((px, py), (LocationType::Wall, step_count + 1));
                    }
                    LocationType::OpenSpace => {
                        let np: u32 = calc_prio(px, py, target_x, target_y)
                            .expect("probably invalid location coordinates2");
                        open_locations.push((px, py, step_count + 1), Reverse(np));
                        visited_locations
                            .insert((px, py), (LocationType::OpenSpace, step_count + 1));
                    }
                }
            }
        }
    }
    visited_locations
        .iter()
        .fold(0, |acc, ((_, _), (lt, _))| match lt {
            LocationType::Wall => acc,
            LocationType::OpenSpace => acc + 1,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn determin_location_1() {
        assert!(matches!(calc_location_type(-1, 2, 10), None));
    }
    #[test]
    fn determin_location_2() {
        assert!(matches!(calc_location_type(12, -1, 10), None));
    }
    #[test]
    fn determin_location_3() {
        assert!(matches!(calc_location_type(-1, -1, 10), None));
    }
    #[test]
    fn determin_location_4() {
        assert!(matches!(
            calc_location_type(0, 0, 10),
            Some(LocationType::OpenSpace)
        ));
    }
    #[test]
    fn determin_location_5() {
        assert!(matches!(
            calc_location_type(9, 1, 10),
            Some(LocationType::Wall)
        ));
    }
    #[test]
    fn prio_determination_1() {
        assert!(matches!(calc_prio(0, 0, 7, 4), Some(11)));
    }
    #[test]
    fn prio_determination_2() {
        assert!(matches!(calc_prio(-1, 0, 7, 4), None));
    }
    #[test]
    fn prio_determination_3() {
        assert!(matches!(calc_prio(32, 12, 89, 10), Some(59)));
    }
    #[test]
    fn prio_determination_4() {
        assert!(matches!(calc_prio(7, 4, 7, 4), Some(0)));
    }
    #[test]
    fn prio_determination_5() {
        assert!(matches!(calc_prio(7, 5, 7, 4), Some(1)));
    }
}
