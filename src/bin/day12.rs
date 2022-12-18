use aoc::io::*;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    cells: Vec<u32>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, pos: &Pos) -> Option<u32> {
        let i = pos.y * self.width + pos.x;
        if i < self.cells.len() {
            Some(self.cells[i])
        } else {
            None
        }
    }

    fn neighbours(&self, &Pos { x, y }: &Pos) -> Vec<Pos> {
        let mut res = Vec::with_capacity(4);
        if x < self.width {
            // up
            if y > 0 {
                res.push(Pos { x, y: y - 1 });
            }
            // down
            if y < self.height - 1 {
                res.push(Pos { x, y: y + 1 });
            }
        }
        if y < self.height {
            // left
            if x > 0 {
                res.push(Pos { x: x - 1, y });
            }
            // right
            if x < self.width - 1 {
                res.push(Pos { x: x + 1, y });
            }
        }
        res
    }
}

fn parse_heightmap(input: &Vec<String>) -> (Pos, Pos, Map) {
    let ascii_offset = 'a' as u32;
    let width = input[0].len();
    let height = input.len();
    let mut cells = Vec::with_capacity(width * height);
    let mut start = Pos { x: 0, y: 0 };
    let mut end = Pos { x: 0, y: 0 };
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start.x = x;
                    start.y = y;
                    cells.push('a' as u32 - ascii_offset);
                }
                'E' => {
                    end.x = x;
                    end.y = y;
                    cells.push('z' as u32 - ascii_offset);
                }
                elevation => {
                    cells.push(elevation as u32 - ascii_offset);
                }
            }
        }
    }
    (
        start,
        end,
        Map {
            cells,
            width,
            height,
        },
    )
}

fn shortest_path(mut frontier: HashSet<Pos>, end: &Pos, heightmap: &Map) -> u32 {
    let mut distance = 0;
    let mut visited: HashSet<Pos> = HashSet::new();
    while frontier.len() > 0 {
        let mut new_frontier = HashSet::new();
        for pos in frontier {
            let elevation = heightmap.get(&pos).expect("missing elevation");
            for neighbour in heightmap.neighbours(&pos) {
                // if we haven't visited this neighbour
                if !visited.contains(&neighbour) {
                    let neighbour_elevation = heightmap.get(&neighbour).expect("missing elevation");
                    // if we're above the next position, or one step below it
                    if elevation >= neighbour_elevation || elevation + 1 == neighbour_elevation {
                        new_frontier.insert(neighbour);
                    }
                }
            }
            if pos == *end {
                return distance;
            }
            visited.insert(pos);
        }
        frontier = new_frontier;
        distance += 1;
    }
    panic!("path not found");
}

fn main() {
    let input: Vec<String> = read_input();
    let (start, end, heightmap) = parse_heightmap(&input);
    let part1 = shortest_path([start.clone()].into(), &end, &heightmap);
    println!("part 1 = {:?}", part1);

    let mut all_a_positions = HashSet::<Pos>::new();
    for (i, c) in heightmap.cells.iter().enumerate() {
        if *c == 0 {
            let pos = Pos {
                x: i % heightmap.width,
                y: i / heightmap.width,
            };
            all_a_positions.insert(pos);
        }
    }
    let part2 = shortest_path(all_a_positions, &end, &heightmap);
    println!("part 2 = {:?}", part2);
}
