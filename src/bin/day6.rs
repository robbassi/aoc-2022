use aoc;
use aoc::result::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn first_unique_window(input: &String, n: usize) -> Option<usize> {
    let windows = input.as_bytes().windows(n);
    for (i, window) in windows.enumerate() {
        let unique: HashSet<&u8> = window.iter().collect();
        if unique.len() == n {
            return Some(i + n);
        }
    }
    None
}

fn first_unique_window_linear(input: &String, n: usize) -> AocResult<usize> {
    let mut queue: VecDeque<u8> = input.bytes().take(n).collect();
    let mut freq = HashMap::<u8, usize>::new();
    for c in input.bytes().take(n) {
        freq.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }
    for (i, c) in input.bytes().skip(n).enumerate() {
        if freq.len() == n {
            return Ok(i + n);
        }
        let last = queue.pop_front().lift()?;
        // decrement
        freq.entry(last).and_modify(|c| *c -= 1);
        // remove if last
        if freq[&last] == 0 {
            freq.remove(&last);
        }
        // insert or increment the next value
        freq.entry(c).and_modify(|c| *c += 1).or_insert(1);
        queue.push_back(c);
    }
    Err("Not found").lift()
}

fn main() {
    let input: Vec<String> = aoc::io::read_input();
    println!("part 1 = {:?}", first_unique_window(&input[0], 4).unwrap());
    println!(
        "part 2 = {}",
        first_unique_window_linear(&input[0], 14).unwrap()
    );
}
