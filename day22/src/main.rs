use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let instructions = fs::read_to_string("../data/day22/input.txt").expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!(
        "\nanswer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

#[derive(Hash)]
struct Node {
    x: u32,
    y: u32,
    size: u32,
    used: u32,
    avail: u32,
    used_pc: u32,
}

enum NodeType {
    Big,
    Small,
}

enum NodeState {
    Empty,
    Full,
}
struct AbstNode {
    typ: NodeType,
    state: NodeState,
}

impl Node {
    pub fn new(x: u32, y: u32, size: u32, used: u32, avail: u32, used_pc: u32) -> Node {
        Node {
            x,
            y,
            size,
            used,
            avail,
            used_pc,
        }
    }
}

impl AbstNode {
    pub fn new(typ: NodeType, state: NodeState) -> AbstNode {
        AbstNode { typ, state }
    }
}

fn create_node_vec(s: &str) -> Vec<Node> {
    s.lines()
        .map(|l| {
            let sl: Vec<&str> = l.split_whitespace().clone().collect();
            let (x, y) = sl[0]
                .split('-')
                .skip(1)
                .map(|k| {
                    k.chars()
                        .skip(1)
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap();
            let mut sl_i: Vec<u32> = Vec::new();
            for i in 1..5 {
                let mut t = sl[i].chars();
                t.next_back();
                sl_i.push(t.collect::<String>().parse::<u32>().unwrap());
            }
            Node::new(x, y, sl_i[0], sl_i[1], sl_i[2], sl_i[3])
        })
        .collect()
}

fn part_one(s: &str) -> usize {
    let nodes: Vec<Node> = create_node_vec(s);

    let t = nodes
        .iter()
        .cartesian_product(nodes.iter())
        .filter(|(a, b)| a.used != 0 && (a.x != b.x || a.y != b.y) && a.used <= b.avail)
        .count();
    /*
        nodes.iter().for_each(|n| {
            println!(
                "x={} y={} s={} u={} a={} u={}%",
                n.x, n.y, n.size, n.used, n.avail, n.used_pc
            )
        });
    */
    t
}

fn create_abst_hashmap(i: Vec<Node>) -> HashMap<(u32, u32), AbstNode> {
    i.iter().fold(HashMap::new(), |mut acc, n| {
        if n.used == 0 {
            match acc.insert((n.x, n.y), AbstNode::new(NodeType::Small, NodeState::Empty)) {
                Some(_) => panic!("zero two times inserted"),
                None => (),
            }
        } else if n.size > 250 {
            assert!(n.used_pc > 50);
            match acc.insert((n.x, n.y), AbstNode::new(NodeType::Big, NodeState::Full)) {
                Some(_) => panic!("big insert two times"),
                None => (),
            }
        } else {
            assert!(n.used_pc > 50);
            match acc.insert((n.x, n.y), AbstNode::new(NodeType::Small, NodeState::Full)) {
                Some(_) => panic!("big insert two times"),
                None => (),
            }
        }
        acc
    })
}

fn part_two(s: &str) -> String {
    let nodes: Vec<Node> = create_node_vec(s);
    let nodes = create_abst_hashmap(nodes);

    // read the shortest path from the consol output

    for y in 0..25 {
        for x in 0..35 {
            if x == 0 && y == 0 {
                print!("T");
            } else if x == 34 && y == 0 {
                print!("G");
            } else {
                match nodes.get(&(x, y)).unwrap().state {
                    NodeState::Full => match nodes.get(&(x, y)).unwrap().typ {
                        NodeType::Big => print!("#"),
                        NodeType::Small => print!("."),
                    },
                    NodeState::Empty => print!("_"),
                }
            }
        }
        println!();
    }

    "read from grid above; 200 steps".to_string()
}
