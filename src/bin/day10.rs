use aoc::io::*;

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn cycles(&self) -> usize {
        match *self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

#[derive(Debug)]
struct Program {
    register: i32,
    counter: usize,
    delay: usize,
    done: bool,
    instructions: Vec<Instruction>,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Program {
        Program {
            register: 1,
            counter: 0,
            delay: instructions[0].cycles(),
            done: false,
            instructions,
        }
    }

    fn step(&mut self) {
        if self.done {
            return;
        }

        self.delay -= 1;
        if self.delay == 0 {
            // execute operation
            match self.instructions[self.counter] {
                Instruction::Noop => (),
                Instruction::Addx(n) => self.register += n,
            }
            // update program counter
            self.counter += 1;
            // setup next operation
            if self.counter < self.instructions.len() {
                let next = &self.instructions[self.counter];
                self.delay = next.cycles();
            } else {
                self.done = true;
            }
        }
    }
}

fn parse_instructions(input: &Vec<String>) -> Vec<Instruction> {
    input.iter().fold(Vec::new(), |mut instructions, line| {
        let parts: Vec<&str> = line.as_str().split(" ").collect();
        match parts[..] {
            ["addx", n] => {
                let value = n.parse().unwrap();
                instructions.push(Instruction::Addx(value));
            }
            ["noop"] => instructions.push(Instruction::Noop),
            _ => panic!(),
        }
        instructions
    })
}

fn part1(instructions: Vec<Instruction>) -> i32 {
    let mut program = Program::new(instructions);
    let mut cycle = 1;
    let mut signal_strength = 0;
    while !program.done {
        if cycle == 20 || (cycle > 20 && (cycle - 20) % 40 == 0) {
            signal_strength += program.register * cycle;
        }
        program.step();
        cycle += 1;
    }
    signal_strength
}

fn part2(instructions: Vec<Instruction>) {
    let mut program = Program::new(instructions);
    let mut cycle = 1;
    while !program.done {
        let col = (cycle - 1) % 40;
        if program.register - 1 <= col && col <= program.register + 1 {
            print!("#");
        } else {
            print!(".");
        }
        program.step();
        if cycle % 40 == 0 {
            println!();
        }
        cycle += 1;
    }
}

fn main() {
    let input: Vec<String> = read_input();
    let instructions = parse_instructions(&input);
    println!("part 1 = {:?}", part1(instructions.clone()));
    println!("part 2 = ");
    part2(instructions.clone());
}
