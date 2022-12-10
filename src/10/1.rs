use std::fs;
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
            self.blocked_cycles += 1;
        } else {
            self.blocked_cycles -= 1;

            if self.blocked_cycles == 0 {
                self.register_x += num;
                self.instruction_index += 1;
            }
        }
    }

    fn signal_strength(&self) -> i32 {
        self.register_x * self.cycle
    }

    fn program_ended(&self) -> bool {
        self.instruction_index == self.program.len
    }

    // fn print_state(&self) {
    //     println!("[CYCLE {}] X = {}", self.cycle, self.register_x);

    //     match self.program.get_instruction(self.instruction_index) {
    //         Some(instruction) => {
    //             dbg!(instruction);
    //         }
    //         None => (),
    //     };
    // }
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

const CYCLES_TO_CHECK: [i32; 6] = [20, 60, 100, 140, 180, 220];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let program = Program::from(&input);

    let mut cpu = CPU::new(program);
    let mut result = 0;

    while !cpu.program_ended() {
        cpu.clock();

        if CYCLES_TO_CHECK.contains(&cpu.cycle) {
            result += cpu.signal_strength();
        }
    }

    println!("{result}");
}
