use std::error::Error;
use std::io::{self, BufRead};
use aoc2019::*;

#[test]
fn test_p1() {
    assert!({
        let mut cpu = Cpu::new(vec![1,0,0,0,99]);
        cpu.run();
        cpu.memory[..5] == [2,0,0,0,99]
    });

    assert!({
        let mut cpu = Cpu::new(vec![2,3,0,3,99]);
        cpu.run();
        cpu.memory[..5] == [2,3,0,6,99]
    });

    assert!({
        let mut cpu = Cpu::new(vec![2,4,4,5,99,0]);
        cpu.run();
        cpu.memory[..6] == [2,4,4,5,99,9801]
    });

    assert!({
        let mut cpu = Cpu::new(vec![1,1,1,4,99,5,6,0,99]);
        cpu.run();
        cpu.memory[..9] == [30,1,1,4,2,5,6,0,99]
    });
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    for line in stdin.lines() {
        let line = line?;
        let instr: Vec<i64> = line
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        // part1
        {
            let mut cpu = Cpu::new(instr.clone());
            cpu.memory[1] = 12;
            cpu.memory[2] = 2;
            cpu.run();
            println!("{}", cpu.memory[0]);
        }

        // part2
        for noun in 0 .. 100 {
            for verb in 0 .. 100 {
                let mut cpu = Cpu::new(instr.clone());
                cpu.memory[1] = noun;
                cpu.memory[2] = verb;
                cpu.run();
                if cpu.memory[0] == 19690720 {
                    println!("{}", 100*noun + verb);
                    return Ok(());
                }
            }
        }
    }
    return Ok(());
}
