use aoc::io::*;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(&b),
            (Packet::List(a), Packet::List(b)) => {
                let items = a.iter().zip(b.iter());
                for (item_a, item_b) in items {
                    match item_a.cmp(item_b) {
                        Ordering::Equal => continue,
                        ord => return ord,
                    }
                }
                a.len().cmp(&b.len())
            }
            (Packet::Int(_), Packet::List(_)) => Packet::List(vec![self.clone(); 1]).cmp(other),
            (Packet::List(_), Packet::Int(_)) => self.cmp(&Packet::List(vec![other.clone(); 1])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn packet_to_string(packet: &Packet) -> String {
    match packet {
        Packet::Int(n) => return format!("{n}"),
        Packet::List(list) => {
            let mut list_str: String = "[".to_owned();
            for (i, item) in list.iter().enumerate() {
                let item_str = packet_to_string(item);
                if i > 0 {
                    list_str.push_str(",");
                }
                list_str.push_str(&item_str);
            }
            list_str.push_str("]");
            return list_str;
        }
    }
}

fn parse_packet(input: &str) -> Result<Packet, String> {
    let tokens = input.chars();
    let mut stack = Vec::<Vec<Packet>>::new();
    let mut num: Option<i32> = None;
    let mut root: Option<Vec<Packet>> = None;
    for token in tokens {
        match token {
            // push new vec
            '[' => {
                stack.push(Vec::new());
            }
            // pop
            ']' => {
                let mut list = stack.pop().ok_or("unexpected ']'")?;
                if let Some(n) = num {
                    list.push(Packet::Int(n));
                    num = None;
                }
                if let Some(parent) = stack.last_mut() {
                    parent.push(Packet::List(list));
                } else {
                    root = Some(list);
                }
            }
            // pop num
            ',' => {
                if let Some(n) = num {
                    let list: &mut Vec<Packet> = stack.last_mut().ok_or("unexpected ','")?;
                    list.push(Packet::Int(n));
                    num = None;
                }
            }
            // push num
            digit => {
                let n: i32 = digit as i32 - 48;
                if let Some(current) = num {
                    num = Some(current * 10 + n);
                } else {
                    num = Some(n);
                }
            }
        }
    }
    root.map(Packet::List)
        .ok_or("expected top level List".into())
}

fn parse_pairs(input: &Vec<String>) -> Vec<(Packet, Packet)> {
    let packets: Vec<&str> = input
        .iter()
        .map(String::as_str)
        .filter(|s| !s.is_empty())
        .collect();
    packets
        .chunks(2)
        .map(|pair| {
            let a = parse_packet(pair[0]).expect("failed to parse first item in pair");
            let b = parse_packet(pair[1]).expect("failed to parse second item in pair");
            let pair_a_str = packet_to_string(&a);
            let pair_b_str = packet_to_string(&b);
            if pair_a_str != pair[0] {
                panic!("roundtrip failed: {}\n{}", pair[0], pair_a_str);
            }
            if pair_b_str != pair[1] {
                panic!("roundtrip failed: {}\n{}", pair[1], pair_b_str);
            }
            (a, b)
        })
        .collect()
}

fn part1(input: &Vec<String>) -> usize {
    let pairs = parse_pairs(&input);
    pairs
        .iter()
        .enumerate()
        .fold(0, |sum, (i, pair)| match pair.0.cmp(&pair.1) {
            Ordering::Less | Ordering::Equal => sum + i + 1,
            Ordering::Greater => sum,
        })
}

fn part2(input: &Vec<String>) -> usize {
    let mut packets: Vec<Packet> = input
        .iter()
        .map(String::as_str)
        .filter(|s| !s.is_empty())
        .map(|line| parse_packet(line).unwrap())
        .collect();
    let divider1 = parse_packet("[[2]]").unwrap();
    let divider2 = parse_packet("[[6]]").unwrap();
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort();
    let divider1_index = packets.iter().position(|p| *p == divider1).unwrap() + 1;
    let divider2_index = packets.iter().position(|p| *p == divider2).unwrap() + 1;
    divider1_index * divider2_index
}

fn main() {
    let input: Vec<String> = read_input();
    println!("part 1 = {:?}", part1(&input));
    println!("part 2 = {:?}", part2(&input));
}
