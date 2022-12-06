use std::collections::HashSet;
use std::io;
use std::io::BufRead;

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

fn main() {
    let input: Vec<String> = io::stdin().lock().lines().map(Result::unwrap).collect();
    println!("part 1 = {}", first_unique_window(&input[0], 4).unwrap());
    println!("part 2 = {}", first_unique_window(&input[0], 14).unwrap());
}
