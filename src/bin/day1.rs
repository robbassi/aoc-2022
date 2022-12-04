use std::io;
use std::io::BufRead;
use std::vec::Vec;

// O(1) space
fn part1(input: &Vec<String>) -> i32 {
    let (max, _) = input
        .into_iter()
        .fold((0, 0), |(max, current_calories), line| {
            match (line.as_str(), current_calories > max) {
                ("", true) => (current_calories, 0),
                ("", false) => (max, 0),
                (string, _) => (max, current_calories + string.parse::<i32>().unwrap()),
            }
        });
    max
}

// O(n) space
fn part2(input: &Vec<String>) -> i32 {
    let mut vec = Vec::new();
    let last = input
        .into_iter()
        .fold(0, |calories, line| match line.as_str() {
            "" => {
                vec.push(calories);
                0
            }
            string => calories + string.parse::<i32>().unwrap(),
        });
    vec.push(last);
    vec.sort_by(|a, b| b.cmp(a));
    vec[0..3].iter().sum()
}

fn insert(vec: &mut Vec<i32>, k: usize, n: i32) {
    let mut i = 0;
    while i < k {
        let k = vec[i];
        if n > k {
            break;
        }
        i += 1;
    }
    if i < k {
        let index = i;
        i += 1;
        while i < k {
            vec[i] = vec[i - 1];
            i += 1;
        }
        vec[index] = n;
    }
}

// O(k) space
fn part2v2(input: &Vec<String>, k: usize) -> i32 {
    let mut top_k = vec![0; k];
    let last = input
        .into_iter()
        .fold(0, |calories, line| match line.as_str() {
            "" => {
                insert(&mut top_k, k, calories);
                0
            }
            string => calories + string.parse::<i32>().unwrap(),
        });
    insert(&mut top_k, k, last);
    top_k.iter().sum()
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines().map(Result::unwrap).collect();
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
    println!("part 2v2 = {}", part2v2(&input, 3));
}
