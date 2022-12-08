use std::collections::VecDeque;
use aoc::io::*;
use aoc::result::*;

#[derive(Debug,Clone)]
struct DirSize<'a> {
    dir: &'a str,
    size: i32,
}

// Builds a vector of directory size for the given input.
fn directoy_sizes(input: &Vec<String>) -> AocResult<Vec<DirSize>> {
    // tracks the parent directory as we traverse
    let mut dirs = VecDeque::<DirSize>::new();
    // the current directory state
    let mut dir = "/";
    let mut size = 0;
    // the final vector of sizes
    let mut res = Vec::new();
    for line in input.iter() {
        let parts: Vec<&str> = line.as_str().split(" ").collect();
        match parts[..] {
            // noop
            ["$", "cd", "/"] => (),
            // add the current directory to res and restore the parent
            // directory state, while combing the child directory size
            ["$", "cd", ".."] => {
                res.push(DirSize{dir, size});
                let parent = dirs.pop_front().lift()?;
                size += parent.size;
                dir = parent.dir;
            },
            // add the current directory state to the stack, and clear the
            // directory state
            ["$", "cd", child] => {
                dirs.push_front(DirSize{dir,size});
                size = 0;
                dir = child;
            }
            // noops
            ["$", "ls"] => (),
            ["dir", _] => (),
            // add the files size to the current directory size
            [file_size, _] => size += file_size.parse::<i32>().lift()?,
            // bad input
            _ => return AocError::parse_error(
                    line.to_string(), 
                    "a command or output".into()
                )
        }
    }
    // pop all the remaining parents off the stack up to "/"
    while let Some(parent) = dirs.pop_front() {
        res.push(DirSize{dir, size});
        size += parent.size;
        dir = parent.dir;
    }
    // add "/"
    res.push(DirSize{dir, size});
    Ok(res)
}

fn part1(input: &Vec<String>) -> AocResult<i32> {
    let mut dirs = directoy_sizes(input)?;
    dirs.retain(|d| d.size <= 100_000);
    Ok(dirs.iter().fold(0, |sum, dir| sum + dir.size))
}

fn part2(input: &Vec<String>) -> AocResult<i32> {
    const MAX_SPACE: i32 = 70_000_000;
    let mut dirs = directoy_sizes(input)?;
    dirs.sort_by(|a, b | b.size.cmp(&a.size));
    let used_space = dirs[0].size;
    let free_space = MAX_SPACE - used_space;
    for (i, dir) in dirs.iter().enumerate() {
        if free_space + dir.size < 30_000_000 {
            return Ok(dirs[i - 1].size)
        }
    }
    Err("Not found").lift()
}

fn main() {
    let input: Vec<String> = read_input();
    println!("part 1 = {:?}", part1(&input).unwrap());
    println!("part 2 = {:?}", part2(&input).unwrap());
}
