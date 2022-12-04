use std::io;
use std::io::BufRead;

fn pairs(string: &String) -> ((i32, i32), (i32, i32)) {
    let nums: Vec<i32> = string
        .as_str()
        .split(&['-', ','])
        .map(|p| p.parse().unwrap())
        .collect();
    match nums.chunks(4).next() {
        Some([a, b, c, d]) => ((*a, *b), (*c, *d)),
        _ => panic!(),
    }
}

fn part1(input: &Vec<String>) -> i32 {
    input.iter().fold(0, |overlaps, line| match pairs(line) {
        ((a, b), (c, d)) if a <= c && b >= d => overlaps + 1,
        ((a, b), (c, d)) if c <= a && d >= b => overlaps + 1,
        _ => overlaps,
    })
}

fn part2(input: &Vec<String>) -> i32 {
    input.iter().fold(0, |overlaps, line| match pairs(line) {
        ((a, b), (c, d)) if a <= d && b >= c => overlaps + 1,
        ((a, b), (c, d)) if d <= a && c >= b => overlaps + 1,
        _ => overlaps,
    })
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines().map(Result::unwrap).collect();
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}
