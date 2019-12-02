pub fn exec_intcode(instr: &mut [usize]) {
    let mut pc: usize = 0;

    loop {
        match instr[pc] {
            1 => {
                let x = instr[instr[pc+1]];
                let y = instr[instr[pc+2]];
                instr[instr[pc+3]] = x+y;
                pc += 4;
            }
            2 => {
                let x = instr[instr[pc+1]];
                let y = instr[instr[pc+2]];
                instr[instr[pc+3]] = x*y;
                pc += 4;
            }
            99 => { break; }
            i => { panic!("invalid instruction: {}", i); }
        }
    }
}
