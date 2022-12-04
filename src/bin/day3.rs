use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn part1(input: &Vec<String>) -> i32 {
    input.into_iter().fold(0, |priorities, line| {
        let chars = line.as_str().chars();
        let common: char = {
            let half = line.len() / 2;
            let comp1: HashSet<char> = chars.clone().take(half).collect();
            let comp2: HashSet<char> = chars.skip(half).collect();
            let common = comp1.intersection(&comp2).nth(0).unwrap();
            *common
        };
        if common.is_ascii_lowercase() {
            priorities + common as i32 - 96
        } else {
            priorities + common as i32 - 64 + 26
        }
    })
}

fn part2(input: &Vec<String>) -> i32 {
    let mut i = 0;
    let mut priorities = 0;
    while i < input.len() {
        let elf1: HashSet<char> = input[i].as_str().chars().collect();
        let elf2: HashSet<char> = input[i + 1].as_str().chars().collect();
        let elf3: HashSet<char> = input[i + 2].as_str().chars().collect();
        let common: char = *elf1
            .intersection(&elf2)
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&elf3)
            .nth(0)
            .unwrap();
        if common.is_ascii_lowercase() {
            priorities += common as i32 - 96;
        } else {
            priorities += common as i32 - 64 + 26;
        }
        i += 3;
    }
    priorities
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}
