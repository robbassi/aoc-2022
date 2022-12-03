use std::io;
use std::io::BufRead;

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> i32 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn wins_to(&self) -> Move {
        match *self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn loses_to(&self) -> Move {
        match *self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

#[inline(always)]
fn parse_selection(character: char) -> Move {
    match character {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        'X' => Move::Rock,
        'Y' => Move::Paper,
        'Z' => Move::Scissors,
        _ => panic!("bad input!"),
    }
}

#[inline(always)]
fn parse_outcome(character: char) -> Outcome {
    match character {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("bad input!"),
    }
}

fn part1(input: &Vec<String>) -> i32 {
    input.into_iter().fold(0, |acc, line| {
        let chars: Vec<char> = line.as_str().chars().collect();
        let opponent_move = parse_selection(chars[0]);
        let my_move = parse_selection(chars[2]);
        if my_move.wins_to() == opponent_move {
            acc + my_move.score() + 6
        } else if opponent_move == my_move {
            acc + my_move.score() + 3
        } else {
            acc + my_move.score()
        }
    })
}

fn part2(input: &Vec<String>) -> i32 {
    input.into_iter().fold(0, |acc, line| {
        let chars: Vec<char> = line.as_str().chars().collect();
        let opponent_move = parse_selection(chars[0]);
        match parse_outcome(chars[2]) {
            Outcome::Win => acc + opponent_move.loses_to().score() + 6,
            Outcome::Draw => acc + opponent_move.score() + 3,
            Outcome::Lose => acc + opponent_move.wins_to().score(),
        }
    })
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}
