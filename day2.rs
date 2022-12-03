use std::io;
use std::io::BufRead;

#[derive(Clone,Copy,PartialEq)]
enum Selection {
    Rock,
    Paper,
    Scissors
}

enum Outcome { Win, Lose, Draw }

/* These are indexed by Selection values */
const SCORE: [i32; 3] = [1,2,3];
const BEATS: [Selection; 3] = [
    Selection::Scissors,
    Selection::Rock,
    Selection::Paper
];
const LOSES: [Selection; 3] = [
    Selection::Paper,
    Selection::Scissors,
    Selection::Rock
];

#[inline(always)]
fn parse_selection(character: char) -> Selection {
    match character {
        'A' => Selection::Rock,
        'B' => Selection::Paper,
        'C' => Selection::Scissors,
        'X' => Selection::Rock,
        'Y' => Selection::Paper,
        'Z' => Selection::Scissors,
        _ => panic!("bad input!")
    }
}

#[inline(always)]
fn parse_outcome(character: char) -> Outcome {
    match character {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("bad input!")
    }
}

fn part1(input: &Vec<String>) -> i32 {
    input.into_iter().fold(0, |acc, line| {
        let chars: Vec<char> = line.as_str().chars().collect();
        let opponent_move = parse_selection(chars[0]);
        let my_move = parse_selection(chars[2]);
        let my_index = my_move as usize;
        if BEATS[my_index] == opponent_move {
            acc + SCORE[my_index] + 6
        } else if opponent_move == my_move {
            acc + SCORE[my_index] + 3
        } else {
            acc + SCORE[my_index]
        }
    })
}

fn part2(input: &Vec<String>) -> i32 {
    input.into_iter().fold(0, |acc, line| {
        let chars: Vec<char> = line.as_str().chars().collect();
        let opponent_move = parse_selection(chars[0]);
        let opponent_index = opponent_move as usize;
        match parse_outcome(chars[2]) {
            Outcome::Win => {
                let index = LOSES[opponent_index] as usize;
                acc + SCORE[index] + 6
            },
            Outcome::Draw => {
                acc + SCORE[opponent_index] + 3
            },
            Outcome::Lose => {
                let index = BEATS[opponent_index] as usize;
                acc + SCORE[index]
            }
        }
    })
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}
