use aoc2019::*;
use permutohedron::heap_recursive;
use std::error::Error;

fn p1(instr: &Vec<i64>) {
    let mut phases = [0,1,2,3,4];
    let mut all_phase_configs = Vec::new();
    heap_recursive(&mut phases, |permutation| {
        all_phase_configs.push(permutation.to_vec())
    });

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

fn run_cpu_until_output(cpu: &mut Cpu) -> Option<i64> {
    loop {
        cpu.step();
        if cpu.last_op == Op::Out {
            return Some(cpu.outputs[cpu.outputs.len() - 1]);
        }
        if cpu.last_op == Op::Stop {
            return None;
        }
    }
}

fn p2(instr: &Vec<i64>) {
    let mut phases = [5,6,7,8,9];
    let mut all_phase_configs = Vec::new();
    heap_recursive(&mut phases, |permutation| {
        all_phase_configs.push(permutation.to_vec())
    });

    let mut best_output = i64::min_value();
    for config in all_phase_configs {
        let phases: Vec<i64> = config.iter().map(|x| *x).collect();
        let mut cpus = vec![
            Cpu::new(instr.clone()),
            Cpu::new(instr.clone()),
            Cpu::new(instr.clone()),
            Cpu::new(instr.clone()),
            Cpu::new(instr.clone()),
        ];
        cpus[0].inputs = vec![phases[0], 0];
        cpus[1].inputs = vec![phases[1]];
        cpus[2].inputs = vec![phases[2]];
        cpus[3].inputs = vec![phases[3]];
        cpus[4].inputs = vec![phases[4]];

        let mut curr_cpu = 0;
        loop {
            let next_cpu = (curr_cpu + 1) % 5;
            match run_cpu_until_output(&mut cpus[curr_cpu]) {
                Some(out) => {
                    cpus[next_cpu].inputs.push(out);
                }
                None => {
                    if curr_cpu == 4 {
                        break;
                    }
                }
            }
            curr_cpu = next_cpu;
        }

        let last_output = cpus[4].outputs[cpus[4].outputs.len() - 1];
        if last_output > best_output {
            best_output = last_output;
        }
    }
    println!("{}", best_output);
}

fn main() -> Result<(), Box<dyn Error>> {
    let instr = read_program()?;
    p1(&instr);
    p2(&instr);
    return Ok(());
}
