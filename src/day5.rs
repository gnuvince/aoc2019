use std::error::Error;
use std::io;
use aoc2019::*;

#[test]
fn test_modes() {
    let mut cpu = Cpu::new(vec![1002,4,3,4,33]);
    cpu.run();
    assert_eq!(cpu.memory[4], 99);
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf)?;
    let instr: Vec<i64> = buf
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    // Part 1
    {
        let mut cpu = Cpu::new(instr.clone());
        cpu.inputs.push(1);
        cpu.run();
        println!("{:?}", cpu.outputs);
    }

    // Part 2
    {
        let mut cpu = Cpu::new(instr);
        cpu.inputs.push(5);
        cpu.run();
        println!("{:?}", cpu.outputs);
    }

    return Ok(());
}
