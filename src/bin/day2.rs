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

    fn from_char(character: char) -> Move {
        match character {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            _ => panic!("Invalid Selection, expecting: {{A,B,C,X,Y,Z}}"),
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_char(character: char) -> Outcome {
        match character {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Invalid Outcome, expecting: {{X,Y,Z}}"),
        }
    }
}

fn part1(input: &Vec<String>) -> i32 {
    input.into_iter().fold(0, |acc, line| {
        let chars: Vec<char> = line.chars().collect();
        match chars.as_slice() {
            [opponent@('A'|'B'|'C') , _, me@('X'|'Y'|'Z')] => {
                let opponent_move = Move::from_char(*opponent);
                let my_move = Move::from_char(*me);
                if my_move.wins_to() == opponent_move {
                    acc + my_move.score() + 6
                } else if opponent_move == my_move {
                    acc + my_move.score() + 3
                } else {
                    acc + my_move.score()
                }
            }
            _ => panic!("bad input, expecting: {{A,B,C}} {{X,Y,Z}}"),
        }
    })
}

fn part2(input: &Vec<String>) -> i32 {
    input.into_iter().fold(0, |acc, line| {
        let chars: Vec<char> = line.chars().collect();
        match chars.as_slice() {
            [opponent@('A'|'B'|'C') , _, desired_outcome@('X'|'Y'|'Z')] => {
                let opponent_move = Move::from_char(*opponent);
                match Outcome::from_char(*desired_outcome) {
                    Outcome::Win => acc + opponent_move.loses_to().score() + 6,
                    Outcome::Draw => acc + opponent_move.score() + 3,
                    Outcome::Lose => acc + opponent_move.wins_to().score(),
                }
            }
            _ => panic!("bad input, expecting: {{A,B,C}} {{X,Y,Z}}"),
        }
    })
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}
