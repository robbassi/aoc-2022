use std::collections::VecDeque;
use std::io;
use std::io::BufRead;

/*
        [D]
    [N] [C]
    [Z] [M] [P]
     1   2   3

     => [['N', 'Z'], ['D', 'C', 'M'], ['P']]
*/
fn parse_crates(input: &Vec<&String>) -> Vec<VecDeque<char>> {
    let line_len = input[0].len();
    let n_vecs = (line_len + 1) / 4;
    let mut vecs: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..n_vecs {
        vecs.push(VecDeque::new());
    }
    for line in input {
        let mut i: usize = 1;
        let chars: Vec<char> = line.chars().collect();
        if chars[i] == '1' {
            break;
        }
        while i < line_len {
            match chars[i] {
                ' ' => (),
                a => vecs[i / 4].push_back(a),
            }
            i += 4;
        }
    }
    vecs
}

#[derive(Debug)]
struct Move {
    n: i32,
    src: usize,
    dest: usize,
}

fn parse_operations(input: &Vec<&String>) -> Vec<Move> {
    input.iter().fold(Vec::new(), |mut ops, line| {
        let lexemes: Vec<&str> = line.as_str().split(" ").collect();
        match lexemes.as_slice() {
            ["move", n, "from", src, "to", dest] => ops.push(Move {
                n: n.parse().unwrap(),
                src: src.parse().unwrap(),
                dest: dest.parse().unwrap(),
            }),
            _ => panic!("Expecting move # from # to #"),
        }
        ops
    })
}

fn parse_input(input: &Vec<String>) -> (Vec<VecDeque<char>>, Vec<Move>) {
    let crate_input: Vec<&String> = input
        .iter()
        .take_while(|line| line.as_str() != "")
        .collect();
    let ops_input: Vec<&String> = input
        .iter()
        .skip_while(|line| line.as_str() != "")
        .skip(1)
        .collect();
    (parse_crates(&crate_input), parse_operations(&ops_input))
}

fn top_of_crates(crates: &Vec<VecDeque<char>>) -> String {
    crates.iter().map(|c| c[0]).collect()
}

fn part1(input: &Vec<String>) -> String {
    let (mut crates, moves) = parse_input(input);
    for Move { n, src, dest } in moves {
        for _ in 0..n {
            let x = crates[src - 1].pop_front().unwrap();
            crates[dest - 1].push_front(x);
        }
    }
    top_of_crates(&crates)
}

fn part2(input: &Vec<String>) -> String {
    let (mut crates, moves) = parse_input(input);
    for Move { n, src, dest } in moves {
        let mut xs = VecDeque::new();
        for _ in 0..n {
            xs.push_front(crates[src - 1].pop_front().unwrap());
        }
        for x in xs {
            crates[dest - 1].push_front(x);
        }
    }
    top_of_crates(&crates)
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines().map(Result::unwrap).collect();
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}
