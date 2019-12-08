use std::error::Error;
use std::io::{self, Read};

const COLS: usize = 25;
const ROWS: usize = 6;

fn main() -> Result<(), Box<dyn Error>> {
    let layers: Vec<Vec<u8>> = {
        let mut stdin = io::stdin();
        let mut buf = Vec::new();
        stdin.read_to_end(&mut buf)?;

        let mut out = Vec::new();
        for ch in buf.chunks(COLS*ROWS) {
            if ch.len() != COLS*ROWS {
                break;
            }
            let v: Vec<u8> = ch.iter().map(|x| *x - b'0').collect();
            out.push(v);
        }
        out
    };

    // Part 1
    {
        let mut least_zeros = usize::max_value();
        let mut product = 0;

        for layer in layers.iter() {
            let mut freqs = vec![0, 0, 0];
            for x in layer {
                freqs[*x as usize] += 1;
            }
            if freqs[0] < least_zeros {
                least_zeros = freqs[0];
                product = freqs[1] * freqs[2];
            }
        }
        println!("{}", product);
    }

    // Part 2
    {
        let mut output = vec![255_u8; COLS*ROWS];
        for layer in layers {
            for (i, x) in layer.iter().enumerate() {
                if output[i] != 255 {
                    continue;
                }
                if *x == 0 { output[i] = b' '; }
                if *x == 1 { output[i] = b'#'; }
            }
        }

        for (i, b) in output.iter().enumerate() {
            if i % COLS == 0 {
                println!("");
            }
            print!("{}", *b as char);
        }
    }

    return Ok(());
}
