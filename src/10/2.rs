use std::fs;
use std::ops::RangeInclusive;
use std::option::Option;

#[derive(Debug)]
struct CPU {
    program: Program,
    instruction_index: usize,
    cycle: i32,
    blocked_cycles: i32,
    register_x: i32,
}

impl CPU {
    fn new(program: Program) -> CPU {
        CPU {
            program,
            cycle: 1,
            blocked_cycles: 0,
            register_x: 1,
            instruction_index: 0,
        }
    }

    fn clock(&mut self) {
        self.cycle += 1;

        match self.program.get_instruction(self.instruction_index) {
            Some(Instruction::Noop) => self.noop(),
            Some(Instruction::Addx(num)) => self.addx(*num),
            None => (),
        };
    }

    fn noop(&mut self) {
        self.instruction_index += 1;
    }

    fn addx(&mut self, num: i32) {
        if self.blocked_cycles == 0 {
            self.blocked_cycles = 1;
        } else {
            self.blocked_cycles = 0;
            self.register_x += num;
            self.instruction_index += 1;
        }
    }

    fn program_ended(&self) -> bool {
        self.instruction_index == self.program.len
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from(line: &str) -> Instruction {
        let instruction = line.to_owned() + " ";

        match instruction.split_once(' ') {
            Some(("addx", num)) => {
                let num: i32 = num.trim().parse().unwrap();
                Instruction::Addx(num)
            }

            Some(("noop", _)) => Instruction::Noop,

            _ => panic!("Unknown instruction: {}", instruction),
        }
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
    len: usize,
}

impl Program {
    fn from(input: &str) -> Program {
        let instructions: Vec<Instruction> = input
            .trim()
            .lines()
            .map(|line| Instruction::from(line))
            .collect();

        let len = &instructions.len();

        Program {
            instructions,
            len: *len,
        }
    }

    fn get_instruction(&self, index: usize) -> Option<&Instruction> {
        self.instructions.get(index)
    }
}

struct Screen {
    pixels: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        Screen {
            width,
            height,
            pixels: vec![vec![false; width]; height],
        }
    }

    fn draw(&mut self, cycle: i32, register: i32) {
        let width = self.width as i32;
        let height = self.height as i32;

        let x = (cycle - 1) % width;
        let y = ((cycle - 1) / width) % height;

        let sprite = Sprite::from(register);

        self.pixels[y as usize][x as usize] = sprite.contains(x);
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match &self.pixels[y][x] {
                    true => print!("# "),
                    false => print!(". "),
                }
            }

            println!("");
        }
    }
}

#[derive(Debug)]
struct Sprite {
    range: RangeInclusive<i32>,
}

impl Sprite {
    fn from(register: i32) -> Sprite {
        let start = register - 1;
        let end = register + 1;
        Sprite { range: start..=end }
    }

    fn contains(&self, x: i32) -> bool {
        self.range.contains(&x)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let program = Program::from(&input);

    let mut cpu = CPU::new(program);
    let mut screen = Screen::new(40, 6);

    while !cpu.program_ended() {
        screen.draw(cpu.cycle, cpu.register_x);
        cpu.clock();
    }

    screen.print();
}
