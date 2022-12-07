use std::collections::HashMap;
use std::collections::VecDeque;
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

fn first_unique_window_linear(input: &String, n: usize) -> Option<usize> {
    let mut windows = input.as_bytes().windows(n);
    let first = windows.next().unwrap();
    let mut queue: VecDeque<u8> = first.iter().copied().collect();
    let mut unique: HashMap<u8, i32> = HashMap::new();
    // build frequencies
    for x in queue.iter() {
        *unique.entry(*x).or_insert(0) += 1;
    }
    if unique.len() == n {
        return Some(n);
    }
    let mut indexed_windows = windows.enumerate();
    while let Some((i, window)) = indexed_windows.next() {
        let last = queue.pop_front().unwrap();
        // decrement
        unique.entry(last).and_modify(|x| *x -= 1);
        // remove if last
        if unique[&last] == 0 {
            unique.remove(&last);
        }
        // insert or increment the next value
        *unique.entry(window[n - 1]).or_insert(0) += 1;
        queue.push_back(window[n - 1]);
        if unique.len() == n {
            return Some(i + n + 1);
        }
    }
    None
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines().map(Result::unwrap).collect();
    println!("part 1 = {}", first_unique_window(&input[0], 4).unwrap());
    println!(
        "part 2 = {}",
        first_unique_window_linear(&input[0], 14).unwrap()
    );
}
