use aoc2019::*;
use permutohedron::heap_recursive;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let instr: Vec<i64> = {
        let stdin = io::stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;
        buf
            .trim()
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect()
    };

    let mut phases = [0,1,2,3,4];
    let mut all_phase_configs = Vec::new();
    heap_recursive(&mut phases, |permutation| {
        all_phase_configs.push(permutation.to_vec())
    });

    // Part 1
    {
        let mut best_output = i64::min_value();
        for config in all_phase_configs {
            let mut next_input = 0;
            for phase in config {
                let mut cpu = Cpu::new(instr.clone());
                cpu.inputs = vec![phase, next_input];
                cpu.run();
                next_input = cpu.outputs[0];
            }

            if next_input > best_output {
                best_output = next_input;
            }
        }
        println!("{}", best_output);
    }

    return Ok(());
}
