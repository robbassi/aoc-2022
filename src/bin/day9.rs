#![feature(const_trait_impl)]

use aoc::io::*;
use std::collections::HashSet;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl const Add for Pos {
    type Output = Pos;

    fn add(self, other: Self) -> Self::Output {
        let Pos(x1, y1) = self;
        let Pos(x2, y2) = other;
        Self(x1 + x2, y1 + y2)
    }
}

impl const Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Self) -> Self::Output {
        let Pos(x1, y1) = self;
        let Pos(x2, y2) = other;
        Self(x1 - x2, y1 - y2)
    }
}

const UP: Pos = Pos(0, 1);
const DOWN: Pos = Pos(0, -1);
const LEFT: Pos = Pos(-1, 0);
const RIGHT: Pos = Pos(1, 0);
const UP_RIGHT: Pos = UP + RIGHT;
const UP_LEFT: Pos = UP + LEFT;
const DOWN_RIGHT: Pos = DOWN + RIGHT;
const DOWN_LEFT: Pos = DOWN + LEFT;

struct Rope {
    knots: Vec<Pos>,
}

impl Rope {
    fn new(n: usize) -> Rope {
        let knots = vec![Pos(0, 0); n];
        Rope { knots }
    }

    fn move_tail(&mut self, i: usize) {
        if i > 0 {
            let dxy = self.knots[i - 1] - self.knots[i];
            let next_move = match dxy {
                // simple moves
                Pos(0, -2) => Some(UP),
                Pos(0, 2) => Some(DOWN),
                Pos(2, 0) => Some(LEFT),
                Pos(-2, 0) => Some(RIGHT),
                // diagonals
                Pos(-1, -2) | Pos(-2, -1) | Pos(-2, -2) => Some(UP_RIGHT),
                Pos(-1, 2) | Pos(-2, 1) | Pos(-2, 2) => Some(DOWN_RIGHT),
                Pos(1, -2) | Pos(2, -1) | Pos(2, -2) => Some(UP_LEFT),
                Pos(1, 2) | Pos(2, 1) | Pos(2, 2) => Some(DOWN_LEFT),
                _ => None,
            };
            if let Some(a_move) = next_move {
                self.move_pos(i - 1, a_move);
            }
        }
    }

    fn move_pos(&mut self, i: usize, dir: Pos) {
        self.knots[i] = self.knots[i] + dir;
        self.move_tail(i);
    }

    fn move_head(&mut self, dir: Pos) {
        let head = self.knots.len() - 1;
        self.move_pos(head, dir)
    }
}

fn unique_tail_positions(input: &Vec<String>, n: usize) -> usize {
    let mut rope = Rope::new(n);
    let mut tail_positions = HashSet::<Pos>::new();
    for line in input.iter() {
        let parts: Vec<&str> = line.as_str().split(" ").collect();
        let steps = parts[1].parse().unwrap();
        let dir = match parts[0] {
            "R" => RIGHT,
            "L" => LEFT,
            "U" => UP,
            "D" => DOWN,
            _ => panic!(),
        };
        for _ in 0..steps {
            rope.move_head(dir);
            tail_positions.insert(rope.knots[0]);
        }
    }
    tail_positions.len()
}

fn main() {
    let input: Vec<String> = read_input();
    println!("part 1 = {:?}", unique_tail_positions(&input, 2));
    println!("part 2 = {:?}", unique_tail_positions(&input, 10));
}
