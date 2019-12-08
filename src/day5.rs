use std::error::Error;
use std::io;
use aoc2019::exec_intcode;

#[test]
fn test_modes() {
    assert!({
        let inputs = vec![];
        let mut outputs = vec![];
        let mut instr: Vec<i64> = vec![1002,4,3,4,33];
        exec_intcode(&mut instr, &inputs, &mut outputs);
        instr[4] == 99
    });
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf)?;
    let mut instr: Vec<i64> = buf
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    // Part 1
    {
        let mut instr = instr.clone();
        let inputs = vec![1];
        let mut outputs = vec![];
        exec_intcode(&mut instr, &inputs, &mut outputs);
        println!("{:?}", outputs);
    }

    // Part 2
    {
        let inputs = vec![5];
        let mut outputs = vec![];
        exec_intcode(&mut instr, &inputs, &mut outputs);
        println!("{:?}", outputs);
    }

    return Ok(());
}
