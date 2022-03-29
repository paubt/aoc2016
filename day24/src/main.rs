use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs;

fn main() {
    let instructions = fs::read_to_string("../data/day24/input.txt").expect("read in file failed");

    let answer_part_one = 518; // part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

enum MazeType {
    Wall,
    Open,
}

fn find_shortest_path(
    maze: &HashMap<(usize, usize), MazeType>,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
) -> usize {
    let mut open_pos: PriorityQueue<(usize, usize), Reverse<usize>> = PriorityQueue::new();
    open_pos.push((from_row, from_col), Reverse(0));

    while let Some(((r, c), Reverse(s))) = open_pos.pop() {
        if r == to_row && c == to_col {
            return s;
        }
        for (nr, nc) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
            match maze.get(&(nr, nc)).unwrap() {
                MazeType::Open => {
                    open_pos.push((nr, nc), Reverse(s + 1));
                }
                MazeType::Wall => (),
            }
        }
    }
    panic!("no connection found")
}

fn gen_distance_map(s: &str) -> (Vec<usize>, HashMap<(usize, usize), usize>) {
    let mut maze: HashMap<(usize, usize), MazeType> = HashMap::new();
    let mut posi: Vec<(usize, (usize, usize))> = Vec::new();
    for (row, l) in s.lines().enumerate() {
        for (col, ele) in l.chars().enumerate() {
            match ele {
                '#' => {
                    maze.insert((row, col), MazeType::Wall);
                    ()
                }
                '.' => {
                    maze.insert((row, col), MazeType::Open);
                    ()
                }
                _ => {
                    maze.insert((row, col), MazeType::Open);
                    posi.push((ele.to_string().parse::<i32>().unwrap() as usize, (row, col)));
                }
            }
        }
    }

    let distance: HashMap<(usize, usize), usize> = posi.iter().cartesian_product(posi.iter()).fold(
        HashMap::new(),
        |mut acc, ((e1, (r1, c1)), (e2, (r2, c2)))| {
            acc.insert((*e1, *e2), find_shortest_path(&maze, *r1, *c1, *r2, *c2));
            acc
        },
    );
    let posi = posi.iter().map(|(e, _)| e).copied().collect();
    (posi, distance)
}

fn part_one(s: &str) -> usize {
    // transfrom maze to step distance between number > 0
    let (mut elements, distances) = gen_distance_map(s);
    elements.retain(|&x| x != 0);

    let mut open_paths: PriorityQueue<(Vec<usize>, Vec<usize>), Reverse<usize>> =
        PriorityQueue::new();
    open_paths.push((vec![0], elements), Reverse(0));

    while let Some(((visited, todo), Reverse(steps))) = open_paths.pop() {
        if todo.is_empty() {
            return steps;
        }
        let last_visited: usize = *visited.last().unwrap();
        for t in &todo {
            let added_distance = distances.get(&(last_visited, *t)).unwrap();
            let mut new_visited = visited.clone();
            new_visited.push(*t);
            let new_todo = todo.iter().filter(|x| *x != t).copied().collect();
            open_paths.push((new_visited, new_todo), Reverse(*added_distance + steps));
        }
    }
    0
}

fn part_two(s: &str) -> usize {
    // transfrom maze to step distance between number > 0
    let (mut elements, distances) = gen_distance_map(s);
    elements.retain(|&x| x != 0);

    let mut open_paths: PriorityQueue<(Vec<usize>, Vec<usize>), Reverse<usize>> =
        PriorityQueue::new();
    open_paths.push((vec![0], elements), Reverse(0));

    while let Some(((visited, todo), Reverse(steps))) = open_paths.pop() {
        if todo.is_empty() {
            return steps;
        }
        let last_visited: usize = *visited.last().unwrap();
        for t in &todo {
            let mut added_distance = *distances.get(&(last_visited, *t)).unwrap();
            if todo.len() == 1 {
                added_distance += distances.get(&(*t, 0)).unwrap();
            }
            let mut new_visited = visited.clone();
            new_visited.push(*t);
            let new_todo = todo.iter().filter(|x| *x != t).copied().collect();
            open_paths.push((new_visited, new_todo), Reverse(added_distance + steps));
        }
    }
    0
}
