use std::io;

#[derive(Debug, Clone, Copy)]
enum Mode { Pos, Imm }

impl Mode {
    fn from_i64(mut n: i64) -> Vec<Mode> {
        n /= 100; // Skip op code
        let mut modes = Vec::with_capacity(3);
        for _ in 0 .. 3 {
            let mode = match n % 10 {
                0 => Mode::Pos,
                1 => Mode::Imm,
                x => panic!("unknown mode: {}", x),
            };
            modes.push(mode);
            n /= 10;
        }
        return modes;
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Stop,
    Add,
    Mul,
    In,
    Out,
    JmpIfTrue,
    JmpIfFalse,
    Lt,
    Eq,
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
            99 => Op::Stop,
            x => panic!("unknown opcode: {}", x)
        }
    }
}

fn fetch(pc: usize, mode: Mode, instr: &[i64]) -> i64 {
    match mode {
        Mode::Pos => instr[instr[pc] as usize],
        Mode::Imm => instr[pc],
    }
}


pub fn exec_intcode(instr: &mut [i64]) {
    let mut pc: usize = 0;

    loop {
        let op = Op::from_i64(instr[pc]);
        let modes = Mode::from_i64(instr[pc]);
        match op {
            Op::Add => {
                let x = fetch(pc+1, modes[0], instr);
                let y = fetch(pc+2, modes[1], instr);
                instr[instr[pc+3] as usize] = x+y;
                pc += 4;
            }
            Op::Mul => {
                let x = fetch(pc+1, modes[0], instr);
                let y = fetch(pc+2, modes[1], instr);
                instr[instr[pc+3] as usize] = x*y;
                pc += 4;
            }
            Op::In => {
                let out_addr = instr[pc+1] as usize;
                let stdin = io::stdin();
                let mut buf = String::new();
                stdin.read_line(&mut buf).expect("cannot read line");
                let buf = buf.trim();
                instr[out_addr] = buf.parse::<i64>().unwrap();
                pc += 2;
            }
            Op::Out => {
                let x = fetch(pc+1, modes[0], instr);
                println!("{}", x);
                pc += 2;
            }
            Op::JmpIfTrue => {
                let x = fetch(pc+1, modes[0], instr);
                let new_pc = fetch(pc+2, modes[1], instr);
                if x != 0 {
                    pc = new_pc as usize;
                } else {
                    pc += 3;
                }
            }
            Op::JmpIfFalse => {
                let x = fetch(pc+1, modes[0], instr);
                let new_pc = fetch(pc+2, modes[1], instr);
                if x == 0 {
                    pc = new_pc as usize;
                } else {
                    pc += 3;
                }
            }
            Op::Lt => {
                let x = fetch(pc+1, modes[0], instr);
                let y = fetch(pc+2, modes[1], instr);
                instr[instr[pc+3] as usize] = (x < y) as i64;
                pc += 4;
            }
            Op::Eq => {
                let x = fetch(pc+1, modes[0], instr);
                let y = fetch(pc+2, modes[1], instr);
                instr[instr[pc+3] as usize] = (x == y) as i64;
                pc += 4;
            }
            Op::Stop => break,
        }
    }
}
