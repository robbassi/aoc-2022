use aoc::io::*;
use aoc::result::*;
use std::collections::VecDeque;

/*
        [D]
    [N] [C]      =>  [['N', 'Z'], ['D', 'C', 'M'], ['P']]
    [Z] [M] [P]
     1   2   3
*/
type Crates = Vec<VecDeque<char>>;

fn parse_crates(input: &Vec<&String>) -> AocResult<Crates> {
    let line_len = input[0].len();
    let n_stacks = (line_len + 1) / 4;
    let mut crates = Vec::new();
    for _ in 0..n_stacks {
        crates.push(VecDeque::new());
    }
    for line in input {
        let mut chars = line.chars();
        match chars.nth(1) {
            Some(c) if c.is_ascii_alphabetic() => crates[0].push_back(c),
            Some(d) if d.is_ascii_digit() => break,
            _ => (), // pass,
        }
        for i in 1..n_stacks {
            match chars.nth(3) {
                Some(c) if c.is_ascii_alphabetic() => crates[i].push_back(c),
                _ => (), // pass,
            }
        }
    }
    Ok(crates)
}

#[derive(Debug)]
struct Move {
    n: i32,
    src: usize,
    dest: usize,
}

fn parse_moves(input: &Vec<&String>) -> AocResult<Vec<Move>> {
    input.iter().fold(Ok(Vec::new()), |ops, line| {
        let mut ops = ops?;
        let lexemes: Vec<&str> = line.as_str().split(" ").collect();
        match lexemes.as_slice() {
            ["move", n, "from", src, "to", dest] => {
                ops.push(Move {
                    n: n.parse().lift()?,
                    src: src.parse().lift()?,
                    dest: dest.parse().lift()?,
                });
                Ok(ops)
            }
            _ => return parse_error(line.to_string(), "move # from # to #".into()),
        }
    })
}

fn parse_input(input: &Vec<String>) -> AocResult<(Crates, Vec<Move>)> {
    let crate_input: Vec<&String> = input
        .iter()
        .take_while(|line| line.as_str() != "")
        .collect();
    let ops_input: Vec<&String> = input
        .iter()
        .skip_while(|line| line.as_str() != "")
        .skip(1)
        .collect();
    Ok((parse_crates(&crate_input)?, parse_moves(&ops_input)?))
}

fn top_of_crates(crates: &Crates) -> String {
    crates.iter().map(|c| c[0]).collect()
}

fn part1(input: &Vec<String>) -> AocResult<String> {
    let (mut crates, moves) = parse_input(input)?;
    for Move { n, src, dest } in moves {
        for _ in 0..n {
            let c = crates[src - 1].pop_front().lift()?;
            crates[dest - 1].push_front(c);
        }
    }
    Ok(top_of_crates(&crates))
}

fn part2(input: &Vec<String>) -> AocResult<String> {
    let (mut crates, moves) = parse_input(input)?;
    for Move { n, src, dest } in moves {
        let mut cs = VecDeque::new();
        for _ in 0..n {
            cs.push_front(crates[src - 1].pop_front().lift()?);
        }
        for c in cs {
            crates[dest - 1].push_front(c);
        }
    }
    Ok(top_of_crates(&crates))
}

fn main() {
    let input: Vec<String> = read_input();
    println!("part 1 = {:?}", part1(&input).unwrap());
    println!("part 2 = {:?}", part2(&input).unwrap());
}
