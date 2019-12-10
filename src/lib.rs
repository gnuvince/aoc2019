#[derive(Debug)]
pub struct Cpu {
    pub pc: i64,                // program counter
    pub ic: i64,                // input counter
    pub rb: i64,                // relative base
    pub memory: Vec<i64>,       // memory (instructions & data)
    pub inputs: Vec<i64>,       // inputs, indexed by ic
    pub outputs: Vec<i64>,      // outputs
    pub last_op: Op,            // last executed Op
}

impl Cpu {
    pub fn new(instructions: Vec<i64>) -> Cpu {
        let mut memory = vec![0_i64; 65536];
        for (i, instr) in instructions.into_iter().enumerate() {
            memory[i] = instr;
        }

        return Cpu {
            pc: 0,
            ic: 0,
            rb: 0,
            memory: memory,
            inputs: Vec::new(),
            outputs: Vec::new(),
            last_op: Op::Boot,
        };
    }

    fn fetch(&self, addr: i64, mode: Mode) -> i64 {
        match mode {
            Mode::Pos => self.memory[self.memory[addr as usize] as usize],
            Mode::Imm => self.memory[addr as usize],
            Mode::Rel => self.memory[(self.rb + addr) as usize],
        }
    }

    fn store(&mut self, addr: i64, val: i64, mode: Mode) {
        match mode {
            Mode::Pos => {
                let x = self.memory[addr as usize] as usize;
                self.memory[x] = val;
            }
            Mode::Imm => panic!("cannot write in immediate mode"),
            Mode::Rel => {
                let x = self.memory[(self.rb + addr) as usize] as usize;
                self.memory[x] = val;
            }
        }
    }

    pub fn run(&mut self) {
        while self.last_op != Op::Stop {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let op = Op::from_i64(self.memory[self.pc as usize]);
        let modes = Mode::from_i64(self.memory[self.pc as usize]);
        match op {
            Op::Add => {
                let x = self.fetch(self.pc+1, modes[0]);
                let y = self.fetch(self.pc+2, modes[1]);
                self.store(self.pc+3, x+y, modes[2]);
                self.pc += 4;
            }
            Op::Mul => {
                let x = self.fetch(self.pc+1, modes[0]);
                let y = self.fetch(self.pc+2, modes[1]);
                self.store(self.pc+3, x*y, modes[2]);
                self.pc += 4;
            }
            Op::In => {
                self.store(self.pc+1, self.inputs[self.ic as usize], modes[0]);
                self.ic += 1;
                self.pc += 2;
            }
            Op::Out => {
                let x = self.fetch(self.pc+1, modes[0]);
                self.outputs.push(x);
                self.pc += 2;
            }
            Op::JmpIfTrue => {
                let x = self.fetch(self.pc+1, modes[0]);
                let new_pc = self.fetch(self.pc+2, modes[1]);
                if x != 0 {
                    self.pc = new_pc;
                } else {
                    self.pc += 3;
                }
            }
            Op::JmpIfFalse => {
                let x = self.fetch(self.pc+1, modes[0]);
                let new_pc = self.fetch(self.pc+2, modes[1]);
                if x == 0 {
                    self.pc = new_pc;
                } else {
                    self.pc += 3;
                }
            }
            Op::Lt => {
                let x = self.fetch(self.pc+1, modes[0]);
                let y = self.fetch(self.pc+2, modes[1]);
                self.store(self.pc+3, (x < y) as i64, modes[2]);
                self.pc += 4;
            }
            Op::Eq => {
                let x = self.fetch(self.pc+1, modes[0]);
                let y = self.fetch(self.pc+2, modes[1]);
                self.store(self.pc+3, (x == y) as i64, modes[2]);
                self.pc += 4;
            }
            Op::SetRb => {
                let incr = self.fetch(self.pc+1, modes[0]);
                self.rb += incr;
                self.pc += 2;
            }
            Op::Stop => (),
            Op::Boot => panic!("should never execute Boot op"),
        }
        self.last_op = op;
    }
}

#[derive(Debug, Clone, Copy)]
enum Mode { Pos, Imm, Rel }

impl Mode {
    fn from_i64(mut n: i64) -> Vec<Mode> {
        n /= 100; // Skip op code
        let mut modes = Vec::with_capacity(3);
        for _ in 0 .. 3 {
            let mode = match n % 10 {
                0 => Mode::Pos,
                1 => Mode::Imm,
                2 => Mode::Rel,
                x => panic!("unknown mode: {}", x),
            };
            modes.push(mode);
            n /= 10;
        }
        return modes;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Boot,
    Stop,
    Add,
    Mul,
    In,
    Out,
    JmpIfTrue,
    JmpIfFalse,
    Lt,
    Eq,
    SetRb,
}

impl Op {
    fn from_i64(n: i64) -> Op {
        match n % 100 {
            1 => Op::Add,
            2 => Op::Mul,
            3 => Op::In,
            4 => Op::Out,
            5 => Op::JmpIfTrue,
            6 => Op::JmpIfFalse,
            7 => Op::Lt,
            8 => Op::Eq,
            9 => Op::SetRb,
            99 => Op::Stop,
            x => panic!("unknown opcode: {}", x)
        }
    }
}
