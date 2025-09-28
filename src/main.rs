use anyhow::{Context, Result};
use std::io::{Read, Write};
use std::{fs, io};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Interpreter for brainfuck")]
struct Args {
    file: String,
}

#[derive(Debug)]
struct Interpreter {
    p: usize,
    tape: [u8; 100000],
    input: io::Stdin,
    output: io::Stdout,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            p: 0,
            tape: [0; 100000],
            input: io::stdin(),
            output: io::stdout(),
        }
    }

    fn shift_right(&mut self) {
        self.p += 1;
    }

    fn shift_left(&mut self) {
        self.p -= 1;
    }

    fn increase(&mut self) {
        self.tape[self.p] = self.tape[self.p].wrapping_add(1);
    }

    fn decrease(&mut self) {
        self.tape[self.p] = self.tape[self.p].wrapping_sub(1);
    }

    fn print(&mut self) -> Result<()> {
        self.output.write_all(&[self.tape[self.p]])?;
        self.output.flush()?;
        Ok(())
    }

    fn read(&mut self) -> Result<()> {
        let mut buf = [0u8];
        self.input.read(&mut buf)?;
        self.tape[self.p] = buf[0];
        Ok(())
    }

    fn has_value(&self) -> bool {
        self.tape[self.p] != 0
    }
}

fn interpret(content: String) -> Result<()> {
    let mut interp = Interpreter::new();
    let mut bstack = Vec::<usize>::new();

    let n = content.len();
    let bytes = content.as_bytes();
    let mut p = 0;

    let find_nxt = |mut p| -> Option<usize> {
        while p < n {
            match bytes[p] {
                b']' => {
                    return Some(p);
                }
                _ => {}
            }
            p += 1;
        }
        None
    };

    while p < n {
        match bytes[p] {
            b'>' => interp.shift_right(),
            b'<' => interp.shift_left(),
            b'+' => interp.increase(),
            b'-' => interp.decrease(),
            b'.' => interp.print()?,
            b',' => interp.read()?,
            b'[' => {
                let nxt = find_nxt(p).unwrap();
                if interp.has_value() {
                    bstack.push(p);
                } else {
                    p = nxt;
                }
            }
            b']' => {
                if interp.has_value() {
                    p = *bstack.last().unwrap();
                } else {
                    bstack.pop();
                }
            }
            _ => {}
        }
        p += 1;
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let content = fs::read_to_string(&args.file)
        .with_context(|| format!("Failed to read file: {}", args.file))?;
    interpret(content)?;

    Ok(())
}
