use aoc::io::*;
use std::cmp::min;

fn part1(input: &Vec<String>) -> i32 {
    let n = input.len();
    let mut visible = vec![false; n * n];
    let default_visible = 4 * (n as i32 - 1);
    let mut total_visible = default_visible;
    let mut count = |x, y| {
        if visible[y * n + x] == false {
            total_visible += 1;
            visible[y * n + x] = true;
        }
    };
    for i in 1..n - 1 {
        // left to right, top to bottom
        let row = input[i].as_bytes();
        let mut max_a = row[0];
        let mut max_b = input[0].as_bytes()[i];
        for j in 1..n - 1 {
            if row[j] > max_a {
                max_a = row[j];
                count(j, i);
            }
            let col = input[j].as_bytes();
            if col[i] > max_b {
                max_b = col[i];
                count(i, j);
            }
        }
        // right to left, bottom to top
        max_a = row[n - 1];
        max_b = input[n - 1].as_bytes()[i];
        for j in (1..n - 1).rev() {
            if row[j] > max_a {
                max_a = row[j];
                count(j, i);
            }
            let col = input[j].as_bytes();
            if col[i] > max_b {
                max_b = col[i];
                count(i, j);
            }
        }
    }
    return total_visible;
}

fn part2(input: &Vec<String>) -> usize {
    let n = input.len();
    let mut max_score = 0;
    for y in 0..n {
        let row = input[y].as_bytes();
        for x in 0..n {
            let tree_height = row[x];
            let mut score = 1;
            // up
            let mut y2 = y;
            if y > 0 {
                y2 = y - 1;
                while y2 > 0 && input[y2].as_bytes()[x] < tree_height {
                    y2 -= 1;
                }
            }
            score *= y - min(y2, n - 1);
            // down
            y2 = y + 1;
            while y2 < n && input[y2].as_bytes()[x] < tree_height {
                y2 += 1;
            }
            score *= min(y2, n - 1) - y;
            // left
            let mut x2 = x;
            if x > 0 {
                x2 = x - 1;
                while x2 > 0 && row[x2] < tree_height {
                    x2 -= 1;
                }
            }
            score *= x - min(x2, n - 1);
            // right
            x2 = x + 1;
            while x2 < n && row[x2] < tree_height {
                x2 += 1;
            }
            score *= min(x2, n - 1) - x;
            // update max
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

fn main() {
    let input: Vec<String> = read_input();
    println!("part 1 = {:?}", part1(&input));
    println!("part 2 = {:?}", part2(&input));
}
