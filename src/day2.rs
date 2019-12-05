use std::error::Error;
use std::io::{self, BufRead};
use aoc2019::exec_intcode;

#[test]
fn test_p1() {
    assert!({
        let mut instr = vec![1,0,0,0,99];
        exec_intcode(&mut instr);
        instr == vec![2,0,0,0,99]
    });

    assert!({
        let mut instr = vec![2,3,0,3,99];
        exec_intcode(&mut instr);
        instr == vec![2,3,0,6,99]
    });

    assert!({
        let mut instr = vec![2,4,4,5,99,0];
        exec_intcode(&mut instr);
        instr == vec![2,4,4,5,99,9801]
    });

    assert!({
        let mut instr = vec![1,1,1,4,99,5,6,0,99];
        exec_intcode(&mut instr);
        instr == vec![30,1,1,4,2,5,6,0,99]
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
            let mut p1_instr = instr.clone();
            p1_instr[1] = 12;
            p1_instr[2] = 2;
            exec_intcode(&mut p1_instr);
            println!("{}", p1_instr[0]);
        }

        // part2
        for noun in 0 .. 100 {
            for verb in 0 .. 100 {
                let mut p2_instr = instr.clone();
                p2_instr[1] = noun;
                p2_instr[2] = verb;
                exec_intcode(&mut p2_instr);
                if p2_instr[0] == 19690720 {
                    println!("{}", 100*noun + verb);
                    return Ok(());
                }
            }
        }
    }
    return Ok(());
}
