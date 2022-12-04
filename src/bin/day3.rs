use std::collections::HashSet;
use std::io;
use std::io::BufRead;

pub trait Priority {
    fn priority(&self) -> i32;
}

impl Priority for char {
    fn priority(&self) -> i32 {
        if self.is_ascii_lowercase() {
            *self as i32 - 96
        } else {
            *self as i32 - 64 + 26
        }
    }
}

fn part1(input: &Vec<String>) -> i32 {
    input.into_iter().fold(0, |priorities, line| {
        let chars = line.chars();
        let common_item = {
            let half = line.len() / 2;
            let comp1: HashSet<_> = chars.clone().take(half).collect();
            let comp2: HashSet<_> = chars.skip(half).collect();
            *comp1.intersection(&comp2).nth(0).unwrap()
        };
        priorities + common_item.priority()
    })
}

fn part2(input: &Vec<String>) -> i32 {
    let mut priorities = 0;
    let mut chunks = input.chunks(3);
    while let Some([elf1, elf2, elf3]) = chunks.next() {
        let elf1_set: HashSet<_> = elf1.chars().collect();
        let elf2_set: HashSet<_> = elf2.chars().collect();
        let common_items: HashSet<_> = elf1_set.intersection(&elf2_set).copied().collect();
        // we don't have to build a set for elf3
        for item in elf3.chars() {
            if common_items.contains(&item) {
                priorities += item.priority();
                break;
            }
        }
    }
    priorities
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines().map(Result::unwrap).collect();
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}
