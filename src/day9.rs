use std::error::Error;
use aoc2019::*;

#[test]
fn test_p1() {
    assert!({
        let instr = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut cpu = Cpu::new(instr.clone());
        cpu.run();
        &cpu.memory[..instr.len()] == &instr[..]
    });

    assert!({
        let mut cpu = Cpu::new(vec![1102,34915192,34915192,7,4,7,99,0]);
        cpu.run();
        cpu.outputs[0].to_string().len() == 16
    });

    assert!({
        let mut cpu = Cpu::new(vec![104,1125899906842624,99]);
        cpu.run();
        cpu.outputs[0] == 1125899906842624
    });
}

fn main() -> Result<(), Box<dyn Error>> {
    let instr = read_program()?;

    // Part 1
    {
        let mut cpu = Cpu::new(instr.clone());
        cpu.inputs.push(1);
        cpu.run();
        println!("{:?}", cpu.outputs);
    }

    // Part 2
    {
        let mut cpu = Cpu::new(instr.clone());
        cpu.inputs.push(2);
        cpu.run();
        println!("{:?}", cpu.outputs);
    }

    return Ok(());
}
